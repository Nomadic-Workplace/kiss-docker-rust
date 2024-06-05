# KISS Docker Wrapper in Rust

[![Rust](https://github.com/Nomadic-Workplace/kiss-docker-rust/actions/workflows/rust_build.yml/badge.svg)](https://github.com/Nomadic-Workplace/kiss-docker-rust/actions/workflows/rust_build.yml)

An Extremely Simple Async Docker Wrapper in Rust

Contrary to other libraries, KISS Docker does not depend on the `docker socket`, but uses `use async_process::Command`
to
build and execute the right commands.

## Quick start

```
[dependencies]
kiss_docker = "0.0.2"
```

```rust
extern crate kiss_docker;

use kiss_docker::container::Container;

#[tokio::main]
async fn main() {
    match kiss_docker::container::list_all(Some("alpine")).await {
        Ok(containers) => containers
            .iter()
            .for_each(|container| println!("{}", container.id)),
        Err(e) => {
            panic!("{}", e);
        }
    };
}
```

See the [the _examples_ directory](examples/) for more examples.

## Testing

Running the tests by default requires a few docker images locally

```bash
docker pull alpine
```

Afterwards it's the usual:

```
cargo test
```