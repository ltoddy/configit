# configit

Zero-boilerplate configuration management in Rust 

## Installation

```
cargo add configit
```

## usage

**config.toml**

```toml
host = "127.0.0.1"
port = 8888
```

**src/main.rs**

```rust
use serde::{Deserialize, Serialize};
use configit::Loader;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
}

fn main() {
    let config = AppConfig::load("config.toml").expect("couldn't load `config.toml` file");
    println!("config: {config:?}"); // config: AppConfig { host: "127.0.0.1", port: 8888 }
}
```
