use thiserror::Error;

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
