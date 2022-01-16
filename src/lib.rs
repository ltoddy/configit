// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

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

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

#[cfg(feature = "yaml_conf")]
use serde_yaml::from_str as from_deserialize;
#[cfg(feature = "toml_conf")]
use toml::from_str as from_deserialize;

use crate::error::Result;
use crate::kind::Kind;

pub mod error;
pub mod kind;

pub trait Loader {
    type Output;

    fn load(app: &'static str) -> Result<Self::Output>;

    fn load_by<P: AsRef<Path>>(filename: P) -> Result<Self::Output>;
}

impl<T> Loader for T
where
    T: serde::de::DeserializeOwned + Sized,
{
    type Output = Self;

    fn load(app: &'static str) -> Result<Self> {
        let mut filename = PathBuf::from(app);
        filename.set_extension(Kind::default().as_file_extension());
        Self::load_by(filename)
    }

    fn load_by<P: AsRef<Path>>(filename: P) -> Result<Self> {
        let file = File::open(filename.as_ref())?;
        let mut reader = BufReader::new(file);
        let mut content = String::with_capacity(1024);
        reader.read_to_string(&mut content)?;
        let data = from_deserialize(&content)?;
        Ok(data)
    }
}
