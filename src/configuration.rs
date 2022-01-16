use std::path::PathBuf;

use crate::kind::Kind;

#[derive(Debug, Clone)]
pub struct Configuration {
    kind: Kind,
    app: &'static str,
}

impl Configuration {
    pub fn builder() -> Builder {
        Builder::default()
    }

    pub fn filename(&self) -> PathBuf {
        let mut filename = PathBuf::from(self.app);
        filename.set_extension(self.kind.as_file_extension());
        filename
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration { kind: Kind::default(), app: env!("CARGO_PKG_NAME") }
    }
}

#[derive(Default)]
pub struct Builder {
    kind: Option<Kind>,
    app: Option<&'static str>,
}

impl Builder {
    pub fn kind(&mut self, kind: Kind) -> &mut Self {
        self.kind = Some(kind);
        self
    }

    pub fn app(&mut self, app: &'static str) -> &mut Self {
        self.app = Some(app);
        self
    }

    pub fn build(self) -> Configuration {
        let kind = self.kind.unwrap_or_default();
        let app = self.app.unwrap_or(env!("CARGO_PKG_NAME"));

        Configuration { kind, app }
    }
}
