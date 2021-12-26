# rustcli
A simple rust framework to build cli applications

[![CI](https://github.com/krishpranav/rustcli/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/krishpranav/rustcli/actions/workflows/rust.yml)

[![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg)](https://forthebadge.com)

## About:
- a rust framework to help developers create cli app easily :)

## Installation:
```toml
[dependencies]
rustcli = { git = "https://github.com/krishpranav/rustcli" }
```

## Quick Start:

- basic cli app using rustcli

```rust
use rustcli::{App};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .description("hey this is a cli app build using rust cli")
        .author("Authorone")
        .version(env!("CARGO_PKG_VERSION"))
        .usage("cli [args]")
        .action(|c| println!("Hello, {:?}", c.args));

    app.run(args);
}
```

- for more tutorial visit the [docs](https://github.com/krishpranav/rustcli/blob/master/README.md)