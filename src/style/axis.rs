use crate::Color;

#[derive(Clone, Debug)]
pub struct AxisConfig {
    pub color: Color,
    pub line_width: f32,
}

impl Default for AxisConfig {
    fn default() -> Self {
        Self {
            color: Color::Black,
            line_width: 1.5,
        }
    }
}
