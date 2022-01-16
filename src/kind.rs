/// Kind of configuration file
#[derive(Debug, Clone, Copy)]
pub enum Kind {
    Toml,
    Yaml,
}

impl Kind {
    pub fn as_file_extension(&self) -> &'static str {
        match self {
            Kind::Toml => "toml",
            Kind::Yaml => "yaml",
        }
    }
}

impl Default for Kind {
    fn default() -> Self {
        #[cfg(feature = "toml_conf")]
        {
            Kind::Toml
        }

        #[cfg(feature = "yaml_conf")]
        {
            Kind::Yaml
        }
    }
}
