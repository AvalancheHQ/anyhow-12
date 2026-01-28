fn main() {
    divan::main();
}

use anyhow::{anyhow, bail, Context, Result};
use std::io;

// Basic error creation benchmarks
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
fn create_error_from_string() -> Result<()> {
    Err(anyhow::Error::msg("error message".to_string()))
}

#[divan::bench]
fn create_error_from_static_str() -> Result<()> {
    Err(anyhow::Error::msg("static error message"))
}

#[divan::bench]
fn bail_macro() -> Result<()> {
    bail!("bailing out with error")
}

// Error conversion benchmarks
#[divan::bench]
fn convert_io_error() -> Result<()> {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    Err(io_err.into())
}

#[divan::bench]
fn convert_string_error() -> Result<()> {
    Err(anyhow!("string error"))
}

// Context addition benchmarks
#[divan::bench]
fn error_with_context() -> Result<String> {
    std::fs::read_to_string("/nonexistent/path").context("failed to read config file")
}

#[divan::bench]
fn error_with_lazy_context() -> Result<String> {
    let path = "/nonexistent/path";
    std::fs::read_to_string(path).with_context(|| format!("failed to read file: {}", path))
}

#[divan::bench]
fn multiple_context_layers() -> Result<()> {
    fn inner() -> Result<()> {
        Err(anyhow!("inner error"))
    }
    
    fn middle() -> Result<()> {
        inner().context("middle context")
    }
    
    middle().context("outer context")
}

#[divan::bench]
fn error_propagation() -> Result<()> {
    fn inner() -> Result<()> {
        Err(anyhow!("inner error"))
    }

    inner().context("outer context")
}

// Downcasting benchmarks
#[divan::bench]
fn error_downcast_ref() {
    let error: anyhow::Error = io::Error::new(io::ErrorKind::NotFound, "file not found").into();
    divan::black_box(error.downcast_ref::<io::Error>());
}

#[divan::bench]
fn error_downcast_success() {
    let error: anyhow::Error = io::Error::new(io::ErrorKind::NotFound, "file not found").into();
    let result = error.downcast_ref::<io::Error>();
    divan::black_box(result.is_some());
}

#[divan::bench]
fn error_downcast_failure() {
    let error: anyhow::Error = anyhow!("not an io error");
    let result = error.downcast_ref::<io::Error>();
    divan::black_box(result.is_none());
}

#[divan::bench]
fn error_is_check() {
    let error: anyhow::Error = io::Error::new(io::ErrorKind::NotFound, "file not found").into();
    divan::black_box(error.is::<io::Error>());
}

// Chain iteration benchmarks
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
fn error_chain_count() {
    let error = std::fs::read_to_string("/nonexistent/path")
        .context("outer context")
        .context("middle context")
        .context("inner context")
        .unwrap_err();

    let count = error.chain().count();
    divan::black_box(count);
}

#[divan::bench]
fn error_root_cause() {
    let error = std::fs::read_to_string("/nonexistent/path")
        .context("failed to read config")
        .unwrap_err();

    let root = error.root_cause();
    divan::black_box(root);
}

// Display and Debug benchmarks
#[divan::bench]
fn error_display() {
    let error: anyhow::Error = io::Error::new(io::ErrorKind::NotFound, "file not found").into();
    let display_string = format!("{}", error);
    divan::black_box(display_string);
}

#[divan::bench]
fn error_display_alternate() {
    let error = std::fs::read_to_string("/nonexistent/path")
        .context("failed to read config")
        .unwrap_err();
    let display_string = format!("{:#}", error);
    divan::black_box(display_string);
}

#[divan::bench]
fn error_debug() {
    let error: anyhow::Error = io::Error::new(io::ErrorKind::NotFound, "file not found").into();
    let debug_string = format!("{:?}", error);
    divan::black_box(debug_string);
}

#[divan::bench]
fn error_debug_alternate() {
    let error: anyhow::Error = io::Error::new(io::ErrorKind::NotFound, "file not found").into();
    let debug_string = format!("{:#?}", error);
    divan::black_box(debug_string);
}

// Complex error scenarios
#[divan::bench]
fn nested_error_with_context() -> Result<()> {
    fn level_3() -> Result<()> {
        Err(io::Error::new(io::ErrorKind::PermissionDenied, "access denied").into())
    }
    
    fn level_2() -> Result<()> {
        level_3().context("level 2 operation failed")
    }
    
    fn level_1() -> Result<()> {
        level_2().context("level 1 operation failed")
    }
    
    level_1().context("top level operation failed")
}

#[divan::bench]
fn error_cloning() {
    let error: anyhow::Error = anyhow!("original error");
    let error_ref = &error;
    let cloned = format!("{}", error_ref);
    divan::black_box(cloned);
}

// Result Ok benchmarks for comparison
#[divan::bench]
fn result_ok() -> Result<i32> {
    Ok(42)
}

#[divan::bench]
fn result_ok_with_context() -> Result<i32> {
    Ok::<i32, io::Error>(42).context("this won't trigger")
}
