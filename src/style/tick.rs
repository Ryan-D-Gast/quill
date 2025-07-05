use crate::elements::Scale;
use pigment::{Color, color};

#[derive(Clone, Debug)]
pub struct TickConfig {
    pub font_size: f32,
    pub label_color: Color,
    pub line_color: Color,
    pub length: f32,
    pub text_padding: f32,
    pub density_x: f32,
    pub density_y: f32,
    pub y_scale_type: Scale,
}

impl Default for TickConfig {
    fn default() -> Self {
        Self {
            font_size: 10.0,
            label_color: color("black").unwrap(),
            line_color: color("black").unwrap(),
            length: 5.0,
            text_padding: 3.0,
            density_x: 50.0,
            density_y: 50.0,
            y_scale_type: Scale::Engineering,
        }
    }
}
