use pigment::{color, Color};

#[derive(Clone, Debug)]
pub struct GridConfig {
    pub color: Color,
    pub line_width: f32,
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            color: color("lightgray").unwrap(),
            line_width: 0.5,
        }
    }
}