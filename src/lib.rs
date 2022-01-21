//! Zero-boilerplate configuration management
//!
//! ### Usage
//!
//! ```rust no_run
//! use serde_derive::Deserialize;
//! use configit::Loader;
//!
//! #[derive(Debug, Deserialize, Serialize)]
//! pub struct AppConfig {
//!     host: String,
//!     port: u16,
//! }
//!
//! let config = AppConfig::load("config.toml").expect("couldn't load `config.toml` file");
//! println!("config: {config:?}");
//! ```

#[cfg(not(any(feature = "toml_conf", feature = "yaml_conf")))]
compile_error!(
    r#"Exactly one config language feature must be enabled to use configit.
    Please enable one of either the `toml_conf` or `yaml_conf` features."#
);

#[cfg(all(feature = "toml_conf", feature = "yaml_conf"))]
compile_error!(
    r#"Exactly one config language feature must be enabled to compile
    configit.  Please disable one of either the `toml_conf` or `yaml_conf` features.
    NOTE: `toml_conf` is a default feature, so disabling it might mean switching off
    default features for configit in your Cargo.toml"#
);

use std::fs;
use std::path::Path;

#[cfg(feature = "yaml_conf")]
use serde_yaml::{from_str as from_deserialize, to_string as to_serialize};
use thiserror::Error;
#[cfg(feature = "toml_conf")]
use toml::{from_str as from_deserialize, to_string_pretty as to_serialize};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Io(#[from] std::io::Error),

    #[cfg(feature = "toml_conf")]
    #[error("{0}")]
    TomlDeserialize(#[from] toml::de::Error),

    #[cfg(feature = "toml_conf")]
    #[error("{0}")]
    TomlSerialize(#[from] toml::ser::Error),

    #[cfg(feature = "yaml_conf")]
    #[error("{0}")]
    Yaml(#[from] serde_yaml::Error),
}

pub trait Loader {
    type Output;

    fn load<P: AsRef<Path>>(filename: P) -> Result<Self::Output>;
    fn load_from_reader<R: std::io::Read>(reader: &mut R) -> Result<Self::Output>;

    fn store<P: AsRef<Path>>(&self, filename: P) -> Result<()>;
    fn store_to_writer<W: std::io::Write>(&self, writer: &mut W) -> Result<()>;
}

impl<T> Loader for T
where
    T: serde::de::DeserializeOwned + serde::ser::Serialize + Sized,
{
    type Output = Self;

    fn load<P: AsRef<Path>>(filename: P) -> Result<Self> {
        let content = fs::read_to_string(filename)?;
        let data = from_deserialize(&content)?;
        Ok(data)
    }

    fn load_from_reader<R: std::io::Read>(reader: &mut R) -> Result<Self::Output> {
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        let data = from_deserialize(&content)?;
        Ok(data)
    }

    fn store<P: AsRef<Path>>(&self, filename: P) -> Result<()> {
        let content = to_serialize(self)?;
        fs::write(filename, content)?;
        Ok(())
    }

    fn store_to_writer<W: std::io::Write>(&self, writer: &mut W) -> Result<()> {
        let content = to_serialize(self)?;
        writer.write_all(content.as_bytes())?;
        Ok(())
    }
}
