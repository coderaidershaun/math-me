//! The crate's error type: every fallible `math_me` operation returns one
//! of these, so a caller can match on what went wrong rather than parse a
//! message.

use std::path::PathBuf;

/// The crate's result alias. Fallible `math_me` functions return this
/// rather than spelling out [`Error`] at every call site.
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A formula failed to compile: `mitex` rejected the LaTeX, or Typst
    /// raised a diagnostic while laying it out.
    #[error("failed to render formula `{latex}`: {message}")]
    MathCompile { latex: String, message: String },

    /// Typst failed to typeset the lesson into a PDF.
    #[error("failed to export pdf: {message}")]
    Pdf { message: String },

    /// A filesystem operation — opening, reading, or writing — failed.
    #[error("filesystem error at {path:?}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    /// The lesson's JSON did not parse, or the lesson could not be encoded.
    #[error("failed to (de)serialize lesson at {path:?}")]
    Serialization {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },

    /// The on-screen viewer failed to start (window, font, or GPU setup).
    ///
    /// Carries eframe's message rather than the `eframe::Error` itself as a
    /// `#[source]`: a lesson is data, and the windowing stack has no business
    /// in the type a lesson author matches on.
    #[error("failed to start the viewer: {message}")]
    Viewer { message: String },
}
