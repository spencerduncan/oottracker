#![deny(
    rust_2018_idioms,
    unused,
    unused_crate_dependencies,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    warnings
)]
#![allow(clippy::large_enum_variant)]
#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Mark tokio as used even on platforms where it's not directly imported
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
use tokio as _;

#[cfg(target_os = "windows")]
use tokio::fs;
use {
    derivative::Derivative,
    derive_more::From,
    enum_iterator::{all, Sequence},
    futures::future::FutureExt as _,
    iced::{
        alignment,
        widget::{
            button::{self, Button},
            container::{self, Container},
            pick_list::{self, PickList},
            text_input::{self, TextInput},
            Column, Image, Row, Space, Text,
        },
        window::{self, Icon},
        Application, Background, Color, Command, Element, Length, Settings,
    },
    iced_futures::Subscription,
    iced_native::{command::Action, keyboard::Modifiers as KeyboardModifiers},
    image::DynamicImage,
    itertools::Itertools as _,
    ootr::Rando,
    oottracker::{
        firebase,
        flag_mapping::{get_checked_locations_summary_filtered, CheckedLocationsSummary},
        github::Repo,
        net::{self, Connection},
        proto::Packet,
        save::*,
        ui::{self, CellStyle, LayoutPreference, *},
        ModelState,
    },
    semver::Version,
    std::{convert::Infallible as Never, env, fmt, io, sync::Arc},
    url::Url,
    wheel::FromArc,
};
#[cfg(target_os = "macos")]
use {
    futures::stream::TryStreamExt as _,
    std::time::Duration,
    tokio::{fs, fs::File, io::AsyncWriteExt as _, time::sleep},
};

#[cfg(feature = "audio")]
mod audio;
mod check_panel;
mod logic;
mod subscriptions;

const CELL_SIZE: u16 = 50;
const STONE_SIZE: u16 = 30;

/// Default dimensions for window initialization (OoT default layout)
const DEFAULT_WIDTH: u32 = 360; // 6 columns * 60px
const DEFAULT_HEIGHT: u32 = 448; // medallion row + 7 cell rows

struct ContainerStyle;

impl container::StyleSheet for ContainerStyle {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::BLACK)),
            ..container::Style::default()
        }
    }
}

fn cell_image(cell: &TrackerCellId, state: &ModelState) -> Image {
    let kind = cell.kind();
    let CellRender {
        img,
        style,
        overlay,
        accessibility: _, // Logic accessibility not used in GUI rendering
    } = kind.render(state);
    match (style, overlay) {
        (CellStyle::Normal, CellOverlay::None) => img.embedded::<Image>(ImageDirContext::Normal),
        (CellStyle::Normal, CellOverlay::Count { count, count_img }) => {
            count_img.embedded(ImageDirContext::Count(count))
        }
        (CellStyle::Normal, CellOverlay::Image(overlay)) => {
            img.with_overlay(&overlay).embedded(true)
        }
        (CellStyle::Dimmed, CellOverlay::None) => img.embedded(ImageDirContext::Dimmed),
        (CellStyle::Dimmed, CellOverlay::Image(overlay)) => {
            img.with_overlay(&overlay).embedded(false)
        }
        (_, CellOverlay::Location { loc, style }) => loc.embedded(match style {
            LocationStyle::Normal => ImageDirContext::Normal,
            LocationStyle::Dimmed => ImageDirContext::Dimmed,
            LocationStyle::Mq => unimplemented!(),
        }),
        (CellStyle::Dimmed, CellOverlay::Count { .. })
        | (CellStyle::LeftDimmed | CellStyle::RightDimmed, _) => unimplemented!(),
    }
    .width(Length::Units(match kind {
        TrackerCellKind::Stone(_) | TrackerCellKind::StoneLocation(_) => STONE_SIZE,
        _ => CELL_SIZE,
    }))
}

trait TrackerCellIdExt {
    fn view<'a>(
        &self,
        state: &ModelState,
        cell_button: &'a mut button::State,
    ) -> Element<'a, Message<ootr_static::Rando>>; //TODO allow ootr_dynamic::Rando
}

impl TrackerCellIdExt for TrackerCellId {
    fn view<'a>(
        &self,
        state: &ModelState,
        cell_button: &'a mut button::State,
    ) -> Element<'a, Message<ootr_static::Rando>> {
        //TODO allow ootr_dynamic::Rando
        Button::new(cell_button, cell_image(self, state))
            .on_press(Message::LeftClick(*self))
            .padding(0)
            .style(DefaultButtonStyle)
            .into()
    }
}

struct DefaultButtonStyle;

impl button::StyleSheet for DefaultButtonStyle {
    fn active(&self) -> button::Style {
        button::Style::default()
    }
}

trait TrackerLayoutExt {
    fn cell_at(&self, pos: [f32; 2], include_songs: bool) -> Option<TrackerCellId>;
}

impl TrackerLayoutExt for TrackerLayout {
    fn cell_at(&self, [x, y]: [f32; 2], include_songs: bool) -> Option<TrackerCellId> {
        let cells = self.cells();

        // If not including songs, calculate the y-threshold based on row positions
        // For OoT layouts, songs are typically in the last 2 rows (rows 6 and 7)
        if !include_songs {
            // Get sorted unique y positions to find row boundaries
            let mut y_positions: Vec<u16> = cells.iter().map(|c| c.pos[1]).collect();
            y_positions.sort_unstable();
            y_positions.dedup();

            // If we have more than 5 rows, exclude the last 2 rows (songs area)
            if y_positions.len() > 5 {
                let songs_start_y = y_positions[5];
                if y >= songs_start_y as f32 {
                    return None;
                }
            }
        }

        cells
            .into_iter()
            .find(
                |CellLayout {
                     pos: [pos_x, pos_y],
                     size: [size_x, size_y],
                     ..
                 }| {
                    (*pos_x..pos_x + size_x).contains(&(x as u16))
                        && (*pos_y..pos_y + size_y).contains(&(y as u16))
                },
            )
            .map(|CellLayout { id, .. }| id)
    }
}

#[derive(Derivative)]
#[derivative(Debug(bound = ""), Clone(bound = ""))]
enum Message<R: Rando> {
    #[allow(dead_code)] // Preparatory for check panel feature
    CheckedLocationsUpdated(CheckedLocationsSummary),
    ClientDisconnected,
    CloseMenu,
    ConfigError(ui::Error),
    ConnectionError(ConnectionError),
    Connect,
    DismissNotification,
    DismissWelcomeScreen,
    InstallUpdate,
    KeyboardModifiers(KeyboardModifiers),
    LeftClick(TrackerCellId),
    LoadConfig(Config),
    Logic(logic::Message<R>),
    MouseMoved([f32; 2]),
    Nop,
    Packet(Packet),
    ResetUpdateState,
    RightClick,
    SetAutoUpdateCheck(bool),
    SetItemFanfarePath(String),
    SetLayoutPreference(LayoutPreference),
    SetMedOrder(ElementOrder),
    SetPasscode(String),
    SetConnection(Arc<dyn Connection>),
    SetConnectionKind(ConnectionKind),
    SetUrl(String),
    SetWarpSongOrder(ElementOrder),
    ToggleCheckPanel,
    UpdateCheck,
    UpdateCheckComplete(Option<Version>),
    UpdateCheckError(UpdateCheckError),
}

impl<R: Rando> fmt::Display for Message<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Message::ClientDisconnected => write!(f, "connection lost"),
            Message::ConfigError(e) => write!(f, "error loading/saving preferences: {}", e),
            Message::ConnectionError(e) => write!(f, "connection error: {}", e),
            _ => write!(f, "{:?}", self), // these messages are not notifications so just fall back to Debug
        }
    }
}

#[derive(Debug, Default)]
struct MenuState {
    dismiss_btn: button::State,
    layout_preference: pick_list::State<LayoutPreference>,
    med_order: pick_list::State<ElementOrder>,
    warp_song_order: pick_list::State<ElementOrder>,
    item_fanfare_path: text_input::State,
    connection_kind: pick_list::State<ConnectionKind>,
    connection_params: ConnectionParams,
    connect_btn: button::State,
}

#[derive(Derivative, Debug, Sequence, Clone, Copy, PartialEq, Eq)]
#[derivative(Default)]
enum ConnectionKind {
    #[derivative(Default)]
    TcpListener,
    Web,
}

impl fmt::Display for ConnectionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectionKind::TcpListener => write!(f, "Project64"),
            ConnectionKind::Web => write!(f, "web"),
        }
    }
}

#[derive(Derivative, Debug, Clone)]
#[derivative(Default)]
enum ConnectionParams {
    #[derivative(Default)]
    TcpListener,
    Web {
        url: String,
        url_state: text_input::State,
        passcode: String,
        passcode_state: text_input::State,
    },
}

impl ConnectionParams {
    fn kind(&self) -> ConnectionKind {
        match self {
            ConnectionParams::TcpListener => ConnectionKind::TcpListener,
            ConnectionParams::Web { .. } => ConnectionKind::Web,
        }
    }

    fn set_kind(&mut self, kind: ConnectionKind) {
        if kind == self.kind() {
            return;
        }
        *self = match kind {
            ConnectionKind::TcpListener => ConnectionParams::TcpListener,
            ConnectionKind::Web => ConnectionParams::Web {
                url: String::default(),
                url_state: text_input::State::default(),
                passcode: String::default(),
                passcode_state: text_input::State::default(),
            },
        };
    }

    fn view<R: Rando + 'static>(&mut self) -> Element<'_, Message<R>> {
        match self {
            ConnectionParams::TcpListener => Row::new().into(),
            ConnectionParams::Web {
                url,
                url_state,
                passcode,
                passcode_state,
            } => Column::new()
                .push(TextInput::new(url_state, "URL", url, Message::SetUrl))
                .push(
                    TextInput::new(passcode_state, "passcode", passcode, Message::SetPasscode)
                        .password(),
                )
                .into(),
        }
    }
}

#[derive(Debug)]
struct State<R: Rando + 'static> {
    flags: Args,
    config: Option<Config>,
    http_client: reqwest::Client,
    update_check: UpdateCheckState,
    connection: Option<Arc<dyn Connection>>,
    keyboard_modifiers: KeyboardModifiers,
    last_cursor_pos: [f32; 2],
    dismiss_welcome_screen_button: button::State,
    enable_update_checks_button: button::State,
    disable_update_checks_button: button::State,
    cell_buttons: [button::State; 52],
    rando: Arc<R>,
    model: ModelState,
    logic: logic::State<R>,
    notification: Option<(bool, Message<R>)>,
    dismiss_notification_button: button::State,
    menu_state: Option<MenuState>,
    #[cfg(feature = "audio")]
    audio_player: Option<audio::AudioPlayer>,
    /// Summary of checked locations from flag mapping.
    /// Updated whenever model state changes.
    checked_locations: Option<CheckedLocationsSummary>,
    /// Whether to show the check panel on the right side of the tracker.
    show_check_panel: bool,
}

impl<R: Rando + 'static> State<R> {
    fn layout(&self) -> TrackerLayout {
        // Determine base layout based on layout preference
        let layout_pref = self
            .config
            .as_ref()
            .map(|cfg| cfg.layout_preference)
            .unwrap_or_default();

        match layout_pref {
            LayoutPreference::Oot => {
                // Original OoT layout logic
                if self
                    .connection
                    .as_ref()
                    .is_none_or(|connection| connection.can_change_state())
                {
                    TrackerLayout::from(&self.config)
                } else if let Some(ref config) = self.config {
                    TrackerLayout::new_auto(config)
                } else {
                    TrackerLayout::default_auto()
                }
            }
            LayoutPreference::Mm => TrackerLayout::MmDefault,
            LayoutPreference::Combo => TrackerLayout::Combo,
            LayoutPreference::DungeonItems => TrackerLayout::DungeonItems,
            LayoutPreference::MmDungeonItems => TrackerLayout::MmDungeonItems,
            LayoutPreference::MmStrayFairies => TrackerLayout::MmStrayFairies,
        }
    }

    /// Adds a visible notification/alert/log message.
    ///
    /// Implemented as a separate method in case the way this is displayed is changed later, e.g. to allow multiple notifications.
    #[must_use]
    fn notify(&mut self, message: Message<R>) -> Command<Message<R>> {
        self.notification = Some((false, message));
        Command::none()
    }

    fn save_config(&self) -> Command<Message<R>> {
        if let Some(ref config) = self.config {
            let config = config.clone();
            Command::single(Action::Future(
                async move {
                    match config.save().await {
                        Ok(()) => Message::Nop,
                        Err(e) => Message::ConfigError(e),
                    }
                }
                .boxed(),
            ))
        } else {
            Command::none()
        }
    }

    /// Plays the item fanfare sound if configured.
    #[cfg(feature = "audio")]
    fn play_fanfare(&self) {
        if let (Some(ref config), Some(ref player)) = (&self.config, &self.audio_player) {
            if let Some(ref path) = config.item_fanfare_path {
                player.play(path);
            }
        }
    }

    /// No-op when audio feature is disabled.
    #[cfg(not(feature = "audio"))]
    fn play_fanfare(&self) {}
}

impl Default for State<ootr_static::Rando> {
    fn default() -> State<ootr_static::Rando> {
        State {
            flags: Args::default(),
            config: None,
            http_client: reqwest::Client::builder()
                .user_agent(concat!("oottracker/", env!("CARGO_PKG_VERSION")))
                .http2_prior_knowledge()
                .use_rustls_tls()
                .https_only(true)
                .build()
                .expect("failed to build HTTP client"),
            update_check: UpdateCheckState::Unknown(button::State::default()),
            connection: None,
            keyboard_modifiers: KeyboardModifiers::default(),
            last_cursor_pos: [0.0, 0.0],
            dismiss_welcome_screen_button: button::State::default(),
            enable_update_checks_button: button::State::default(),
            disable_update_checks_button: button::State::default(),
            cell_buttons: [
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
                button::State::default(),
            ],
            rando: Arc::new(ootr_static::Rando),
            model: ModelState::default(),
            logic: logic::State::default(),
            notification: None,
            dismiss_notification_button: button::State::default(),
            menu_state: None,
            #[cfg(feature = "audio")]
            audio_player: audio::AudioPlayer::new(),
            checked_locations: None,
            show_check_panel: false,
        }
    }
}

impl From<Args> for State<ootr_static::Rando> {
    //TODO include Rando in flags and make this impl generic
    fn from(flags: Args) -> State<ootr_static::Rando> {
        State {
            flags,
            ..State::default()
        }
    }
}

impl Application for State<ootr_static::Rando> {
    //TODO include Rando in flags and make this impl generic
    type Executor = iced::executor::Default;
    type Message = Message<ootr_static::Rando>;
    type Flags = Args;

    fn new(
        flags: Args,
    ) -> (
        State<ootr_static::Rando>,
        Command<Message<ootr_static::Rando>>,
    ) {
        (
            State::from(flags),
            Command::single(Action::Future(
                async {
                    match Config::new().await {
                        Ok(Some(config)) => Message::LoadConfig(config),
                        Ok(None) => Message::Nop,
                        Err(e) => Message::ConfigError(e),
                    }
                }
                .boxed(),
            )),
        )
    }

    fn title(&self) -> String {
        if let Some(ref connection) = self.connection {
            format!("OoT Tracker ({} connected)", connection.display_kind())
        } else {
            "OoT Tracker".to_string()
        }
    }

    fn update(
        &mut self,
        message: Message<ootr_static::Rando>,
    ) -> Command<Message<ootr_static::Rando>> {
        match message {
            Message::CheckedLocationsUpdated(summary) => {
                self.checked_locations = Some(summary);
            }
            Message::ClientDisconnected => {
                if self
                    .notification
                    .as_ref()
                    .is_none_or(|&(is_temp, _)| is_temp)
                {
                    // don't override an existing, probably more descriptive error message
                    return self.notify(message);
                }
            }
            Message::CloseMenu => self.menu_state = None,
            Message::ConfigError(_) => return self.notify(message),
            Message::Connect => {
                if self.connection.is_some() {
                    self.connection = None;
                } else if let Some(ref menu_state) = self.menu_state {
                    let params = menu_state.connection_params.clone();
                    let model = self.model.clone();
                    return Command::single(Action::Future(
                        async move {
                            match connect(params, model).await {
                                Ok(connection) => Message::SetConnection(connection),
                                Err(e) => Message::ConnectionError(e),
                            }
                        }
                        .boxed(),
                    ));
                }
            }
            Message::ConnectionError(_) => return self.notify(message),
            Message::DismissNotification => self.notification = None,
            Message::DismissWelcomeScreen => {
                self.config = Some(Config::default());
                return self.save_config();
            }
            Message::InstallUpdate => {
                self.update_check = UpdateCheckState::Installing;
                let client = self.http_client.clone();
                return Command::single(Action::Future(
                    async move {
                        match run_updater(&client).await {
                            Ok(never) => match never {},
                            Err(e) => Message::UpdateCheckError(e),
                        }
                    }
                    .boxed(),
                ));
            }
            Message::KeyboardModifiers(modifiers) => self.keyboard_modifiers = modifiers,
            Message::LeftClick(cell) => {
                let kind = cell.kind();
                // Check if item was dimmed (not collected) before click
                let was_dimmed = matches!(
                    kind.render(&self.model).style,
                    CellStyle::Dimmed | CellStyle::LeftDimmed | CellStyle::RightDimmed
                );
                if kind.left_click(
                    self.connection
                        .as_ref()
                        .is_none_or(|connection| connection.can_change_state()),
                    self.keyboard_modifiers,
                    &mut self.model,
                ) {
                    self.menu_state = Some(MenuState::default());
                } else {
                    // Check if item is now collected (not dimmed)
                    let is_collected = matches!(
                        kind.render(&self.model).style,
                        CellStyle::Normal | CellStyle::LeftDimmed | CellStyle::RightDimmed
                    );
                    // Play fanfare if item changed from not-collected to collected
                    if was_dimmed && is_collected {
                        self.play_fanfare();
                    }
                    if let Some(ref connection) = self.connection {
                        if connection.can_change_state() {
                            let send_fut = connection.set_state(&self.model);
                            return Command::single(Action::Future(
                                async move {
                                    match send_fut.await {
                                        Ok(()) => Message::Nop,
                                        Err(e) => Message::ConnectionError(e.into()),
                                    }
                                }
                                .boxed(),
                            ));
                        }
                    }
                }
            }
            Message::LoadConfig(config) => match config.version {
                0 => {
                    let auto_update_check = config.auto_update_check;
                    self.config = Some(config);
                    if auto_update_check == Some(true) {
                        return Command::single(Action::Future(
                            async { Message::UpdateCheck }.boxed(),
                        ));
                    }
                }
                v => unimplemented!("config version from the future: {}", v),
            },
            Message::Logic(msg) => return self.logic.update(msg),
            Message::MouseMoved(pos) => self.last_cursor_pos = pos,
            Message::Nop => {}
            Message::Packet(packet) => {
                match packet {
                    Packet::Goodbye => unreachable!(), // Goodbye is not yielded from proto::read
                    Packet::SaveDelta(delta) => {
                        self.model.ram.save = &self.model.ram.save + &delta;
                        self.model.update_knowledge();
                    }
                    Packet::SaveInit(save) => {
                        self.model.ram.save = save;
                        self.model.update_knowledge();
                    }
                    Packet::KnowledgeInit(knowledge) => self.model.knowledge = knowledge,
                    Packet::RamInit(ram) => {
                        if ram.save.game_mode == GameMode::Gameplay {
                            self.model.ram = ram
                        }
                        self.model.update_knowledge();
                    }
                    Packet::UpdateCell(cell_id, value) => {
                        if let Some(ref connection) = self.connection {
                            if let Some(app) = connection.firebase_app() {
                                if let Err(e) = app.set_cell(&mut self.model, cell_id, value) {
                                    return self.notify(Message::ConnectionError(
                                        ConnectionError::from(e),
                                    ));
                                }
                            }
                        }
                    }
                    Packet::ModelInit(model) => {
                        self.model = model;
                        self.model.update_knowledge();
                    }
                    Packet::ModelDelta(delta) => {
                        self.model += delta;
                        self.model.update_knowledge();
                    }
                    Packet::MmRamInit(mm_save) => {
                        self.model.ram.mm_save = Some(mm_save);
                        self.model.update_knowledge();
                    }
                }
                // Update checked locations summary after model changes
                self.checked_locations = Some(get_checked_locations_summary_filtered(&self.model));
            }
            Message::ResetUpdateState => {
                self.update_check = UpdateCheckState::Unknown(button::State::default())
            }
            Message::RightClick => {
                if self.menu_state.is_none() {
                    if let Some(cell) = self
                        .layout()
                        .cell_at(self.last_cursor_pos, self.notification.is_none())
                    {
                        if cell.kind().right_click(
                            self.connection
                                .as_ref()
                                .is_none_or(|connection| connection.can_change_state()),
                            self.keyboard_modifiers,
                            &mut self.model,
                        ) {
                            self.menu_state = Some(MenuState::default());
                        } else if let Some(ref connection) = self.connection {
                            if connection.can_change_state() {
                                let send_fut = connection.set_state(&self.model);
                                return Command::single(Action::Future(
                                    async move {
                                        match send_fut.await {
                                            Ok(()) => Message::Nop,
                                            Err(e) => Message::ConnectionError(e.into()),
                                        }
                                    }
                                    .boxed(),
                                ));
                            }
                        }
                    }
                }
            }
            Message::SetAutoUpdateCheck(enable) => {
                if let Some(config) = self.config.as_mut() {
                    config.auto_update_check = Some(enable);
                    return self.save_config();
                }
            }
            Message::SetItemFanfarePath(path) => {
                let config = self.config.as_mut().expect("config not yet loaded");
                config.item_fanfare_path = if path.is_empty() {
                    None
                } else {
                    Some(path.into())
                };
                return self.save_config();
            }
            Message::SetConnection(connection) => self.connection = Some(connection),
            Message::SetConnectionKind(kind) => {
                if let Some(MenuState {
                    ref mut connection_params,
                    ..
                }) = self.menu_state
                {
                    connection_params.set_kind(kind);
                }
            }
            Message::SetLayoutPreference(layout_preference) => {
                if let Some(config) = self.config.as_mut() {
                    config.layout_preference = layout_preference;
                    return self.save_config();
                }
            }
            Message::SetMedOrder(med_order) => {
                if let Some(config) = self.config.as_mut() {
                    config.med_order = med_order;
                    return self.save_config();
                }
            }
            Message::SetPasscode(new_passcode) => {
                if let Some(MenuState {
                    connection_params:
                        ConnectionParams::Web {
                            ref mut passcode, ..
                        },
                    ..
                }) = self.menu_state
                {
                    *passcode = new_passcode;
                }
            }
            Message::SetUrl(new_url) => {
                if let Some(MenuState {
                    connection_params: ConnectionParams::Web { ref mut url, .. },
                    ..
                }) = self.menu_state
                {
                    *url = new_url;
                }
            }
            Message::SetWarpSongOrder(warp_song_order) => {
                if let Some(config) = self.config.as_mut() {
                    config.warp_song_order = warp_song_order;
                    return self.save_config();
                }
            }
            Message::ToggleCheckPanel => {
                self.show_check_panel = !self.show_check_panel;
            }
            Message::UpdateCheck => {
                self.update_check = UpdateCheckState::Checking;
                let client = self.http_client.clone();
                return Command::single(Action::Future(
                    async move {
                        match check_for_updates(&client).await {
                            Ok(update_available) => Message::UpdateCheckComplete(update_available),
                            Err(e) => Message::UpdateCheckError(e),
                        }
                    }
                    .boxed(),
                ));
            }
            Message::UpdateCheckComplete(Some(new_ver)) => {
                self.update_check = UpdateCheckState::UpdateAvailable {
                    new_ver,
                    update_btn: button::State::default(),
                    reset_btn: button::State::default(),
                }
            }
            Message::UpdateCheckComplete(None) => {
                self.update_check = UpdateCheckState::NoUpdateAvailable
            }
            Message::UpdateCheckError(e) => {
                self.update_check = UpdateCheckState::Error {
                    e,
                    reset_btn: button::State::default(),
                }
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Message<ootr_static::Rando>> {
        let layout = self.layout();
        let (layout_width, _layout_height) = layout.pixel_dimensions();
        let mut cell_buttons = self.cell_buttons.iter_mut();

        macro_rules! cell {
            ($cell:expr) => {{
                $cell.id.view(
                    &self.model,
                    cell_buttons.next().expect("not enough cell button states"),
                )
            }};
        }

        if let Some(ref mut menu_state) = self.menu_state {
            return Column::new()
                .push(
                    Row::new()
                        .push(
                            Button::new(&mut menu_state.dismiss_btn, Text::new("Back"))
                                .on_press(Message::CloseMenu),
                        )
                        .push(Space::with_width(Length::Fill))
                        .push(self.update_check.view()),
                )
                .push(
                    Text::new("Preferences")
                        .size(24)
                        .width(Length::Fill)
                        .horizontal_alignment(alignment::Horizontal::Center),
                )
                .push(Text::new("Tracker layout:"))
                .push(PickList::new(
                    &mut menu_state.layout_preference,
                    all().collect_vec(),
                    self.config.as_ref().map(|cfg| cfg.layout_preference),
                    Message::SetLayoutPreference,
                ))
                .push(Text::new("Medallion order:"))
                .push(PickList::new(
                    &mut menu_state.med_order,
                    all().collect_vec(),
                    self.config.as_ref().map(|cfg| cfg.med_order),
                    Message::SetMedOrder,
                ))
                .push(Text::new("Warp song order:"))
                .push(PickList::new(
                    &mut menu_state.warp_song_order,
                    all().collect_vec(),
                    self.config.as_ref().map(|cfg| cfg.warp_song_order),
                    Message::SetWarpSongOrder,
                ))
                .push(Text::new("Item fanfare sound (MP3 path):"))
                .push(TextInput::new(
                    &mut menu_state.item_fanfare_path,
                    "Path to MP3 file",
                    self.config
                        .as_ref()
                        .and_then(|cfg| cfg.item_fanfare_path.as_ref())
                        .map(|p| p.to_string_lossy())
                        .as_deref()
                        .unwrap_or(""),
                    Message::SetItemFanfarePath,
                ))
                .push(
                    Text::new("Connect")
                        .size(24)
                        .width(Length::Fill)
                        .horizontal_alignment(alignment::Horizontal::Center),
                )
                //TODO replace connection options with "current connection" info when connected
                .push(PickList::new(
                    &mut menu_state.connection_kind,
                    all().collect_vec(),
                    Some(menu_state.connection_params.kind()),
                    Message::SetConnectionKind,
                ))
                .push(menu_state.connection_params.view())
                .push(
                    Button::new(
                        &mut menu_state.connect_btn,
                        Text::new(if self.connection.is_some() {
                            "Disconnect"
                        } else {
                            "Connect"
                        }),
                    )
                    .on_press(Message::Connect),
                )
                .padding(5)
                .into();
        }

        // Build rows dynamically based on layout
        let cells = layout.cells();

        // Group cells by y-position to form rows
        let mut y_positions: Vec<u16> = cells.iter().map(|c| c.pos[1]).collect();
        y_positions.sort_unstable();
        y_positions.dedup();

        // Build rows dynamically
        let mut view = Column::new();
        let mut cells_iter = cells.into_iter().peekable();

        for (row_idx, &y_pos) in y_positions.iter().enumerate() {
            // Collect cells for this row (same y position)
            let mut row_cells: Vec<CellLayout> = Vec::new();
            while cells_iter.peek().is_some_and(|c| c.pos[1] == y_pos) {
                row_cells.push(cells_iter.next().unwrap());
            }

            // Sort by x position
            row_cells.sort_by_key(|c| c.pos[0]);

            // Build the row
            let mut row = Row::new();
            for cell_layout in row_cells {
                row = row.push(cell!(cell_layout));
            }
            row = row.spacing(10);

            // For the normal tracker view (after first 2 rows), check if we should stop for notification
            if row_idx >= 5 && self.notification.is_some() {
                break;
            }

            view = view.push(row);
        }

        // Handle special UI elements (update check, welcome screen, etc.)
        let view = if let Some(ref config) = self.config {
            if let UpdateCheckState::UpdateAvailable {
                ref new_ver,
                ref mut update_btn,
                ref mut reset_btn,
            } = self.update_check
            {
                view.push(
                    Text::new(format!(
                        "OoT Tracker {} is available — you have {}",
                        new_ver,
                        env!("CARGO_PKG_VERSION")
                    ))
                    .color([1.0, 1.0, 1.0])
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center),
                )
                .push(
                    Row::new()
                        .push(
                            Button::new(update_btn, Text::new("Update"))
                                .on_press(Message::InstallUpdate),
                        )
                        .push(
                            Button::new(reset_btn, Text::new("Dismiss"))
                                .on_press(Message::ResetUpdateState),
                        )
                        .spacing(5),
                )
            } else if config.auto_update_check.is_some() {
                if let Some((is_temp, ref notification)) = self.notification {
                    let mut row = Row::new().push(
                        Text::new(format!("{}", notification))
                            .color([1.0, 1.0, 1.0])
                            .width(Length::Fill),
                    );
                    if !is_temp {
                        row = row.push(
                            Button::new(
                                &mut self.dismiss_notification_button,
                                Text::new("X").color([1.0, 0.0, 0.0]),
                            )
                            .on_press(Message::DismissNotification),
                        );
                    }
                    view.push(row.height(Length::Units(101)))
                } else {
                    view
                }
            } else {
                view.push(
                    Text::new("Check for updates on startup?")
                        .color([1.0, 1.0, 1.0])
                        .width(Length::Fill)
                        .horizontal_alignment(alignment::Horizontal::Center),
                )
                .push(
                    Row::new()
                        .push(
                            Button::new(&mut self.enable_update_checks_button, Text::new("Yes"))
                                .on_press(Message::SetAutoUpdateCheck(true)),
                        )
                        .push(
                            Button::new(&mut self.disable_update_checks_button, Text::new("No"))
                                .on_press(Message::SetAutoUpdateCheck(false)),
                        )
                        .spacing(5),
                )
            }
        } else {
            view.push(
                Text::new(
                    "Welcome to the OoT tracker!\nTo change settings, right-click a Medallion.",
                )
                .color([1.0, 1.0, 1.0])
                .width(Length::Fill)
                .horizontal_alignment(alignment::Horizontal::Center),
            )
            .push(
                Button::new(&mut self.dismiss_welcome_screen_button, Text::new("OK"))
                    .on_press(Message::DismissWelcomeScreen),
            )
        };

        // Use computed layout dimensions for container sizing
        // Float tracker at top with no wasted space above
        let items_view = Container::new(view.spacing(10).padding(5))
            .width(Length::Units(layout_width as u16))
            .height(Length::Shrink);

        if self.flags.show_logic_tracker {
            // Logic tracker shows as side panel (existing behavior)
            let items_container = Container::new(items_view)
                .width(Length::Units(layout_width as u16 + 2))
                .height(Length::Fill)
                .align_y(alignment::Vertical::Top)
                .style(ContainerStyle);
            Row::new()
                .push(items_container)
                .push(self.logic.view(&self.rando).map(Message::Logic))
                .width(Length::Fill)
                .into()
        } else if self.show_check_panel {
            // Check panel anchored below the tracker (vertical layout)
            let check_panel = Container::new(
                Text::new("Check Panel")
                    .color([1.0, 1.0, 1.0])
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .style(ContainerStyle);
            Container::new(
                Column::new()
                    .push(items_view)
                    .push(check_panel)
                    .width(Length::Fill),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .style(ContainerStyle)
            .into()
        } else {
            // Tracker only - float at top, fill remaining space with background
            Container::new(items_view)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_y(alignment::Vertical::Top)
                .style(ContainerStyle)
                .into()
        }
    }

    fn subscription(&self) -> iced::Subscription<Message<ootr_static::Rando>> {
        Subscription::batch(vec![
            iced_native::subscription::events_with(|event, status| match (event, status) {
                (
                    iced_native::Event::Keyboard(iced_native::keyboard::Event::ModifiersChanged(
                        modifiers,
                    )),
                    _,
                ) => Some(Message::KeyboardModifiers(modifiers)),
                // Ctrl+L to toggle check panel
                (
                    iced_native::Event::Keyboard(iced_native::keyboard::Event::KeyPressed {
                        key_code: iced_native::keyboard::KeyCode::L,
                        modifiers,
                    }),
                    _,
                ) if modifiers.control() => Some(Message::ToggleCheckPanel),
                (
                    iced_native::Event::Mouse(iced_native::mouse::Event::CursorMoved { position }),
                    _,
                ) => Some(Message::MouseMoved(position.into())),
                (
                    iced_native::Event::Mouse(iced_native::mouse::Event::ButtonReleased(
                        iced_native::mouse::Button::Right,
                    )),
                    iced_native::event::Status::Ignored,
                ) => Some(Message::RightClick),
                _ => None,
            }),
            Subscription::from_recipe(subscriptions::Subscription::new(
                self.connection
                    .clone()
                    .unwrap_or_else(|| Arc::new(net::NullConnection)),
            )),
        ])
    }
}

#[derive(Debug, From, FromArc, Clone)]
enum ConnectionError {
    ExtraPathSegments,
    #[from]
    Firebase(firebase::Error),
    MissingRoomName,
    #[from]
    Net(net::Error),
    #[from_arc]
    Reqwest(Arc<reqwest::Error>),
    UnsupportedHost(Option<url::Host<String>>),
    UnsupportedRoomKind(String),
    #[from]
    UrlParse(url::ParseError),
    #[from_arc]
    Write(Arc<async_proto::WriteError>),
}

impl fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectionError::ExtraPathSegments => write!(f, "too many path segments in URL"),
            ConnectionError::Firebase(e) => write!(f, "Firebase error: {}", e),
            ConnectionError::MissingRoomName => write!(f, "missing room name"),
            ConnectionError::Net(e) => e.fmt(f),
            ConnectionError::Reqwest(e) => {
                if let Some(url) = e.url() {
                    write!(f, "HTTP error at {}: {}", url, e)
                } else {
                    write!(f, "HTTP error: {}", e)
                }
            }
            ConnectionError::UnsupportedHost(Some(host)) => {
                write!(f, "the tracker at {} is not (yet) supported", host)
            }
            ConnectionError::UnsupportedHost(None) => {
                write!(f, "this kind of connection is not supported")
            }
            ConnectionError::UnsupportedRoomKind(kind) => {
                write!(f, "“{}” rooms are not (yet) supported", kind)
            }
            ConnectionError::UrlParse(e) => e.fmt(f),
            ConnectionError::Write(e) => e.fmt(f),
        }
    }
}

async fn connect(
    params: ConnectionParams,
    state: ModelState,
) -> Result<Arc<dyn Connection>, ConnectionError> {
    let connection = match params {
        ConnectionParams::TcpListener => Arc::new(net::TcpConnection) as Arc<dyn Connection>,
        ConnectionParams::Web { url, passcode, .. } => {
            let url = url.parse::<Url>()?;

            macro_rules! firebase_host {
                ($ty:ident) => {{
                    let mut path_segments = url.path_segments().into_iter().flatten().fuse();
                    let name = match (
                        path_segments.next(),
                        path_segments.next(),
                        path_segments.next(),
                    ) {
                        (None, _, _) => return Err(ConnectionError::MissingRoomName),
                        (Some(room_name), None, _) | (Some(_), Some(room_name), None) => {
                            room_name.to_owned()
                        }
                        (Some(_), Some(_), Some(_)) => {
                            return Err(ConnectionError::ExtraPathSegments)
                        }
                    };
                    let session = firebase::Session::new(firebase::$ty).await?;
                    Arc::new(net::FirebaseConnection::new(firebase::Room {
                        session,
                        name,
                        passcode,
                    })) as Arc<dyn Connection>
                }};
            }

            match url.host() {
                Some(url::Host::Domain("oot-tracker.web.app"))
                | Some(url::Host::Domain("oot-tracker.firebaseapp.com")) => {
                    firebase_host!(OldRestreamTracker)
                }
                Some(url::Host::Domain("ootr-tracker.web.app"))
                | Some(url::Host::Domain("ootr-tracker.firebaseapp.com")) => {
                    firebase_host!(RestreamTracker)
                }
                Some(url::Host::Domain("ootr-random-settings-tracker.web.app"))
                | Some(url::Host::Domain("ootr-random-settings-tracker.firebaseapp.com")) => {
                    firebase_host!(RslItemTracker)
                }
                //TODO support for rsl-settings-tracker.web.app
                Some(url::Host::Domain("oottracker.fenhl.net")) => {
                    let mut path_segments = url.path_segments().into_iter().flatten().fuse();
                    match path_segments.next() {
                        None => return Err(ConnectionError::MissingRoomName),
                        Some("room") => Arc::new(
                            net::WebConnection::new(
                                path_segments
                                    .next()
                                    .ok_or(ConnectionError::MissingRoomName)?,
                            )
                            .await?,
                        ),
                        Some("restream") => {
                            return Err(ConnectionError::UnsupportedRoomKind(
                                "restream".to_string(),
                            ))
                        } //TODO support for single-player restream room connections
                        Some(room_kind) => {
                            return Err(ConnectionError::UnsupportedRoomKind(room_kind.to_owned()))
                        }
                    }
                }
                host => {
                    return Err(ConnectionError::UnsupportedHost(
                        host.map(|host| host.to_owned()),
                    ))
                }
            }
        }
    };
    if connection.can_change_state() {
        connection.set_state(&state).await?;
    }
    Ok(connection)
}

#[derive(Debug)]
enum UpdateCheckState {
    Unknown(button::State),
    Checking,
    Error {
        e: UpdateCheckError,
        reset_btn: button::State,
    },
    UpdateAvailable {
        new_ver: Version,
        update_btn: button::State,
        reset_btn: button::State,
    },
    NoUpdateAvailable,
    Installing,
}

impl UpdateCheckState {
    fn view(&mut self) -> Element<'_, Message<ootr_static::Rando>> {
        match self {
            UpdateCheckState::Unknown(check_btn) => Row::new()
                .push(Text::new(concat!("version ", env!("CARGO_PKG_VERSION"))))
                .push(
                    Button::new(check_btn, Text::new("Check for Updates"))
                        .on_press(Message::UpdateCheck),
                )
                .into(),
            UpdateCheckState::Checking => Text::new(concat!(
                "version ",
                env!("CARGO_PKG_VERSION"),
                " — checking for updates…"
            ))
            .into(),
            UpdateCheckState::Error { e, reset_btn } => Row::new()
                .push(Text::new(format!("error checking for updates: {}", e)))
                .push(
                    Button::new(reset_btn, Text::new("Dismiss"))
                        .on_press(Message::ResetUpdateState),
                )
                .into(),
            UpdateCheckState::UpdateAvailable {
                new_ver,
                update_btn,
                ..
            } => Row::new()
                .push(Text::new(format!(
                    "{} is available — you have {}",
                    new_ver,
                    env!("CARGO_PKG_VERSION")
                )))
                .push(Button::new(update_btn, Text::new("Update")).on_press(Message::InstallUpdate))
                .into(),
            UpdateCheckState::NoUpdateAvailable => Text::new(concat!(
                "version ",
                env!("CARGO_PKG_VERSION"),
                " — up to date"
            ))
            .into(),
            UpdateCheckState::Installing => Text::new(concat!(
                "version ",
                env!("CARGO_PKG_VERSION"),
                " — Installing update…"
            ))
            .into(),
        }
    }
}

#[derive(Debug, Clone, From, FromArc)]
enum UpdateCheckError {
    #[from_arc]
    Io(Arc<io::Error>),
    #[cfg(target_os = "macos")]
    MissingAsset,
    NoReleases,
    #[from_arc]
    Reqwest(Arc<reqwest::Error>),
    #[from_arc]
    SemVer(Arc<semver::Error>),
    #[from]
    Ui(ui::Error),
}

impl fmt::Display for UpdateCheckError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UpdateCheckError::Io(e) => write!(f, "I/O error: {}", e),
            #[cfg(target_os = "macos")]
            UpdateCheckError::MissingAsset => {
                write!(f, "release does not have a download for this platform")
            }
            UpdateCheckError::NoReleases => write!(f, "there are no released versions"),
            UpdateCheckError::Reqwest(e) => {
                if let Some(url) = e.url() {
                    write!(f, "HTTP error at {}: {}", url, e)
                } else {
                    write!(f, "HTTP error: {}", e)
                }
            }
            UpdateCheckError::SemVer(e) => e.fmt(f),
            UpdateCheckError::Ui(e) => e.fmt(f),
        }
    }
}

async fn check_for_updates(client: &reqwest::Client) -> Result<Option<Version>, UpdateCheckError> {
    let repo = Repo::new("fenhl", "oottracker");
    if let Some(release) = repo.latest_release(client).await? {
        let new_ver = release.version()?;
        Ok(if new_ver > Version::parse(env!("CARGO_PKG_VERSION"))? {
            Some(new_ver)
        } else {
            None
        })
    } else {
        Err(UpdateCheckError::NoReleases)
    }
}

async fn run_updater(
    #[cfg_attr(windows, allow(unused))] client: &reqwest::Client,
) -> Result<Never, UpdateCheckError> {
    #[cfg(target_os = "macos")]
    {
        //TODO use Sparkle or similar on macOS for automation?
        let release = Repo::new("fenhl", "oottracker")
            .latest_release(&client)
            .await?
            .ok_or(UpdateCheckError::NoReleases)?;
        let (asset,) = release
            .assets
            .into_iter()
            .filter(|asset| asset.name.ends_with("-mac.dmg"))
            .collect_tuple()
            .ok_or(UpdateCheckError::MissingAsset)?;
        let response = client
            .get(asset.browser_download_url)
            .send()
            .await?
            .error_for_status()?;
        let project_dirs = dirs()?;
        let cache_dir = project_dirs.cache_dir();
        fs::create_dir_all(cache_dir).await?;
        let dmg_download_path = cache_dir.join(asset.name);
        {
            let mut data = response.bytes_stream();
            let mut dmg_file = File::create(&dmg_download_path).await?;
            while let Some(chunk) = data.try_next().await? {
                dmg_file.write_all(chunk.as_ref()).await?;
            }
        }
        sleep(Duration::from_secs(1)).await; // to make sure the download is closed
        std::process::Command::new("open")
            .arg(dmg_download_path)
            .spawn()?;
        std::process::exit(0)
    }
    #[cfg(target_os = "windows")]
    {
        let project_dirs = dirs()?;
        let cache_dir = project_dirs.cache_dir();
        fs::create_dir_all(cache_dir).await?;
        let updater_path = cache_dir.join("updater.exe");
        #[cfg(target_arch = "x86_64")]
        let updater_data =
            include_bytes!("../../../target/x86_64-pc-windows-msvc/release/oottracker-updater.exe");
        fs::write(&updater_path, updater_data).await?;
        let _ = std::process::Command::new(updater_path)
            .arg(env::current_exe()?)
            .spawn()?;
        std::process::exit(0)
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        let _ = client;
        unimplemented!("automatic updates are not supported on this platform")
    }
}

#[derive(Debug, Default, clap::Parser)]
#[clap(version)]
struct Args {
    #[clap(long = "logic")]
    show_logic_tracker: bool,
}

#[derive(Debug, From)]
enum Error {
    Iced(iced::Error),
    Icon(window::icon::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Iced(e) => e.fmt(f),
            Error::Icon(e) => write!(f, "failed to set app icon: {}", e),
        }
    }
}

#[wheel::main]
fn main(args: Args) -> Result<(), Error> {
    let icon = images::icon::<DynamicImage>().to_rgba8();
    State::run(Settings {
        window: window::Settings {
            size: (
                DEFAULT_WIDTH + if args.show_logic_tracker { 800 } else { 0 },
                DEFAULT_HEIGHT + if args.show_logic_tracker { 400 } else { 0 },
            ),
            min_size: Some((DEFAULT_WIDTH, DEFAULT_HEIGHT)),
            max_size: if args.show_logic_tracker {
                None
            } else {
                Some((DEFAULT_WIDTH, DEFAULT_HEIGHT))
            },
            resizable: args.show_logic_tracker,
            icon: Some(Icon::from_rgba(
                icon.as_flat_samples().as_slice().to_owned(),
                icon.width(),
                icon.height(),
            )?),
            ..window::Settings::default()
        },
        ..Settings::with_flags(args)
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==========================================================================
    // ConnectionKind tests
    // ==========================================================================

    #[test]
    fn test_connection_kind_default_is_tcp_listener() {
        assert_eq!(ConnectionKind::default(), ConnectionKind::TcpListener);
    }

    #[test]
    fn test_connection_kind_display_tcp_listener() {
        assert_eq!(ConnectionKind::TcpListener.to_string(), "Project64");
    }

    #[test]
    fn test_connection_kind_display_web() {
        assert_eq!(ConnectionKind::Web.to_string(), "web");
    }

    #[test]
    fn test_connection_kind_equality() {
        assert_eq!(ConnectionKind::TcpListener, ConnectionKind::TcpListener);
        assert_eq!(ConnectionKind::Web, ConnectionKind::Web);
        assert_ne!(ConnectionKind::TcpListener, ConnectionKind::Web);
    }

    // ==========================================================================
    // ConnectionParams tests
    // ==========================================================================

    #[test]
    fn test_connection_params_default_is_tcp_listener() {
        let params = ConnectionParams::default();
        assert_eq!(params.kind(), ConnectionKind::TcpListener);
    }

    #[test]
    fn test_connection_params_tcp_listener_kind() {
        let params = ConnectionParams::TcpListener;
        assert_eq!(params.kind(), ConnectionKind::TcpListener);
    }

    #[test]
    fn test_connection_params_web_kind() {
        let params = ConnectionParams::Web {
            url: String::new(),
            url_state: text_input::State::default(),
            passcode: String::new(),
            passcode_state: text_input::State::default(),
        };
        assert_eq!(params.kind(), ConnectionKind::Web);
    }

    #[test]
    fn test_connection_params_set_kind_tcp_to_web() {
        let mut params = ConnectionParams::default();
        params.set_kind(ConnectionKind::Web);
        assert_eq!(params.kind(), ConnectionKind::Web);
        if let ConnectionParams::Web { url, passcode, .. } = params {
            assert!(url.is_empty());
            assert!(passcode.is_empty());
        } else {
            panic!("Expected Web variant");
        }
    }

    #[test]
    fn test_connection_params_set_kind_web_to_tcp() {
        let mut params = ConnectionParams::Web {
            url: "http://example.com".to_string(),
            url_state: text_input::State::default(),
            passcode: "secret".to_string(),
            passcode_state: text_input::State::default(),
        };
        params.set_kind(ConnectionKind::TcpListener);
        assert_eq!(params.kind(), ConnectionKind::TcpListener);
    }

    #[test]
    fn test_connection_params_set_kind_web_same_noop() {
        let mut params = ConnectionParams::Web {
            url: "http://preserved.com".to_string(),
            url_state: text_input::State::default(),
            passcode: "preserved".to_string(),
            passcode_state: text_input::State::default(),
        };
        params.set_kind(ConnectionKind::Web);
        if let ConnectionParams::Web { url, passcode, .. } = params {
            assert_eq!(url, "http://preserved.com");
            assert_eq!(passcode, "preserved");
        } else {
            panic!("Expected Web variant");
        }
    }

    // ==========================================================================
    // TrackerLayoutExt cell_at tests
    // ==========================================================================

    #[test]
    fn test_cell_at_outside_bounds_returns_none() {
        let layout = TrackerLayout::default();
        // Position far outside any cell
        let result = layout.cell_at([-100.0, -100.0], true);
        assert!(result.is_none());
    }

    #[test]
    fn test_cell_at_excludes_songs_when_flag_false() {
        let layout = TrackerLayout::default();
        // Get the layout height to find a position in the songs area (bottom of layout)
        let (_, height) = layout.pixel_dimensions();
        // Position near the bottom of the layout (in the songs area)
        let result = layout.cell_at([10.0, height as f32 - 10.0], false);
        assert!(result.is_none());
    }

    // ==========================================================================
    // Message Display tests
    // ==========================================================================

    #[test]
    fn test_message_display_client_disconnected() {
        let msg: Message<ootr_static::Rando> = Message::ClientDisconnected;
        assert_eq!(msg.to_string(), "connection lost");
    }

    #[test]
    fn test_message_display_connection_error_net() {
        let msg: Message<ootr_static::Rando> =
            Message::ConnectionError(ConnectionError::MissingRoomName);
        assert_eq!(msg.to_string(), "connection error: missing room name");
    }

    #[test]
    fn test_message_display_connection_error_extra_path() {
        let msg: Message<ootr_static::Rando> =
            Message::ConnectionError(ConnectionError::ExtraPathSegments);
        assert_eq!(
            msg.to_string(),
            "connection error: too many path segments in URL"
        );
    }

    // ==========================================================================
    // ConnectionError Display tests
    // ==========================================================================

    #[test]
    fn test_connection_error_display_extra_path_segments() {
        let err = ConnectionError::ExtraPathSegments;
        assert_eq!(err.to_string(), "too many path segments in URL");
    }

    #[test]
    fn test_connection_error_display_missing_room_name() {
        let err = ConnectionError::MissingRoomName;
        assert_eq!(err.to_string(), "missing room name");
    }

    #[test]
    fn test_connection_error_display_unsupported_host_some() {
        let err = ConnectionError::UnsupportedHost(Some(url::Host::Domain("example.com".into())));
        assert_eq!(
            err.to_string(),
            "the tracker at example.com is not (yet) supported"
        );
    }

    #[test]
    fn test_connection_error_display_unsupported_host_none() {
        let err = ConnectionError::UnsupportedHost(None);
        assert_eq!(err.to_string(), "this kind of connection is not supported");
    }

    #[test]
    fn test_connection_error_display_unsupported_room_kind() {
        let err = ConnectionError::UnsupportedRoomKind("tournament".to_string());
        // The Display implementation uses smart quotes
        assert!(err
            .to_string()
            .contains("tournament")
            .then_some(())
            .is_some());
        assert!(err.to_string().contains("rooms are not (yet) supported"));
    }

    // ==========================================================================
    // UpdateCheckError Display tests
    // ==========================================================================

    #[test]
    fn test_update_check_error_display_no_releases() {
        let err = UpdateCheckError::NoReleases;
        assert_eq!(err.to_string(), "there are no released versions");
    }

    // ==========================================================================
    // Constants tests
    // ==========================================================================

    #[test]
    fn test_cell_size_constant() {
        assert_eq!(CELL_SIZE, 50);
    }

    #[test]
    fn test_stone_size_constant() {
        assert_eq!(STONE_SIZE, 30);
    }

    #[test]
    fn test_default_width_constant() {
        // DEFAULT_WIDTH = 6 columns * 60px = 360
        assert_eq!(DEFAULT_WIDTH, 360);
    }

    #[test]
    fn test_default_height_constant() {
        // DEFAULT_HEIGHT = medallion row + 7 cell rows = 448
        assert_eq!(DEFAULT_HEIGHT, 448);
    }

    // ==========================================================================
    // Layout dimension tests
    // ==========================================================================

    #[test]
    fn test_default_layout_pixel_dimensions() {
        let layout = TrackerLayout::default();
        let (width, height) = layout.pixel_dimensions();
        assert_eq!(width, DEFAULT_WIDTH);
        // Height varies based on layout content
        assert!(height > 0);
    }

    #[test]
    fn test_layout_column_count() {
        let layout = TrackerLayout::default();
        // Default OoT layout has 6 columns
        assert_eq!(layout.column_count(), 6);
    }

    #[test]
    fn test_layout_row_count() {
        let layout = TrackerLayout::default();
        // Default layout has multiple rows
        assert!(layout.row_count() > 0);
    }
}
