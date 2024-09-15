use crate::util::{extract_root_table, from_value, Val};
use config::{FileStoredFormat, Format, Map, Value};
use std::error::Error;

#[derive(Debug)]
pub struct Ason;

impl Format for Ason {
    fn parse(&self, uri: Option<&String>, text: &str) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>> {
        let hjson: Val = ason::from_str(text).map_err(|err| Box::new(err) as Box<dyn Error + Send + Sync>)?;

        extract_root_table(from_value(uri, hjson))
    }
}

static EXTENSIONS: [&str; 1] = ["ason"];

impl FileStoredFormat for Ason {
    fn file_extensions(&self) -> &'static [&'static str] {
        &EXTENSIONS
    }
}
