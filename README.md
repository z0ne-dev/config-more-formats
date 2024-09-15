[![Crates.io](https://img.shields.io/crates/v/config-more-formats.svg)](https://crates.io/crates/config-more-formats)
[![Workflow Status](https://github.com/z0ne-dev/xconfig/workflows/main/badge.svg)](https://github.com/z0ne-dev/xconfig/actions?query=workflow%3A%22main%22)
[![Average time to resolve an issue](https://isitmaintained.com/badge/resolution/z0ne-dev/xconfig.svg)](https://isitmaintained.com/project/z0ne-dev/xconfig "Average time to resolve an issue")
[![Percentage of issues still open](https://isitmaintained.com/badge/open/z0ne-dev/xconfig.svg)](https://isitmaintained.com/project/z0ne-dev/xconfig "Percentage of issues still open")
![Maintenance](https://img.shields.io/badge/maintenance-passively--maintained-yellowgreen.svg)

# config-more-formats

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
| JSON            | `json`       | [serde_json](https://crates.io/crates/serde_json)                       | config supplied JSON format                                                      |
| JSON5           | `json5`      | [serde-json5](https://crates.io/crates/serde_json5)                     | config supplied JSON5 format                                                     |
| RON             | `ron`        | [ron](https://crates.io/crates/ron)                                     | config supplied RON format                                                       |
| TOML            | `toml`       | [toml](https://crates.io/crates/toml)                                   | config supplied TOML format                                                      |
| YAML            | `yaml`       | [yaml-rust](https://crates.io/crates/yaml-rust)                         | config supplied YAML format (using deprecated `serde_yaml`)                      |
| INI             | `ini`        | [ini](https://crates.io/crates/ini)                                     | config supplied INI format                                                       |
|                 | `all`        |                                                                         | Enable all formats except for `yaml`                                             |

If you do not enable yaml, yaml_ng will be used for yaml files instead.
Instead of `all`, enable only the formats you need to reduce compile times and dependencies.

The current development version of config already uses [`yaml-rust2`](https://crates.io/crates/yaml-rust2)
which is a fork of `yaml-rust` and is actively maintained.
This crate uses `serde_yaml_ng` which is another actively maintained solution for YAML.

## Example of [`by_file_extension`]

```rust
use config::Config;use config_more_formats::by_file_extension;

fn main() {
    let settings = Config::builder()
        .add_source(by_file_extension("settings.toml").unwrap())
        .build()
        .unwrap();
}
```

## License

Licensed under
* MIT license ([LICENSE](LICENSE) or https://opensource.org/licenses/MIT)

at your option.
