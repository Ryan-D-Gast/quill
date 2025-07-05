use pigment::{Color, color};

#[derive(Clone, Debug)]
pub struct TitleConfig {
    pub font_size: f32,
    pub color: Color,
}

impl Default for TitleConfig {
    fn default() -> Self {
        Self {
            font_size: 20.0,
            color: color("black").unwrap(),
        }
    }
}
