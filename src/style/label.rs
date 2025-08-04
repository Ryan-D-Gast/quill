use crate::color::Color;

#[derive(Clone, Debug)]
pub struct LabelConfig {
    pub font_size: f32,
    pub color: Color,
}

impl Default for LabelConfig {
    fn default() -> Self {
        Self {
            font_size: 14.0,
            color: Color::Black,
        }
    }
}
