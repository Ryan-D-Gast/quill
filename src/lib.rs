//! Quill: A plotting library for Rust.

// TODO: Turn draft in this one file into multiple files and broken up components.
// Other things to add:
// - Axis tick settings like log scales specialized ticks etc.
// - Better legend styling
// - Support annotations
// - Add caption below the plot
// - if y_min and x_min are the same use one number for the origin e.g. (0.0 y axis, 0.0 x axis) is rendered as one 0.0 at vertex of x-y axis
// - Real testing of all the enum options for settings

pub mod elements;
pub use elements::*;

pub mod style;
pub use style::*;

mod series;
pub use series::Series;

mod plot;
pub use plot::Plot;