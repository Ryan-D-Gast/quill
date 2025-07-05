// Drawing helper functions for Plot SVG rendering
use pigment::Color;

pub fn to_svg_color_string(color: &Color) -> String {
    let (r, g, b) = color.rgb();
    format!("#{:02x}{:02x}{:02x}", r, g, b)
}

mod axis;
mod data_series;
mod label;
mod legend;
mod ticks_and_grids;

pub use axis::draw_axis_lines;
pub use data_series::draw_data_series;
pub use label::{draw_title, draw_x_label, draw_y_label};
pub use legend::draw_legend;
pub use ticks_and_grids::draw_ticks_and_grids;
