//! Zero-boilerplate configuration management
//!
//! ### Usage
//!
//! ```rust no_run
//! use serde_derive::Deserialize;
//! use configit::Loader;
//!
//! #[derive(Debug, Deserialize)]
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
use serde_yaml::from_str as from_deserialize;
use thiserror::Error;
#[cfg(feature = "toml_conf")]
use toml::from_str as from_deserialize;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Io(#[from] std::io::Error),

    #[cfg(feature = "toml_conf")]
    #[error("{0}")]
    Toml(#[from] toml::de::Error),

    #[cfg(feature = "yaml_conf")]
    #[error("{0}")]
    Yaml(#[from] serde_yaml::Error),
}

pub trait Loader {
    type Output;

    fn load<P: AsRef<Path>>(filename: P) -> Result<Self::Output>;
}

impl<T> Loader for T
where
    T: serde::de::DeserializeOwned + Sized,
{
    type Output = Self;

    fn load<P: AsRef<Path>>(filename: P) -> Result<Self> {
        let content = fs::read_to_string(filename)?;
        let data = from_deserialize(&content)?;
        Ok(data)
    }
}
