use crate::util::{extract_root_table, from_value, Val};
use config::{FileStoredFormat, Format, Map, Value};
use std::error::Error;

#[derive(Debug)]
pub struct Properties;

impl Format for Properties {
    fn parse(&self, uri: Option<&String>, text: &str) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>> {
        let properties: Val =
            serde_java_properties::from_str(text).map_err(|err| Box::new(err) as Box<dyn Error + Send + Sync>)?;

        extract_root_table(from_value(uri, properties))
    }
}

static EXTENSIONS: [&str; 1] = ["properties"];

impl FileStoredFormat for Properties {
    fn file_extensions(&self) -> &'static [&'static str] {
        &EXTENSIONS
    }
}
