/*!
Add more formats to [`config`](https://crates.io/crates/config).

This crate provides additional formats for figment.
If you need additional formats, you can include this crate and use the provided formats.

Additionally, this adds a function to parse a file by its extension.

Supported formats added by this crate:

| Format          | Feature      | Crate                                                                   | Description                                                                      |
|-----------------|--------------|-------------------------------------------------------------------------|----------------------------------------------------------------------------------|
| YAML-NG         | `yaml_ng`    | [serde-yaml-ng](https://crates.io/crates/serde_yaml_ng)                 | An actively maintained fork of [serde_yaml](https://crates.io/crates/serde_yaml) |
| JAVA Properties | `properties` | [serde-java-properties](https://crates.io/crates/serde_java_properties) | Java properties file format                                                      |
| HJSON           | `hjson`      | [serde-hjson](https://crates.io/crates/serde_hjson)                     | [Human JSON](https://hjson.github.io/)                                           |
| HCL             | `hcl`        | [hcl-rs](https://crates.io/crates/hcl)                                  | HashiCorp Configuration Language                                                 |
| Ason            | `ason`       | [ason](https://crates.io/crates/ason)                                   | Ason format                                                                      |
| JSON            | `json`       | [serde_json](https://crates.io/crates/serde_json)                       | config supplied JSON format                                                     |
| JSON5           | `json5`      | [serde-json5](https://crates.io/crates/serde_json5)                     | config supplied JSON5 format                                                    |
| RON             | `ron`        | [ron](https://crates.io/crates/ron)                                     | config supplied RON format                                                      |
| TOML            | `toml`       | [toml](https://crates.io/crates/toml)                                   | config supplied TOML format                                                     |
| YAML            | `yaml`       | [yaml-rust](https://crates.io/crates/yaml-rust)                         | config supplied YAML format (using deprecated `serde_yaml`)                     |
| INI             | `ini`        | [ini](https://crates.io/crates/ini)                                     | config supplied INI format                                                      |
|                 | `all`        |                                                                         | Enable all formats except for `yaml`                                             |

If you do not enable yaml, yaml_ng will be used for yaml files instead.
Instead of `all`, enable only the formats you need to reduce compile times and dependencies.

The current development version of config already uses [`yaml-rust2`](https://crates.io/crates/yaml-rust2)
which is a fork of `yaml-rust` and is actively maintained.
This crate uses `serde_yaml_ng` which is another actively maintained solution for YAML.

# Example of [`by_file_extension`]

```rust
use config::Config;use config_more_formats::by_file_extension;

# fn main() {
#     std::fs::File::create("settings.toml").unwrap();

    let settings = Config::builder()
        .add_source(by_file_extension("settings.toml").unwrap())
        .build()
        .unwrap();

#     std::fs::remove_file("settings.toml").unwrap();
# }
```
*/

#[cfg(feature = "hjson")]
mod hjson;

#[cfg(feature = "hjson")]
pub use hjson::Hjson;
use std::error::Error;
#[cfg(feature = "properties")]
mod properties;
#[cfg(feature = "properties")]
pub use properties::Properties;
mod util;

#[cfg(feature = "hcl")]
mod hcl;
#[cfg(feature = "hcl")]
pub use hcl::Hcl;
#[cfg(feature = "ason")]
mod ason;
#[cfg(feature = "ason")]
pub use ason::Ason;
#[cfg(feature = "yaml_ng")]
mod yaml_ng;
#[cfg(feature = "yaml_ng")]
pub use yaml_ng::YamlNg;

use config::{FileFormat, FileSourceFile, FileStoredFormat, Format, Map, Value};
use std::fmt::Display;
use std::path::Path;

/// Error type for format parsing.
///
/// This error type is used when parsing by file extension fails.
#[derive(Debug)]
pub enum FormatError {
    /// There was no file extension found.
    NoExtensionFound,
    /// The file format is not supported.
    UnsupportedFormat(String),
}

impl Display for FormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormatError::NoExtensionFound => write!(f, "No file extension found"),
            FormatError::UnsupportedFormat(ext) => write!(f, "Unsupported file format: {}", ext),
        }
    }
}

/// Parse a file by its extension.
///
/// This function will attempt to parse a file by its extension. If the extension is not supported,
/// it will return an error.
pub fn by_file_extension(file: &str) -> Result<config::File<FileSourceFile, FormatWrapper>, FormatError> {
    let ext = Path::new(file);
    let ext = ext.extension();
    let ext = ext.ok_or(FormatError::NoExtensionFound)?;
    let ext = ext.to_string_lossy();
    let ext = ext.to_string();

    match ext.as_str() {
        // config native formats
        #[cfg(feature = "toml")]
        "toml" => Ok(config::File::new(file, config::FileFormat::Toml.into())),
        #[cfg(feature = "json")]
        "json" => Ok(config::File::new(file, config::FileFormat::Json.into())),
        #[cfg(feature = "yaml")]
        "yaml" | "yml" => Ok(config::File::new(file, config::FileFormat::Yaml.into())),
        #[cfg(feature = "ron")]
        "ron" => Ok(config::File::new(file, config::FileFormat::Ron.into())),
        #[cfg(feature = "json5")]
        "json5" => Ok(config::File::new(file, config::FileFormat::Json5.into())),
        #[cfg(feature = "ini")]
        "ini" => Ok(config::File::new(file, config::FileFormat::Ini.into())),

        // unit structs
        #[cfg(feature = "hjson")]
        "hjson" => Ok(config::File::new(file, FormatWrapper::Hjson)),
        #[cfg(feature = "properties")]
        "properties" => Ok(config::File::new(file, FormatWrapper::Properties)),
        #[cfg(feature = "yaml_ng")]
        "yaml_ng" => Ok(config::File::new(file, FormatWrapper::YamlNg)),
        #[cfg(feature = "hcl")]
        "hcl" => Ok(config::File::new(file, FormatWrapper::Hcl)),
        #[cfg(feature = "ason")]
        "ason" => Ok(config::File::new(file, FormatWrapper::Ason)),
        #[cfg(all(feature = "yaml_ng", not(feature = "yaml")))]
        "yaml" | "yml" => Ok(config::File::new(file, FormatWrapper::YamlNg)),

        _ => Err(FormatError::UnsupportedFormat(ext)),
    }
}

#[derive(Debug, Clone)]
pub enum FormatWrapper {
    Enum(FileFormat),
    #[cfg(feature = "ason")]
    Ason,
    #[cfg(feature = "hcl")]
    Hcl,
    #[cfg(feature = "hjson")]
    Hjson,
    #[cfg(feature = "properties")]
    Properties,
    #[cfg(feature = "yaml_ng")]
    YamlNg,
}

impl Format for FormatWrapper {
    fn parse(&self, uri: Option<&String>, text: &str) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>> {
        match self {
            FormatWrapper::Enum(f) => f.parse(uri, text),
            #[cfg(feature = "ason")]
            FormatWrapper::Ason => Ason.parse(uri, text),
            #[cfg(feature = "hcl")]
            FormatWrapper::Hcl => Hcl.parse(uri, text),
            #[cfg(feature = "hjson")]
            FormatWrapper::Hjson => Hjson.parse(uri, text),
            #[cfg(feature = "properties")]
            FormatWrapper::Properties => Properties.parse(uri, text),
            #[cfg(feature = "yaml_ng")]
            FormatWrapper::YamlNg => YamlNg.parse(uri, text),
        }
    }
}

impl FileStoredFormat for FormatWrapper {
    fn file_extensions(&self) -> &'static [&'static str] {
        match self {
            FormatWrapper::Enum(f) => f.file_extensions(),
            #[cfg(feature = "ason")]
            FormatWrapper::Ason => Ason.file_extensions(),
            #[cfg(feature = "hcl")]
            FormatWrapper::Hcl => Hcl.file_extensions(),
            #[cfg(feature = "hjson")]
            FormatWrapper::Hjson => Hjson.file_extensions(),
            #[cfg(feature = "properties")]
            FormatWrapper::Properties => Properties.file_extensions(),
            #[cfg(feature = "yaml_ng")]
            FormatWrapper::YamlNg => YamlNg.file_extensions(),
        }
    }
}

impl From<FileFormat> for FormatWrapper {
    fn from(f: FileFormat) -> Self {
        FormatWrapper::Enum(f)
    }
}

#[cfg(feature = "ason")]
impl From<Ason> for FormatWrapper {
    fn from(_: Ason) -> Self {
        FormatWrapper::Ason
    }
}

#[cfg(feature = "hcl")]
impl From<Hcl> for FormatWrapper {
    fn from(_: Hcl) -> Self {
        FormatWrapper::Hcl
    }
}

#[cfg(feature = "hjson")]
impl From<Hjson> for FormatWrapper {
    fn from(_: Hjson) -> Self {
        FormatWrapper::Hjson
    }
}

#[cfg(feature = "properties")]
impl From<Properties> for FormatWrapper {
    fn from(_: Properties) -> Self {
        FormatWrapper::Properties
    }
}

#[cfg(feature = "yaml_ng")]
impl From<YamlNg> for FormatWrapper {
    fn from(_: YamlNg) -> Self {
        FormatWrapper::YamlNg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_by_file_extension_toml() {
        #[cfg(feature = "toml")]
        {
            let result = by_file_extension("settings.toml");
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_by_file_extension_json() {
        #[cfg(feature = "json")]
        {
            let result = by_file_extension("settings.json");
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_by_file_extension_yaml() {
        #[cfg(feature = "yaml")]
        {
            let result = by_file_extension("settings.yaml");
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_by_file_extension_unsupported() {
        let result = by_file_extension("settings.unsupported");
        assert!(matches!(result, Err(FormatError::UnsupportedFormat(_))));
    }

    #[test]
    fn test_by_file_extension_no_extension() {
        let result = by_file_extension("settings");
        assert!(matches!(result, Err(FormatError::NoExtensionFound)));
    }

    #[test]
    fn test_by_file_extension_properties() {
        #[cfg(feature = "properties")]
        {
            let result = by_file_extension("settings.properties");
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_by_file_extension_hjson() {
        #[cfg(feature = "hjson")]
        {
            let result = by_file_extension("settings.hjson");
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_by_file_extension_hcl() {
        #[cfg(feature = "hcl")]
        {
            let result = by_file_extension("settings.hcl");
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_by_file_extension_ason() {
        #[cfg(feature = "ason")]
        {
            let result = by_file_extension("settings.ason");
            assert!(result.is_ok());
        }
    }
}
