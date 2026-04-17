mod json;
mod table;

use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};

use crate::cli::args::OutputFormat;

pub use json::*;
pub use table::*;

/// Global output format setting (thread-safe)
/// 0 = Table, 1 = Json, 2 = Compact
static OUTPUT_FORMAT: AtomicU8 = AtomicU8::new(1);
static QUIET_MODE: AtomicBool = AtomicBool::new(false);

pub fn set_format(format: OutputFormat) {
    let value = match format {
        OutputFormat::Table => 0,
        OutputFormat::Json => 1,
        OutputFormat::Compact => 2,
    };
    OUTPUT_FORMAT.store(value, Ordering::Relaxed);
}

pub fn get_format() -> OutputFormat {
    match OUTPUT_FORMAT.load(Ordering::Relaxed) {
        1 => OutputFormat::Json,
        2 => OutputFormat::Compact,
        _ => OutputFormat::Table,
    }
}

pub fn set_quiet(quiet: bool) {
    QUIET_MODE.store(quiet, Ordering::Relaxed);
}

pub fn is_quiet() -> bool {
    QUIET_MODE.load(Ordering::Relaxed)
}
