# `fakeroot-rs`

[![Crates.io](https://img.shields.io/crates/v/fakeroot-rs.svg)](https://crates.io/crates/fakeroot-rs)

A Rust-native implementation of `fakeroot`, using Linux user namespaces to run commands in an environment where it appears to have root privileges.

This crate provides both a library for programmatic use in your own Rust projects and a standalone CLI binary.

## Library Usage

To use this library, add it to your `Cargo.toml`. For a cleaner API, it's recommended to rename the package to `fakeroot`.

```toml
[dependencies]
# Recommended rename for a cleaner API
fakeroot = { package = "fakeroot-rs", version = "0.1.0" }
```

You can then use the `FakerootCommandExt` trait to apply the fakeroot environment to any `std::process::Command`.

### Example

Here is a simple example of how to run `whoami` inside a fakeroot environment.

```rust
use std::process::Command;
use anyhow::Result;
use fakeroot::FakerootCommandExt;

fn main() -> Result<()> {
    println!("Running 'whoami' normally:");
    Command::new("whoami").status()?;

    println!("
Running 'whoami' in a fakeroot environment:");
    let mut cmd = Command::new("whoami");
    let status = cmd.fakeroot()?.status()?; // Apply fakeroot

    if !status.success() {
        eprintln!("Fakeroot command failed!");
    }

    Ok(())
}
```

## CLI Usage

The `fakeroot-rs` binary can also be installed and used directly from the command line.

### Installation
```sh
cargo install fakeroot-rs
```

### Examples
```sh
# Run a command in the fakeroot environment
$ fakeroot-rs whoami
root

# Start a new shell with root privileges
$ fakeroot-rs
# whoami
root
# exit
```

## How It Works

This tool uses the `CLONE_NEWUSER` flag to create a new user namespace. Inside this namespace, it maps the container's root user (UID 0) to your real user ID on the host. This means that processes running inside the namespace think they are root, but any interaction with the host system is performed with your actual user privileges.

## License

This project is licensed under the MIT License.
