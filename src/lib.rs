//! Quill: An abstraction layer over the Typst typesetting engine to crate graphs and short reports in rust.

mod quill;
mod line;

pub use quill::Quill;
pub use line::LinePlot;