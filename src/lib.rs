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
| YAML            | `yaml`       | [serde_yaml](https://crates.io/crates/serde_yaml)                       | config supplied YAML format (using deprecated `serde_yaml`)                     |
| INI             | `ini`        | [ini](https://crates.io/crates/ini)                                     | config supplied INI format                                                      |
|                 | `all`        |                                                                         | Enable all formats except for `yaml`                                             |

If you do not enable yaml, yaml_ng will be used for yaml files instead.
Instead of `all`, enable only the formats you need to reduce compile times and dependencies.

# Example of [`by_file_extension`]

```rust
use config::Config;use config_more_formats::by_file_extension;

fn main() {
    let settings = Config::builder()
        .add_source(by_file_extension("settings.toml").unwrap())
        .build()
        .unwrap();
}
```
*/

#[cfg(feature = "hjson")]
mod hjson;
#[cfg(feature = "hjson")]
pub use hjson::Hjson;
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

use config::{FileSourceFile, FileStoredFormat};
use std::fmt::Display;
use std::path::Path;

/// Error type for format parsing.
///
/// This error type is used when parsing by file extension fails.
#[derive(Debug)]
pub enum FormatError {
    /// There was an error parsing the file extension.
    UnsupportedExtension,
    /// There was no file extension found.
    NoExtensionFound,
    /// The file format is not supported.
    UnsupportedFormat(String),
}

impl Display for FormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormatError::UnsupportedExtension => write!(f, "Unsupported file extension"),
            FormatError::NoExtensionFound => write!(f, "No file extension found"),
            FormatError::UnsupportedFormat(ext) => write!(f, "Unsupported file format: {}", ext),
        }
    }
}

/// Parse a file by its extension.
///
/// This function will attempt to parse a file by its extension. If the extension is not supported,
/// it will return an error.
///
/// # Example
///
/// ```rust
/// ```
pub fn by_file_extension<F: FileStoredFormat>(file: &str) -> Result<config::File<FileSourceFile, F>, FormatError> {
    let ext = Path::new(file)
        .extension()
        .ok_or(FormatError::UnsupportedExtension)?
        .to_string_lossy()
        .to_string();

    match ext.as_str() {
        // config native formats
        #[cfg(feature = "toml")]
        "toml" => Ok(config::File::new(file, config::FileFormat::Toml)),
        #[cfg(feature = "json")]
        "json" => Ok(config::File::new(file, config::FileFormat::Json)),
        #[cfg(feature = "yaml")]
        "yaml" | "yml" => Ok(config::File::new(file, config::FileFormat::Yaml)),
        #[cfg(feature = "ron")]
        "ron" => Ok(config::File::new(file, config::FileFormat::Ron)),
        #[cfg(feature = "json5")]
        "json5" => Ok(config::File::new(file, config::FileFormat::Json5)),
        #[cfg(feature = "ini")]
        "ini" => Ok(config::File::new(file, config::FileFormat::Ini)),

        #[cfg(feature = "hjson")]
        "hjson" => Ok(config::File::new(file, Hjson)),
        #[cfg(feature = "properties")]
        "properties" => Ok(config::File::new(file, Properties)),

        #[cfg(feature = "yaml_ng")]
        "yaml_ng" => Ok(config::File::new(file, YamlNg)),
        #[cfg(feature = "hcl")]
        "hcl" => Ok(config::File::new(file, Hcl)),
        #[cfg(feature = "ason")]
        "ason" => Ok(config::File::new(file, Ason)),
        #[cfg(all(feature = "yaml_ng", not(feature = "yaml")))]
        "yaml" | "yml" => Ok(config::File::new(file, YamlNg)),

        "" => Err(FormatError::NoExtensionFound),
        _ => Err(FormatError::UnsupportedFormat(ext)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_by_file_extension_toml() {
        #[cfg(feature = "toml")]
        {
            let result = by_file_extension::<config::FileFormat>("settings.toml");
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_by_file_extension_json() {
        #[cfg(feature = "json")]
        {
            let result = by_file_extension::<config::FileFormat>("settings.json");
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_by_file_extension_yaml() {
        #[cfg(feature = "yaml")]
        {
            let result = by_file_extension::<config::FileFormat>("settings.yaml");
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_by_file_extension_unsupported() {
        let result = by_file_extension::<config::FileFormat>("settings.unsupported");
        assert!(matches!(result, Err(FormatError::UnsupportedFormat(_))));
    }

    #[test]
    fn test_by_file_extension_no_extension() {
        let result = by_file_extension::<config::FileFormat>("settings");
        assert!(matches!(result, Err(FormatError::NoExtensionFound)));
    }

    #[test]
    fn test_by_file_extension_properties() {
        #[cfg(feature = "properties")]
        {
            let result = by_file_extension::<config::FileFormat>("settings.properties");
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_by_file_extension_hjson() {
        #[cfg(feature = "hjson")]
        {
            let result = by_file_extension::<config::FileFormat>("settings.hjson");
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_by_file_extension_hcl() {
        #[cfg(feature = "hcl")]
        {
            let result = by_file_extension::<config::FileFormat>("settings.hcl");
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_by_file_extension_ason() {
        #[cfg(feature = "ason")]
        {
            let result = by_file_extension::<config::FileFormat>("settings.ason");
            assert!(result.is_ok());
        }
    }
}
