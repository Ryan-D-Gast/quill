//! # Quill: A plotting library for Rust
//!
//! Quill is a fast and flexible plotting library for Rust that creates beautiful, 
//! publication-ready charts and graphs. It supports various plot types including line plots,
//! scatter plots, and more, with extensive customization options for styling and formatting.
//!
//! ## Quick Start
//!
//! ```rust
//! use quill::prelude::*;
//!
//! // Generate some sample data
//! let data: Vec<(f64, f64)> = (0..=50)
//!     .map(|x| {
//!         let xf = x as f64 * 0.2;
//!         (xf, xf.sin())
//!     })
//!     .collect();
//!
//! // Create a line plot
//! let plot = Plot::builder()
//!     .dimensions((800, 600))
//!     .title("Sine Wave Example")
//!     .x_label("X Values")
//!     .y_label("Y Values")
//!     .legend(Legend::TopRightOutside)
//!     .grid(Grid::Solid)
//!     .data([Series::builder()
//!         .name("sin(x)")
//!         .color(Color::Blue)
//!         .data(data)
//!         .marker(Marker::Circle)
//!         .marker_size(3.0)
//!         .line(Line::Solid)
//!         .build()])
//!     .build();
//!
//! // Save to file
//! plot.to_svg("sine_wave.svg").unwrap();
//!
//! // With feat="png"
//! let scale = 1.0; // Scale factor for PNG
//! plot.to_png("sine_wave.png", scale).unwrap();
//! ```
//!
//! ## More Examples
//!
//! For more examples including scatter plots, different scales, and advanced styling,
//! check out the [`examples/`](https://github.com/Ryan-D-Gast/quill/tree/main/examples) directory in the repository.
//!
//! ## License
//!
//! Copyright 2025 Ryan D. Gast
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! you may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//!
//! http://www.apache.org/licenses/LICENSE-2.0
//!
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.

// TODO:
// Things to add:
// - Customizable ticks such as engineering notation, custom symbols, etc.
// - Better legend styling
// - Support annotations
// - Add caption below the plot
// - if y_min and x_min are the same use one number for the origin e.g. (0.0 y axis, 0.0 x axis) is rendered as one 0.0 at vertex of x-y axis
// - Real testing of all the enum options for settings

pub mod color;
pub mod draw;
pub mod plot;
pub mod series;
pub mod traits;
pub mod elements;
pub mod style;

// Users should use the prelude for convenience as it re-exports what they need for almost all use cases
pub mod prelude;