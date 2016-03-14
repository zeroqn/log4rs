//! Encoders

use std::fmt;
use std::io;
use log::LogRecord;

pub mod pattern;

/// A trait implemented by types that can serialize a `LogRecord` into a
/// `Write`r.
pub trait Encode: fmt::Debug + Send + 'static {
    /// Encodes the `LogRecord` into bytes and writes them.
    fn encode(&mut self, w: &mut Write, record: &LogRecord) -> io::Result<()>;
}

/// A trait for types that an `Encode`r will write to.
///
/// It extends `std::io::Write` and currently offers no functionality beyond
/// that, though additional methods (with default implementations) may be added
/// in the future for functionality such as output color control.
pub trait Write: io::Write {}