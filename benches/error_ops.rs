fn main() {
    divan::main();
}

use anyhow::{anyhow, Context, Result};
use std::io;

#[divan::bench]
fn create_simple_error() -> Result<()> {
    Err(anyhow!("simple error message"))
}

#[divan::bench]
fn create_formatted_error() -> Result<()> {
    let value = 42;
    Err(anyhow!("error with value: {}", value))
}

#[divan::bench]
fn error_with_context() -> Result<String> {
    std::fs::read_to_string("/nonexistent/path").context("failed to read config file")
}

#[divan::bench]
fn error_propagation() -> Result<()> {
    fn inner() -> Result<()> {
        Err(anyhow!("inner error"))
    }

    inner().context("outer context")
}

#[divan::bench]
fn error_downcast() {
    let error: anyhow::Error = io::Error::new(io::ErrorKind::NotFound, "file not found").into();
    divan::black_box(error.downcast_ref::<io::Error>());
}

#[divan::bench]
fn error_chain_iteration() {
    let error = std::fs::read_to_string("/nonexistent/path")
        .context("failed to read config")
        .unwrap_err();

    for cause in error.chain() {
        divan::black_box(cause);
    }
}

#[divan::bench]
fn error_display() {
    let error: anyhow::Error = io::Error::new(io::ErrorKind::NotFound, "file not found").into();
    let display_string = format!("{}", error);
    divan::black_box(display_string);
}

#[divan::bench]
fn error_debug() {
    let error: anyhow::Error = io::Error::new(io::ErrorKind::NotFound, "file not found").into();
    let debug_string = format!("{:?}", error);
    divan::black_box(debug_string);
}
