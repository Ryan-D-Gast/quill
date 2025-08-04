use crate::color::Color;

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
            color: Color::LightGray,
            line_width: 0.5,
            minor_color: Color::LightGray,
            minor_line_width: 0.3,
        }
    }
}
