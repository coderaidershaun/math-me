//! `math_me`: author a math lesson in short, readable Rust — headings,
//! notes, prose mixed with inline and display maths, hoverable term
//! explanations, figures, plots, an on-screen viewer, and Typst-native PDF
//! export.
//!
//! A lesson is data, not code: [`LessonBuilder`] produces an owned,
//! serializable [`Lesson`] — strings, vecs, floats, no closures — so a
//! tutoring engine can compile a lesson once, [`Lesson::save`] it, and any
//! viewer built on this library can [`Lesson::load`] and replay it without
//! recompiling.
//!
//! ```no_run
//! use math_me::prelude::*;
//!
//! let lesson = Lesson::builder("A short lesson")
//!     .heading("What is a derivative?")
//!     .para(|p| p.text("The derivative of ").math(r"x^2").text(" is 2x."))
//!     .display(r"\frac{d}{dx} x^2 = 2x")
//!     .build();
//!
//! assert!(lesson.audit().is_empty());
//! ```

mod error;
mod expr;
mod figure;
mod formula;
mod glossary;
mod lesson;
mod mathview;
mod pdf;
mod plot;
mod symbols;
mod terms;
mod viewer;

pub use error::{Error, Result};
pub use lesson::{AuditFinding, Block, Figure, Inline, Lesson, LessonBuilder, ParaBuilder};
pub use plot::Plot;

/// Everything a lesson author needs, in one `use`.
pub mod prelude {
    pub use crate::{AuditFinding, Figure, Lesson, LessonBuilder, ParaBuilder, Plot, Result};
}
