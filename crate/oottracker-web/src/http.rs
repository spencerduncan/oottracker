use {
    crate::{edit_room, get_room, restream::render_double_cell, Error, MwRooms, Restreams, Rooms},
    itertools::Itertools as _,
    ootr_utils::{PyModules, Version},
    oottracker::{
        flag_mapping::{get_checked_locations_summary, CheckedLocationsSummary},
        ui::{DoubleTrackerLayout, TrackerCellId, TrackerLayout},
        websocket::MwItem,
        ModelState,
    },
    pyo3::{prelude::*, types::PyDict},
    rocket::{
        form::Form,
        fs::{relative, FileServer},
        http::uri::Origin,
        response::{content::RawHtml, status::NotFound, Redirect},
        serde::json::Json,
        uri, FromForm, FromFormField, Rocket, State, UriDisplayQuery,
    },
    rocket_util::{html, Doctype, ToHtml},
    sqlx::SqlitePool,
    std::{num::NonZeroU8, time::Duration},
};

trait TrackerCellIdExt {
    fn view<'a>(
        &self,
        click_uri: Origin<'_>,
        cell_id: u8,
        state: &ModelState,
        colspan: u8,
        loc: bool,
    ) -> RawHtml<String>;
}

impl TrackerCellIdExt for TrackerCellId {
    fn view<'a>(
        &self,
        click_uri: Origin<'_>,
        cell_id: u8,
        state: &ModelState,
        colspan: u8,
        loc: bool,
    ) -> RawHtml<String> {
        let kind = self.kind();
        let content = kind.render(state);
        let css_classes = if loc {
            format!("cols{colspan} loc")
        } else {
            format!("cols{colspan}")
        };
        html! {
            form(id = format!("cell{cell_id}"), method = "POST", action = click_uri.to_string(), class = css_classes) {
                button(type = "submit") : content;
            }
        }
    }
}

#[derive(FromFormField, UriDisplayQuery)]
enum Theme {
    Light,
    Dark,
}

fn tracker_page<'a>(
    layout_name: &'a str,
    theme: Option<Theme>,
    items: impl ToHtml,
) -> RawHtml<String> {
    html! {
        : Doctype;
        html {
            head {
                meta(charset = "utf-8");
                title : "OoT Tracker";
                meta(name = "author", content = "Fenhl");
                meta(name = "viewport", content = "width=device-width, initial-scale=1");
                link(rel = "icon", sizes = "512x512", type = "image/png", href = "/static/img/favicon.png");
                link(rel = "stylesheet", href = "/static/common.css");
                link(rel = "stylesheet", href = "/static/checked-locations.css");
                link(rel = "stylesheet", href = "/static/location-filter.css");
                @match theme {
                    Some(Theme::Light) => link(rel = "stylesheet", href = "/static/light.css");
                    None => link(rel = "stylesheet", href = "/static/light.css", media = "(prefers-color-scheme: light)");
                    Some(Theme::Dark) => {}
                }
            }
            body {
                div(id = "location-filter-placeholder");
                div(class = format!("items {layout_name}")) : items;
                noscript {
                    p : "live update disabled (requires JavaScript)";
                }
                footer {
                    a(href = "https://fenhl.net/disc") : "disclaimer / Impressum";
                }
                script(src = "/static/proto.js");
                script(src = "/static/checked-locations.js");
                script(src = "/static/location-filter.js");
            }
        }
    }
}

#[rocket::get("/")]
fn index() -> RawHtml<String> {
    RawHtml(format!(
        include_str!("../../../assets/web/index.html"),
        env!("CARGO_PKG_VERSION")
    ))
}

#[rocket::get("/settings")]
fn settings() -> RawHtml<String> {
    RawHtml(include_str!("../../../assets/web/settings.html").to_owned())
}

#[derive(FromForm)]
struct GoRoomForm<'r> {
    #[field(validate = len(1..))]
    room: &'r str,
}

#[rocket::post("/", data = "<form>")]
fn post_index(form: Form<GoRoomForm<'_>>) -> Redirect {
    Redirect::to(rocket::uri!(room(form.room.to_owned(), _)))
}

#[rocket::get("/mw/<room>/<world>?<theme>&<delay>")]
async fn mw_room_input(
    room: &str,
    world: NonZeroU8,
    theme: Option<Theme>,
    delay: Option<f64>,
) -> Redirect {
    Redirect::permanent(uri!(mw_room_view(
        room,
        world,
        TrackerLayout::default(),
        theme,
        delay
    )))
}

#[rocket::get("/mw/<room>/<world>/<layout>?<theme>&<delay>")]
async fn mw_room_view(
    mw_rooms: &State<MwRooms>,
    room: &str,
    world: NonZeroU8,
    layout: TrackerLayout,
    theme: Option<Theme>,
    delay: Option<f64>,
) -> Option<RawHtml<String>> {
    let mw_rooms = mw_rooms.read().await;
    let mw_room = mw_rooms.get(room)?;
    if let Some(delay) = delay {
        mw_room.write().await.autotracker_delay = Duration::try_from_secs_f64(delay).ok()?;
    }
    let mw_room = mw_room.read().await;
    let (_, _, model, _, _) = mw_room.world(world)?;
    Some(tracker_page(
        &layout.to_string(),
        theme,
        html! {
            @for cell in layout.cells() {
                @let cell_id = cell.idx.try_into().expect("too many cells");
                : cell.id.view(rocket::uri!(mw_click(room, world, layout, cell_id)), cell_id, model, (cell.size[0] / 20 + 1) as u8, cell.size[1] < 30);
            }
        },
    ))
}

#[rocket::post("/mw/<room>/<world>/<layout>/click/<cell_id>")]
async fn mw_click(
    mw_rooms: &State<MwRooms>,
    room: &str,
    world: NonZeroU8,
    layout: TrackerLayout,
    cell_id: u8,
) -> Result<Redirect, NotFound<&'static str>> {
    {
        let mw_rooms = mw_rooms.read().await;
        let mw_room = mw_rooms
            .get(room)
            .ok_or(NotFound("No such multiworld room"))?;
        let mut mw_room = mw_room.write().await;
        let (tx, _, model, _, _) = mw_room.world_mut(world).ok_or(NotFound("No such world"))?;
        layout
            .cells()
            .get(usize::from(cell_id))
            .ok_or(NotFound("No such cell"))?
            .id
            .kind()
            .click(model);
        tx.send(())
            .expect("failed to notify websockets about state change");
    }
    Ok(Redirect::to(rocket::uri!(mw_room_view(
        room,
        world,
        layout,
        _,
        _
    ))))
}

fn world_class(world_id: NonZeroU8) -> Option<&'static str> {
    match world_id.get() {
        0 => unreachable!(),
        1 => Some("power"),
        2 => Some("wisdom"),
        3 => Some("courage"),
        _ => None,
    }
}

fn format_override_key(modules: PyModules<'_>, key: u32, item_name: &str) -> PyResult<String> {
    let location_list = modules.py().import("LocationList")?;
    for location_name in location_list.getattr("location_table")?.iter()? {
        let location_name = location_name?.extract::<String>()?;
        if modules.override_key(&location_name, item_name)? == Some(key) {
            return Ok(location_name);
        }
    }
    Ok(format!("0x{key:08x}"))
}

fn format_item_kind(modules: PyModules<'_>, kind: u16) -> PyResult<String> {
    let item_list = modules.py().import("ItemList")?;
    for (item_name, entry) in item_list
        .getattr("item_table")?
        .downcast::<PyDict>()?
        .iter()
    {
        let (_, _, get_item_id, _) = entry.extract::<(&PyAny, &PyAny, Option<u16>, &PyAny)>()?;
        if get_item_id == Some(kind) {
            return item_name.extract();
        }
    }
    Ok(format!("0x{kind:04x}"))
}

#[derive(Debug, thiserror::Error, rocket_util::Error)]
enum NotesError {
    #[error(transparent)]
    Python(#[from] PyErr),
}

#[rocket::get("/mw-notes/<room>")]
async fn mw_notes(
    mw_rooms: &State<MwRooms>,
    room: &str,
) -> Result<Option<RawHtml<String>>, NotesError> {
    let mw_rooms = mw_rooms.read().await;
    let Some(mw_room) = mw_rooms.get(room) else {
        return Ok(None);
    };
    let mw_room = mw_room.read().await;
    let rando_version = Version::from_dev(6, 2, 205); //TODO don't hardcode
    Python::with_gil(|py| {
        let modules = rando_version.py_modules(py)?;
        Ok(Some(html! {
            : Doctype;
            html {
                head {
                    meta(charset = "utf-8");
                    title : "OoT Tracker";
                    meta(name = "author", content = "Fenhl");
                    meta(name = "viewport", content = "width=device-width, initial-scale=1");
                    link(rel = "icon", sizes = "512x512", type = "image/png", href = "/static/img/favicon.png");
                    link(rel = "stylesheet", href = "/static/common.css");
                    link(rel = "stylesheet", href = "/static/light.css", media = "(prefers-color-scheme: light)");
                }
                body {
                    div(class = "table-wrapper") {
                        @for (idx, (_, _, _, queue, own_items)) in mw_room.worlds.iter().enumerate() {
                            @let world_id = NonZeroU8::new((idx + 1).try_into().unwrap()).unwrap();
                            div {
                                h1(class? = world_class(world_id)) {
                                    : "For player ";
                                    : world_id.get();
                                };
                                table {
                                    thead {
                                        tr {
                                            th : "From world";
                                            th : "From location";
                                            th : "Item";
                                        }
                                    }
                                    tbody {
                                        @for MwItem { source, key, kind } in own_items.iter().sorted().chain(queue) {
                                            tr {
                                                @let item_name = format_item_kind(modules.clone(), *kind)?;
                                                td(class? = world_class(*source)) : source.get();
                                                td(class? = world_class(*source)) : format_override_key(modules.clone(), *key, &item_name)?;
                                                td(class? = world_class(world_id)) : item_name;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    p : "live update not yet implemented (refresh to update)"; //TODO
                    footer {
                        a(href = "https://fenhl.net/disc") : "disclaimer / Impressum";
                    }
                }
            }
        }))
    })
}

#[rocket::get("/restream/<restreamer>/<runner>?<theme>")]
async fn restream_room_input(restreamer: &str, runner: &str, theme: Option<Theme>) -> Redirect {
    Redirect::permanent(uri!(restream_room_view(
        restreamer,
        runner,
        TrackerLayout::default(),
        theme
    )))
}

#[rocket::get("/restream/<restreamer>/<runner>/<layout>?<theme>")]
async fn restream_room_view(
    restreams: &State<Restreams>,
    restreamer: &str,
    runner: &str,
    layout: TrackerLayout,
    theme: Option<Theme>,
) -> Option<RawHtml<String>> {
    let restreams = restreams.read().await;
    let restream = restreams.get(restreamer)?;
    let (_, _, model_state_view) = restream.runner(runner)?;
    Some(tracker_page(
        &layout.to_string(),
        theme,
        html! {
            @for cell in layout.cells() {
                @let cell_id = cell.idx.try_into().expect("too many cells");
                : cell.id.view(rocket::uri!(restream_click(restreamer, runner, layout, cell_id)), cell_id, &model_state_view, (cell.size[0] / 20 + 1) as u8, cell.size[1] < 30);
            }
        },
    ))
}

#[rocket::post("/restream/<restreamer>/<runner>/<layout>/click/<cell_id>")]
async fn restream_click(
    restreams: &State<Restreams>,
    restreamer: &str,
    runner: &str,
    layout: TrackerLayout,
    cell_id: u8,
) -> Result<Redirect, NotFound<&'static str>> {
    {
        let mut restreams = restreams.write().await;
        let restream = restreams
            .get_mut(restreamer)
            .ok_or(NotFound("No such restream"))?;
        let (tx, _, model_state_view) = restream
            .runner_mut(runner)
            .ok_or(NotFound("No such runner"))?;
        layout
            .cells()
            .get(usize::from(cell_id))
            .ok_or(NotFound("No such cell"))?
            .id
            .kind()
            .click(model_state_view);
        tx.send(())
            .expect("failed to notify websockets about state change");
    }
    Ok(Redirect::to(rocket::uri!(restream_room_view(
        restreamer,
        runner,
        layout,
        _
    ))))
}

#[rocket::get("/restream/<restreamer>/<runner1>/<layout>/with/<runner2>?<theme>")]
async fn restream_double_room_layout(
    restreams: &State<Restreams>,
    restreamer: &str,
    runner1: &str,
    layout: DoubleTrackerLayout,
    runner2: &str,
    theme: Option<Theme>,
) -> Option<RawHtml<String>> {
    let restreams = restreams.read().await;
    let restream = restreams.get(restreamer)?;
    let cells = layout
        .cells()
        .into_iter()
        .map(|reward| {
            Some(render_double_cell(
                restream.runner(runner1)?.2,
                restream.runner(runner2)?.2,
                reward,
            ))
        })
        .collect::<Option<Vec<_>>>()?;
    Some(tracker_page(
        &layout.to_string(),
        theme,
        html! {
            @for (cell_id, render) in cells.into_iter().enumerate() {
                div(id = format!("cell{cell_id}"), class = "cols3") : render;
            }
        },
    ))
}

#[rocket::get("/room/<name>?<theme>")]
async fn room(
    rooms: &State<Rooms>,
    name: &str,
    theme: Option<Theme>,
) -> Result<RawHtml<String>, Error> {
    Ok(get_room(rooms, name.to_owned(), |room| {
        let layout = TrackerLayout::default();
        tracker_page(&layout.to_string(), theme, html! {
            @for cell in layout.cells() {
                @let cell_id = cell.idx.try_into().expect("too many cells");
                : cell.id.view(rocket::uri!(click(name, cell_id)), cell_id, &room.model, (cell.size[0] / 20 + 1) as u8, cell.size[1] < 30);
            }
        })
    }).await?)
}

#[rocket::get("/room/<name>/<layout>?<theme>")]
async fn room_with_layout(
    rooms: &State<Rooms>,
    name: &str,
    layout: TrackerLayout,
    theme: Option<Theme>,
) -> Result<RawHtml<String>, Error> {
    Ok(get_room(rooms, name.to_owned(), |room| {
        tracker_page(&layout.to_string(), theme, html! {
            @for cell in layout.cells() {
                @let cell_id = cell.idx.try_into().expect("too many cells");
                : cell.id.view(rocket::uri!(click_with_layout(name, &layout, cell_id)), cell_id, &room.model, (cell.size[0] / 20 + 1) as u8, cell.size[1] < 30);
            }
        })
    }).await?)
}

#[rocket::post("/room/<name>/click/<cell_id>")]
async fn click(
    pool: &State<SqlitePool>,
    rooms: &State<Rooms>,
    name: &str,
    cell_id: u8,
) -> Result<Redirect, Error> {
    edit_room(pool, rooms, name.to_owned(), |room| {
        let layout = TrackerLayout::default();
        layout
            .cells()
            .get(usize::from(cell_id))
            .ok_or(Error::CellId)?
            .id
            .kind()
            .click(&mut room.model);
        Ok(())
    })
    .await?;
    Ok(Redirect::to(rocket::uri!(room(name, _))))
}

#[rocket::post("/room/<name>/<layout>/click/<cell_id>")]
async fn click_with_layout(
    pool: &State<SqlitePool>,
    rooms: &State<Rooms>,
    name: &str,
    layout: TrackerLayout,
    cell_id: u8,
) -> Result<Redirect, Error> {
    edit_room(pool, rooms, name.to_owned(), |room| {
        layout
            .cells()
            .get(usize::from(cell_id))
            .ok_or(Error::CellId)?
            .id
            .kind()
            .click(&mut room.model);
        Ok(())
    })
    .await?;
    Ok(Redirect::to(rocket::uri!(room_with_layout(
        name,
        layout,
        Option::<Theme>::None
    ))))
}

// ============================================================================
// API Endpoints for Checked Locations
// ============================================================================

/// Returns the checked locations for a room as JSON.
///
/// This endpoint provides the status of all mapped locations based on the
/// current game state (memory flags). It can be used by the web UI to display
/// which locations have been checked off.
///
/// # Response
///
/// Returns a JSON object with:
/// - `total_mapped`: Total number of mapped locations
/// - `checked_count`: Number of locations that have been checked
/// - `unchecked_count`: Number of locations not yet checked
/// - `unknown_count`: Number of locations with unknown status
/// - `locations`: Array of individual location check results
#[rocket::get("/api/room/<name>/checked-locations")]
async fn api_checked_locations(
    rooms: &State<Rooms>,
    name: &str,
) -> Result<Json<CheckedLocationsSummary>, Error> {
    let summary = get_room(rooms, name.to_owned(), |room| {
        get_checked_locations_summary(&room.model)
    })
    .await?;
    Ok(Json(summary))
}

pub(crate) fn rocket(
    pool: SqlitePool,
    rooms: Rooms,
    restreams: Restreams,
    mw_rooms: MwRooms,
) -> Rocket<rocket::Build> {
    rocket::custom(rocket::Config {
        port: 24807,
        ..rocket::Config::default()
    })
    .manage(pool)
    .manage(rooms)
    .manage(restreams)
    .manage(mw_rooms)
    .mount(
        "/static",
        FileServer::new(
            relative!("../../assets/web/static"),
            rocket::fs::Options::None,
        ),
    )
    .mount(
        "/",
        rocket::routes![
            index,
            post_index,
            settings,
            mw_room_input,
            mw_room_view,
            mw_click,
            mw_notes,
            restream_room_input,
            restream_room_view,
            restream_click,
            restream_double_room_layout,
            room,
            room_with_layout,
            click,
            click_with_layout,
            api_checked_locations,
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use oottracker::flag_mapping::CheckStatus;
    use rocket::{
        http::{ContentType, Status},
        local::asynchronous::Client,
    };
    use std::collections::HashMap;
    use tokio::sync::Mutex;

    /// Creates a test Rocket instance without database dependency.
    /// Uses in-memory state for rooms and empty restreams/mw_rooms.
    /// Mounts all routes to avoid "unused import" warnings.
    async fn test_rocket() -> Rocket<rocket::Build> {
        let rooms = Rooms::new(Mutex::new(HashMap::default()));
        let restreams = Restreams::default();
        let mw_rooms = MwRooms::default();

        rocket::custom(rocket::Config {
            port: 24807,
            ..rocket::Config::default()
        })
        .manage(rooms)
        .manage(restreams)
        .manage(mw_rooms)
        .mount(
            "/",
            // Mount all routes to prevent "unused" warnings on route functions.
            // Note: routes requiring database (room, click) or Python (mw_notes)
            // will return errors when called without proper state, but this
            // ensures all route handlers are referenced during test compilation.
            rocket::routes![
                index,
                post_index,
                settings,
                mw_room_input,
                mw_room_view,
                mw_click,
                mw_notes,
                restream_room_input,
                restream_room_view,
                restream_click,
                restream_double_room_layout,
                room,
                room_with_layout,
                click,
                click_with_layout,
                api_checked_locations,
            ],
        )
    }

    // ==========================================================================
    // Index Route Tests
    // ==========================================================================

    #[rocket::async_test]
    async fn test_index_returns_html() {
        let client = Client::tracked(test_rocket().await)
            .await
            .expect("valid rocket instance");

        let response = client.get("/").dispatch().await;
        assert_eq!(response.status(), Status::Ok);

        let body = response.into_string().await.expect("response body");
        assert!(body.contains("OoT Tracker"));
        assert!(body.contains("<html>"));
    }

    #[rocket::async_test]
    async fn test_index_contains_version() {
        let client = Client::tracked(test_rocket().await)
            .await
            .expect("valid rocket instance");

        let response = client.get("/").dispatch().await;
        let body = response.into_string().await.expect("response body");

        // The version placeholder {} is replaced with CARGO_PKG_VERSION
        assert!(body.contains(env!("CARGO_PKG_VERSION")));
    }

    #[rocket::async_test]
    async fn test_index_contains_room_form() {
        let client = Client::tracked(test_rocket().await)
            .await
            .expect("valid rocket instance");

        let response = client.get("/").dispatch().await;
        let body = response.into_string().await.expect("response body");

        // Verify form elements are present
        assert!(body.contains(r#"method="POST""#));
        assert!(body.contains(r#"action="/""#));
        assert!(body.contains(r#"name="room""#));
    }

    // ==========================================================================
    // Settings Route Tests
    // ==========================================================================

    #[rocket::async_test]
    async fn test_settings_returns_html() {
        let client = Client::tracked(test_rocket().await)
            .await
            .expect("valid rocket instance");

        let response = client.get("/settings").dispatch().await;
        assert_eq!(response.status(), Status::Ok);

        let body = response.into_string().await.expect("response body");
        assert!(body.contains("OoTMM Tracker Settings"));
        assert!(body.contains("<html>"));
    }

    #[rocket::async_test]
    async fn test_settings_contains_form_elements() {
        let client = Client::tracked(test_rocket().await)
            .await
            .expect("valid rocket instance");

        let response = client.get("/settings").dispatch().await;
        let body = response.into_string().await.expect("response body");

        // Verify key form elements are present
        assert!(body.contains(r#"id="settings-form""#));
        assert!(body.contains(r#"name="logicMode""#));
        assert!(body.contains(r#"name="openDungeonsOot""#));
        assert!(body.contains(r#"name="agelessBoots""#));
    }

    // ==========================================================================
    // Post Index (Form Submission) Tests
    // ==========================================================================

    #[rocket::async_test]
    async fn test_post_index_redirects_to_room() {
        let client = Client::tracked(test_rocket().await)
            .await
            .expect("valid rocket instance");

        let response = client
            .post("/")
            .header(ContentType::Form)
            .body("room=test-room")
            .dispatch()
            .await;

        // Should redirect to the room
        assert_eq!(response.status(), Status::SeeOther);

        let location = response
            .headers()
            .get_one("Location")
            .expect("redirect location");
        assert!(location.contains("/room/test-room"));
    }

    #[rocket::async_test]
    async fn test_post_index_with_hyphenated_room_name() {
        let client = Client::tracked(test_rocket().await)
            .await
            .expect("valid rocket instance");

        let response = client
            .post("/")
            .header(ContentType::Form)
            .body("room=my-test-room-123")
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::SeeOther);
        let location = response
            .headers()
            .get_one("Location")
            .expect("redirect location");
        assert!(location.contains("/room/my-test-room-123"));
    }

    #[rocket::async_test]
    async fn test_post_index_empty_room_validation() {
        let client = Client::tracked(test_rocket().await)
            .await
            .expect("valid rocket instance");

        // Empty room name should fail validation
        let response = client
            .post("/")
            .header(ContentType::Form)
            .body("room=")
            .dispatch()
            .await;

        // Rocket returns 422 Unprocessable Entity for validation failures
        assert_eq!(response.status(), Status::UnprocessableEntity);
    }

    // ==========================================================================
    // Multiworld Room Input Route Tests
    // ==========================================================================

    #[rocket::async_test]
    async fn test_mw_room_input_redirects_to_view() {
        let client = Client::tracked(test_rocket().await)
            .await
            .expect("valid rocket instance");

        let response = client.get("/mw/test-room/1").dispatch().await;

        // Should permanent redirect to the full view URL with default layout
        assert_eq!(response.status(), Status::PermanentRedirect);

        let location = response
            .headers()
            .get_one("Location")
            .expect("redirect location");
        assert!(location.contains("/mw/test-room/1/default"));
    }

    #[rocket::async_test]
    async fn test_mw_room_input_with_theme() {
        let client = Client::tracked(test_rocket().await)
            .await
            .expect("valid rocket instance");

        let response = client.get("/mw/test-room/2?theme=Dark").dispatch().await;

        assert_eq!(response.status(), Status::PermanentRedirect);
        let location = response
            .headers()
            .get_one("Location")
            .expect("redirect location");
        // Theme should be passed through
        assert!(location.contains("theme=Dark"));
    }

    #[rocket::async_test]
    async fn test_mw_room_input_with_delay() {
        let client = Client::tracked(test_rocket().await)
            .await
            .expect("valid rocket instance");

        let response = client.get("/mw/test-room/1?delay=1.5").dispatch().await;

        assert_eq!(response.status(), Status::PermanentRedirect);
        let location = response
            .headers()
            .get_one("Location")
            .expect("redirect location");
        assert!(location.contains("delay=1.5"));
    }

    // ==========================================================================
    // Restream Room Input Route Tests
    // ==========================================================================

    #[rocket::async_test]
    async fn test_restream_room_input_redirects_to_view() {
        let client = Client::tracked(test_rocket().await)
            .await
            .expect("valid rocket instance");

        let response = client.get("/restream/fenhl/player1").dispatch().await;

        assert_eq!(response.status(), Status::PermanentRedirect);
        let location = response
            .headers()
            .get_one("Location")
            .expect("redirect location");
        assert!(location.contains("/restream/fenhl/player1/default"));
    }

    #[rocket::async_test]
    async fn test_restream_room_input_with_theme() {
        let client = Client::tracked(test_rocket().await)
            .await
            .expect("valid rocket instance");

        let response = client
            .get("/restream/fenhl/player1?theme=Light")
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::PermanentRedirect);
        let location = response
            .headers()
            .get_one("Location")
            .expect("redirect location");
        assert!(location.contains("theme=Light"));
    }

    // ==========================================================================
    // Theme Enum Tests
    // ==========================================================================

    #[test]
    fn test_theme_from_form_field_light() {
        use rocket::form::{FromFormField, ValueField};
        let field = ValueField::from_value("Light");
        let theme: Theme = Theme::from_value(field).unwrap();
        assert!(matches!(theme, Theme::Light));
    }

    #[test]
    fn test_theme_from_form_field_dark() {
        use rocket::form::{FromFormField, ValueField};
        let field = ValueField::from_value("Dark");
        let theme: Theme = Theme::from_value(field).unwrap();
        assert!(matches!(theme, Theme::Dark));
    }

    // ==========================================================================
    // TrackerCellIdExt Tests
    // ==========================================================================

    #[test]
    fn test_tracker_cell_id_view_generates_form() {
        use oottracker::ui::TrackerCellId;
        use rocket::http::uri::Origin;

        let state = ModelState::default();
        let cell_id = TrackerCellId::KokiriEmerald;
        let click_uri = Origin::parse("/room/test/click/0").unwrap();

        let html = cell_id.view(click_uri, 0, &state, 1, false);
        let html_str = html.0;

        // Verify form structure
        assert!(html_str.contains(r#"method="POST""#));
        assert!(html_str.contains(r#"action="/room/test/click/0""#));
        assert!(html_str.contains(r#"id="cell0""#));
        assert!(html_str.contains("<button"));
    }

    #[test]
    fn test_tracker_cell_id_view_with_colspan() {
        use oottracker::ui::TrackerCellId;
        use rocket::http::uri::Origin;

        let state = ModelState::default();
        let cell_id = TrackerCellId::KokiriEmerald;
        let click_uri = Origin::parse("/click/0").unwrap();

        let html = cell_id.view(click_uri, 0, &state, 3, false);
        let html_str = html.0;

        assert!(html_str.contains("cols3"));
    }

    #[test]
    fn test_tracker_cell_id_view_with_loc_class() {
        use oottracker::ui::TrackerCellId;
        use rocket::http::uri::Origin;

        let state = ModelState::default();
        let cell_id = TrackerCellId::KokiriEmerald;
        let click_uri = Origin::parse("/click/0").unwrap();

        // With loc=true, should have "loc" class
        let html = cell_id.view(click_uri, 0, &state, 1, true);
        let html_str = html.0;

        assert!(html_str.contains("loc"));
    }

    // ==========================================================================
    // Tracker Page Generation Tests
    // ==========================================================================

    #[test]
    fn test_tracker_page_generates_valid_html() {
        use rocket_util::html;

        let items = html! {
            div : "test content";
        };

        let page = tracker_page("default", None, items);
        let html_str = page.0;

        // Verify HTML structure
        assert!(html_str.contains("<!DOCTYPE html>"));
        assert!(html_str.contains("<html>"));
        assert!(html_str.contains("</html>"));
        assert!(html_str.contains("<title>OoT Tracker</title>"));
        assert!(html_str.contains("test content"));
    }

    #[test]
    fn test_tracker_page_with_light_theme() {
        use rocket_util::html;

        let items = html! {};
        let page = tracker_page("default", Some(Theme::Light), items);
        let html_str = page.0;

        // Light theme should include light.css without media query
        assert!(html_str.contains(r#"href="/static/light.css""#));
        assert!(!html_str.contains("prefers-color-scheme"));
    }

    #[test]
    fn test_tracker_page_with_dark_theme() {
        use rocket_util::html;

        let items = html! {};
        let page = tracker_page("default", Some(Theme::Dark), items);
        let html_str = page.0;

        // Dark theme should NOT include light.css
        assert!(!html_str.contains("light.css"));
    }

    #[test]
    fn test_tracker_page_with_no_theme() {
        use rocket_util::html;

        let items = html! {};
        let page = tracker_page("default", None, items);
        let html_str = page.0;

        // No theme should include light.css with media query
        assert!(html_str.contains("prefers-color-scheme: light"));
    }

    #[test]
    fn test_tracker_page_includes_layout_class() {
        use rocket_util::html;

        let items = html! {};
        let page = tracker_page("mw-expanded", None, items);
        let html_str = page.0;

        assert!(html_str.contains(r#"class="items mw-expanded""#));
    }

    // ==========================================================================
    // World Class Helper Tests
    // ==========================================================================

    #[test]
    fn test_world_class_player_1() {
        let world_id = NonZeroU8::new(1).unwrap();
        assert_eq!(world_class(world_id), Some("power"));
    }

    #[test]
    fn test_world_class_player_2() {
        let world_id = NonZeroU8::new(2).unwrap();
        assert_eq!(world_class(world_id), Some("wisdom"));
    }

    #[test]
    fn test_world_class_player_3() {
        let world_id = NonZeroU8::new(3).unwrap();
        assert_eq!(world_class(world_id), Some("courage"));
    }

    #[test]
    fn test_world_class_player_4_and_above() {
        let world_id = NonZeroU8::new(4).unwrap();
        assert_eq!(world_class(world_id), None);

        let world_id = NonZeroU8::new(5).unwrap();
        assert_eq!(world_class(world_id), None);
    }

    // ==========================================================================
    // GoRoomForm Tests
    // ==========================================================================

    #[test]
    fn test_go_room_form_valid() {
        // Test that the form struct compiles and has expected fields
        let _form = GoRoomForm { room: "test-room" };
    }

    // ==========================================================================
    // Error Response Tests
    // ==========================================================================

    #[rocket::async_test]
    async fn test_nonexistent_route_returns_404() {
        let client = Client::tracked(test_rocket().await)
            .await
            .expect("valid rocket instance");

        let response = client.get("/nonexistent/path").dispatch().await;
        assert_eq!(response.status(), Status::NotFound);
    }

    #[rocket::async_test]
    async fn test_invalid_world_id_format() {
        let client = Client::tracked(test_rocket().await)
            .await
            .expect("valid rocket instance");

        // World ID must be a positive integer, "abc" should fail
        let response = client.get("/mw/test-room/abc").dispatch().await;
        assert_eq!(response.status(), Status::NotFound);
    }

    #[rocket::async_test]
    async fn test_world_id_zero_fails() {
        let client = Client::tracked(test_rocket().await)
            .await
            .expect("valid rocket instance");

        // NonZeroU8 cannot be 0
        let response = client.get("/mw/test-room/0").dispatch().await;
        assert_eq!(response.status(), Status::NotFound);
    }

    // ==========================================================================
    // Checked Locations API Tests
    // ==========================================================================

    #[rocket::async_test]
    async fn test_checked_locations_valid_room() {
        let client = Client::tracked(test_rocket().await)
            .await
            .expect("valid rocket instance");

        // Valid room name should return 200 OK with JSON
        let response = client
            .get("/api/room/test-room-432/checked-locations")
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Ok);

        // Verify we get valid JSON
        let summary: CheckedLocationsSummary = response.into_json().await.unwrap();
        assert!(summary.total_mapped >= 0);
    }

    #[rocket::async_test]
    async fn test_checked_locations_response_structure() {
        let client = Client::tracked(test_rocket().await)
            .await
            .expect("valid rocket instance");

        let response = client
            .get("/api/room/test-struct-check/checked-locations")
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Ok);

        let summary: CheckedLocationsSummary = response.into_json().await.unwrap();

        // Verify all required fields are present and valid
        // total_mapped should equal checked + unchecked + unknown
        assert_eq!(
            summary.total_mapped,
            summary.checked_count + summary.unchecked_count + summary.unknown_count
        );

        // locations vector should exist (may be empty for new room)
        assert!(summary.locations.len() <= summary.total_mapped);
    }

    #[rocket::async_test]
    async fn test_checked_locations_array_structure() {
        let client = Client::tracked(test_rocket().await)
            .await
            .expect("valid rocket instance");

        let response = client
            .get("/api/room/test-array-check/checked-locations")
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Ok);

        let summary: CheckedLocationsSummary = response.into_json().await.unwrap();

        // Each location should have location_id and status fields
        for location in &summary.locations {
            // location_id should be a non-empty string
            assert!(!location.location_id.is_empty());

            // status should be one of the valid enum values
            // (this is enforced by the type system, but we verify deserialization worked)
            let _ = match location.status {
                CheckStatus::Checked => "checked",
                CheckStatus::Unchecked => "unchecked",
                CheckStatus::Unknown => "unknown",
            };

            // is_mapped field should exist
            let _ = location.is_mapped;
        }
    }

    #[rocket::async_test]
    async fn test_checked_locations_status_values_valid() {
        let client = Client::tracked(test_rocket().await)
            .await
            .expect("valid rocket instance");

        let response = client
            .get("/api/room/test-status-check/checked-locations")
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Ok);

        let summary: CheckedLocationsSummary = response.into_json().await.unwrap();

        // Count status values manually and verify they match summary counts
        let mut checked = 0;
        let mut unchecked = 0;
        let mut unknown = 0;

        for location in &summary.locations {
            match location.status {
                CheckStatus::Checked => checked += 1,
                CheckStatus::Unchecked => unchecked += 1,
                CheckStatus::Unknown => unknown += 1,
            }
        }

        // The counts should match what we counted from the array
        assert_eq!(summary.checked_count, checked);
        assert_eq!(summary.unchecked_count, unchecked);
        assert_eq!(summary.unknown_count, unknown);
    }

    #[rocket::async_test]
    async fn test_checked_locations_empty_room() {
        let client = Client::tracked(test_rocket().await)
            .await
            .expect("valid rocket instance");

        // Fresh room should auto-create with default state
        let response = client
            .get("/api/room/brand-new-room-empty/checked-locations")
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Ok);

        let summary: CheckedLocationsSummary = response.into_json().await.unwrap();

        // For a new room with default ModelState, checked_count should be 0
        // (no locations have been checked yet)
        assert_eq!(summary.checked_count, 0);
    }

    #[rocket::async_test]
    async fn test_checked_locations_invalid_room_name_uppercase() {
        let client = Client::tracked(test_rocket().await)
            .await
            .expect("valid rocket instance");

        // Room names must match ^[0-9a-z]+(?:-[0-9a-z]+)*$
        // Uppercase letters are invalid
        let response = client
            .get("/api/room/InvalidName/checked-locations")
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::NotFound);
    }

    #[rocket::async_test]
    async fn test_checked_locations_invalid_room_name_special_chars() {
        let client = Client::tracked(test_rocket().await)
            .await
            .expect("valid rocket instance");

        // Special characters are invalid
        let response = client
            .get("/api/room/test_room/checked-locations")
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::NotFound);
    }

    #[rocket::async_test]
    async fn test_checked_locations_invalid_room_name_leading_hyphen() {
        let client = Client::tracked(test_rocket().await)
            .await
            .expect("valid rocket instance");

        // Leading hyphen is invalid
        let response = client
            .get("/api/room/-invalid/checked-locations")
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::NotFound);
    }

    #[rocket::async_test]
    async fn test_checked_locations_counts_match_array_length() {
        let client = Client::tracked(test_rocket().await)
            .await
            .expect("valid rocket instance");

        let response = client
            .get("/api/room/test-counts-integration/checked-locations")
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Ok);

        let summary: CheckedLocationsSummary = response.into_json().await.unwrap();

        // Integration check: locations array length should equal sum of counts
        assert_eq!(
            summary.locations.len(),
            summary.checked_count + summary.unchecked_count + summary.unknown_count
        );

        // Also verify total_mapped equals the same sum
        assert_eq!(
            summary.total_mapped,
            summary.checked_count + summary.unchecked_count + summary.unknown_count
        );
    }
}
