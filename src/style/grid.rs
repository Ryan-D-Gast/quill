use pigment::{Color, color};

#[derive(Clone, Debug)]
pub struct GridConfig {
    pub color: Color,
    pub line_width: f32,
    pub minor_color: Color,
    pub minor_line_width: f32,
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            color: color("lightgray").unwrap(),
            line_width: 0.5,
            minor_color: color("lightgray").unwrap(),
            minor_line_width: 0.3,
        }
    }
}
