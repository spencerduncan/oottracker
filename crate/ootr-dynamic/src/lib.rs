#![deny(
    rust_2018_idioms,
    unused,
    unused_crate_dependencies,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    warnings
)]
#![forbid(unsafe_code)]

use {
    crate::region::{parse_dungeon_info, RawRegion},
    itertools::Itertools as _,
    ootr::{item::Item, region::Region, Regions},
    pyo3::prelude::*,
    semver::Version,
    serde::de::DeserializeOwned,
    std::{
        cell::RefCell,
        collections::{HashMap, HashSet},
        fmt,
        fs::{self, File},
        io::{self, BufRead, BufReader},
        path::{Path, PathBuf},
        sync::Arc,
    },
    wheel::FromArc,
};

mod region;

pub struct Rando<'p> {
    py: Python<'p>,
    path: PathBuf,
    escaped_items: RefCell<Option<Arc<HashMap<String, Item>>>>,
    item_table: RefCell<Option<Arc<HashMap<String, Item>>>>,
    logic_tricks: RefCell<Option<Arc<HashSet<String>>>>,
    regions: RefCell<Option<Regions<Self>>>, //TODO glitched support
    setting_infos: RefCell<Option<Arc<HashSet<String>>>>,
}

impl<'p> Rando<'p> {
    pub fn new(py: Python<'p>, path: impl AsRef<Path>) -> Rando<'p> {
        Rando {
            py,
            path: path.as_ref().to_owned(),
            escaped_items: RefCell::default(),
            item_table: RefCell::default(),
            logic_tricks: RefCell::default(),
            regions: RefCell::default(),
            setting_infos: RefCell::default(),
        }
    }

    /// Imports and returns the given Python module from the randomizer codebase.
    fn import(&self, module: &str) -> PyResult<&'p PyModule> {
        let sys = self.py.import("sys")?;
        sys.getattr("path")?
            .call_method1("append", (self.path.display().to_string(),))?;
        self.py.import(module)
    }
}

impl<'p> fmt::Debug for Rando<'p> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //TODO f.debug_struct("Rando").field("path", path).finish_non_exhaustive() (https://github.com/rust-lang/rust/issues/67364)
        write!(f, "Rando {{ path: ")?;
        self.path.fmt(f)?;
        write!(f, ", .. }}")
    }
}

#[derive(Debug, FromArc, Clone)]
pub enum RandoErr {
    #[from_arc]
    Io(Arc<io::Error>),
    InvalidLogicHelper,
    ItemNotFound,
    NonJsonRegionFile(String),
    NonUnicodeRegionFilename,
    #[from_arc]
    Py(Arc<PyErr>),
    UnknownRegionFilename(String),
}

impl fmt::Display for RandoErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RandoErr::Io(e) => write!(f, "I/O error: {}", e),
            RandoErr::InvalidLogicHelper => write!(f, "multiple ( found in logic helper"),
            RandoErr::ItemNotFound => write!(f, "no such item"),
            RandoErr::NonJsonRegionFile(name) => write!(
                f,
                "expected region filename ending in .json but found {}",
                name
            ),
            RandoErr::NonUnicodeRegionFilename => write!(f, "non-Unicode region filename"),
            RandoErr::Py(e) => write!(f, "Python error: {}", e),
            RandoErr::UnknownRegionFilename(name) => {
                write!(f, "unexpected region filename: {}", name)
            }
        }
    }
}

impl ootr::RandoErr for RandoErr {
    const ITEM_NOT_FOUND: RandoErr = RandoErr::ItemNotFound;
}

impl<'p> ootr::Rando for Rando<'p> {
    type Err = RandoErr;
    type RegionName = String;

    fn escaped_items(&self) -> Result<Arc<HashMap<String, Item>>, RandoErr> {
        if self.escaped_items.borrow().is_none() {
            let items = self
                .import("RuleParser")?
                .getattr("escaped_items")?
                .call_method0("items")?
                .iter()?
                .map(|elt| {
                    elt.and_then(|elt| elt.extract())
                        .and_then(|(esc_name, item_name)| {
                            Ok((
                                esc_name,
                                item_name,
                                self.import("ItemList")?
                                    .getattr("item_table")?
                                    .get_item(item_name)?
                                    .get_item(0)?
                                    .extract::<&str>()?,
                            ))
                        })
                })
                .filter_map(|elt| match elt {
                    Ok((esc_name, item_name, kind)) => {
                        if kind == "Event" && item_name != "Scarecrow Song" {
                            //HACK treat Scarecrow Song as not an event since it's not defined as one in any region
                            None
                        } else {
                            match Item::from_str(self, item_name) {
                                Ok(item) => Some(Ok((esc_name, item))),
                                Err(e) => Some(Err(e)),
                            }
                        }
                    }
                    Err(e) => Some(Err(e.into())),
                })
                .try_collect()?;
            *self.escaped_items.borrow_mut() = Some(Arc::new(items));
        }
        Ok(Arc::clone(
            self.escaped_items.borrow().as_ref().expect("just inserted"),
        ))
    }

    fn item_table(&self) -> Result<Arc<HashMap<String, Item>>, RandoErr> {
        if self.item_table.borrow().is_none() {
            let items = self
                .import("ItemList")?
                .getattr("item_table")?
                .call_method0("items")?
                .iter()?
                .map(|elt| {
                    let (name, (kind, _, _, _)) =
                        elt?.extract::<(String, (String, &PyAny, &PyAny, &PyAny))>()?;
                    PyResult::Ok((name, kind))
                })
                .try_collect::<_, Vec<_>, _>()?
                .into_iter()
                .filter_map(|(name, kind)| {
                    if kind != "Event" || name == "Scarecrow Song" {
                        //HACK treat Scarecrow Song as not an event since it's not defined as one in any region
                        Some((name.clone(), Item(name)))
                    } else {
                        None
                    }
                })
                .collect();
            *self.item_table.borrow_mut() = Some(Arc::new(items));
        }
        Ok(Arc::clone(
            self.item_table.borrow().as_ref().expect("just inserted"),
        ))
    }

    fn logic_tricks(&self) -> Result<Arc<HashSet<String>>, RandoErr> {
        if self.logic_tricks.borrow().is_none() {
            let mut tricks = HashSet::default();
            for trick in self
                .import("SettingsList")?
                .getattr("logic_tricks")?
                .call_method0("values")?
                .iter()?
            {
                tricks.insert(trick?.get_item("name")?.extract()?);
            }
            *self.logic_tricks.borrow_mut() = Some(Arc::new(tricks));
        }
        Ok(Arc::clone(
            self.logic_tricks.borrow().as_ref().expect("just inserted"),
        ))
    }

    fn regions(&self) -> Result<Regions<Self>, RandoErr> {
        if self.regions.borrow().is_none() {
            let world_path = self.path.join("data").join("World"); //TODO glitched support
            let mut regions = Vec::default();
            for region_path in fs::read_dir(world_path)? {
                let region_path = region_path?;
                let filename = region_path.file_name();
                let filename = filename
                    .to_str()
                    .ok_or(RandoErr::NonUnicodeRegionFilename)?;
                let dungeon = parse_dungeon_info(
                    filename
                        .strip_suffix(".json")
                        .ok_or_else(|| RandoErr::NonJsonRegionFile(filename.to_owned()))?,
                )?;
                let region_file = File::open(region_path.path())?;
                for raw_region in
                    read_json_lenient_sync::<_, Vec<RawRegion>>(BufReader::new(region_file))?
                {
                    let name = raw_region.region_name.clone();
                    //assert_eq!(dungeon.map(|(dungeon, _)| dungeon.to_string().replace('\'', "")), raw_region.dungeon);
                    regions.push(Arc::new(Region {
                        dungeon,
                        scene: raw_region.scene,
                        hint: raw_region.hint,
                        time_passes: raw_region.time_passes,
                        events: raw_region.events.into_keys().collect(),
                        locations: raw_region.locations.into_keys().collect(),
                        exits: raw_region.exits.into_keys().collect(),
                        name,
                    }));
                }
            }
            *self.regions.borrow_mut() = Some(Arc::new(regions));
        }
        Ok(Arc::clone(
            self.regions.borrow().as_ref().expect("just inserted"),
        ))
    }

    fn root() -> String {
        "Root".to_owned()
    }

    fn setting_infos(&self) -> Result<Arc<HashSet<String>>, RandoErr> {
        if self.setting_infos.borrow().is_none() {
            let mut settings = HashSet::default();
            // setting_infos is a dict where keys are setting names, so iterate directly
            for setting_name in self
                .import("SettingsList")?
                .getattr("SettingInfos")?
                .getattr("setting_infos")?
                .iter()?
            {
                settings.insert(setting_name?.extract()?);
            }
            *self.setting_infos.borrow_mut() = Some(Arc::new(settings));
        }
        Ok(Arc::clone(
            self.setting_infos.borrow().as_ref().expect("just inserted"),
        ))
    }
}

fn read_json_lenient_sync<R: BufRead, T: DeserializeOwned>(mut reader: R) -> io::Result<T> {
    let mut buf = String::default();
    let mut line_buf = String::default();
    while reader.read_line(&mut line_buf)? > 0 {
        buf.push_str(
            &line_buf
                .split('#')
                .next()
                .expect("split always yields at least one element")
                .replace("\r", "")
                .replace('\n', " "),
        );
        line_buf.clear();
    }
    Ok(serde_json::from_str(&buf)?)
}

pub fn version() -> Version {
    Version::parse(env!("CARGO_PKG_VERSION")).expect("failed to parse current version")
}

#[test]
#[ignore] // Requires Python and randomizer to be set up at a specific path
fn load_rando_data() -> Result<(), RandoErr> {
    use ootr::Rando as _;

    Python::with_gil(|py| {
        let rando = Rando::new(
            py,
            "C:\\Users\\fenhl\\AppData\\Local\\Fenhl\\RSL\\cache\\ootr-latest",
        );
        rando.escaped_items()?;
        rando.item_table()?;
        rando.logic_tricks()?;
        rando.regions()?;
        rando.setting_infos()?;
        Ok(())
    })
}

#[cfg(test)]
mod json_parsing_tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_parse_simple_json_object() {
        let input = r#"{"key": "value"}"#;
        let reader = Cursor::new(input);
        let result: HashMap<String, String> = read_json_lenient_sync(reader).unwrap();
        assert_eq!(result.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_parse_json_array() {
        let input = r#"[1, 2, 3]"#;
        let reader = Cursor::new(input);
        let result: Vec<i32> = read_json_lenient_sync(reader).unwrap();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_strip_single_line_comment() {
        // Comment after JSON content should be stripped
        let input = r#"{"key": "value"} # this is a comment"#;
        let reader = Cursor::new(input);
        let result: HashMap<String, String> = read_json_lenient_sync(reader).unwrap();
        assert_eq!(result.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_strip_comment_on_separate_line() {
        let input = "# This is a comment line\n{\"key\": \"value\"}";
        let reader = Cursor::new(input);
        let result: HashMap<String, String> = read_json_lenient_sync(reader).unwrap();
        assert_eq!(result.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_multiline_json_with_comments() {
        let input = r#"# Header comment
{
    "name": "test", # inline comment
    "value": 42
}
# Footer comment"#;
        let reader = Cursor::new(input);
        let result: serde_json::Value = read_json_lenient_sync(reader).unwrap();
        assert_eq!(result["name"], "test");
        assert_eq!(result["value"], 42);
    }

    #[test]
    fn test_handle_crlf_line_endings() {
        // Windows-style line endings should be handled
        let input = "{\"key\": \"value\"}\r\n";
        let reader = Cursor::new(input);
        let result: HashMap<String, String> = read_json_lenient_sync(reader).unwrap();
        assert_eq!(result.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_handle_mixed_line_endings() {
        let input = "{\r\n\"a\": 1,\n\"b\": 2\r\n}";
        let reader = Cursor::new(input);
        let result: HashMap<String, i32> = read_json_lenient_sync(reader).unwrap();
        assert_eq!(result.get("a"), Some(&1));
        assert_eq!(result.get("b"), Some(&2));
    }

    #[test]
    fn test_invalid_json_returns_error() {
        let input = "not valid json";
        let reader = Cursor::new(input);
        let result: io::Result<serde_json::Value> = read_json_lenient_sync(reader);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_input_returns_error() {
        let input = "";
        let reader = Cursor::new(input);
        let result: io::Result<serde_json::Value> = read_json_lenient_sync(reader);
        assert!(result.is_err());
    }

    #[test]
    fn test_comment_only_input_returns_error() {
        let input = "# just a comment\n# another comment";
        let reader = Cursor::new(input);
        let result: io::Result<serde_json::Value> = read_json_lenient_sync(reader);
        assert!(result.is_err());
    }

    #[test]
    fn test_hash_inside_string_preserved() {
        // Hash characters inside JSON strings should NOT be treated as comments
        // Note: This tests the current behavior - the function strips after #
        // even inside strings, which may be intentional for the OoTR format
        let input = r#"{"key": "value"}"#;
        let reader = Cursor::new(input);
        let result: HashMap<String, String> = read_json_lenient_sync(reader).unwrap();
        assert_eq!(result.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_nested_json_structure() {
        let input = r#"{
            "outer": {
                "inner": [1, 2, 3]
            }
        }"#;
        let reader = Cursor::new(input);
        let result: serde_json::Value = read_json_lenient_sync(reader).unwrap();
        assert_eq!(result["outer"]["inner"][0], 1);
        assert_eq!(result["outer"]["inner"][2], 3);
    }

    #[test]
    fn test_json_with_unicode() {
        let input = r#"{"greeting": "こんにちは"}"#;
        let reader = Cursor::new(input);
        let result: HashMap<String, String> = read_json_lenient_sync(reader).unwrap();
        assert_eq!(result.get("greeting"), Some(&"こんにちは".to_string()));
    }

    #[test]
    fn test_json_with_escaped_characters() {
        let input = r#"{"path": "C:\\Users\\test"}"#;
        let reader = Cursor::new(input);
        let result: HashMap<String, String> = read_json_lenient_sync(reader).unwrap();
        assert_eq!(result.get("path"), Some(&"C:\\Users\\test".to_string()));
    }

    #[test]
    fn test_json_boolean_and_null_values() {
        let input = r#"{"active": true, "deleted": false, "data": null}"#;
        let reader = Cursor::new(input);
        let result: serde_json::Value = read_json_lenient_sync(reader).unwrap();
        assert_eq!(result["active"], true);
        assert_eq!(result["deleted"], false);
        assert!(result["data"].is_null());
    }

    #[test]
    fn test_json_numeric_types() {
        // Test various numeric types: integers, floats, negative numbers
        let input = r#"{"int": 42, "negative": -17, "float": 3.14, "exp": 1.5e10}"#;
        let reader = Cursor::new(input);
        let result: serde_json::Value = read_json_lenient_sync(reader).unwrap();
        assert_eq!(result["int"], 42);
        assert_eq!(result["negative"], -17);
        assert_eq!(result["float"], 3.14);
        assert_eq!(result["exp"], 1.5e10);
    }

    #[test]
    fn test_empty_json_structures() {
        // Test empty object
        let input = "{}";
        let reader = Cursor::new(input);
        let result: serde_json::Value = read_json_lenient_sync(reader).unwrap();
        assert!(result.is_object());
        assert_eq!(result.as_object().unwrap().len(), 0);

        // Test empty array
        let input = "[]";
        let reader = Cursor::new(input);
        let result: serde_json::Value = read_json_lenient_sync(reader).unwrap();
        assert!(result.is_array());
        assert_eq!(result.as_array().unwrap().len(), 0);
    }

    #[test]
    fn test_multiple_consecutive_comment_lines() {
        let input = r#"# comment 1
# comment 2
# comment 3
{"key": "value"}
# trailing comment"#;
        let reader = Cursor::new(input);
        let result: HashMap<String, String> = read_json_lenient_sync(reader).unwrap();
        assert_eq!(result.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_whitespace_only_lines() {
        let input = "   \n\t\n{\"key\": \"value\"}\n   \n";
        let reader = Cursor::new(input);
        let result: HashMap<String, String> = read_json_lenient_sync(reader).unwrap();
        assert_eq!(result.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_large_array_parsing() {
        // Test parsing a larger array structure
        let input = "[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]";
        let reader = Cursor::new(input);
        let result: Vec<i32> = read_json_lenient_sync(reader).unwrap();
        assert_eq!(result.len(), 20);
        assert_eq!(result[0], 1);
        assert_eq!(result[19], 20);
    }
}
