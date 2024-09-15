use crate::util::{extract_root_table, from_value, Val};
use config::{FileStoredFormat, Format, Map, Value};
use std::error::Error;

#[derive(Debug)]
pub struct Hjson;

impl Format for Hjson {
    fn parse(&self, uri: Option<&String>, text: &str) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>> {
        let hjson: Val = serde_hjson::from_str(text).map_err(|err| Box::new(err) as Box<dyn Error + Send + Sync>)?;

        extract_root_table(from_value(uri, hjson))
    }
}

static EXTENSIONS: [&str; 1] = ["hjson"];

impl FileStoredFormat for Hjson {
    fn file_extensions(&self) -> &'static [&'static str] {
        &EXTENSIONS
    }
}
