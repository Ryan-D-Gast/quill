use raqote::SolidSource;

#[derive(Clone, Debug)]
pub struct LabelConfig {
    pub font_size: f32,
    pub color: SolidSource,
}

impl Default for LabelConfig {
    fn default() -> Self {
        Self {
            font_size: 14.0,
            color: SolidSource { r: 0x00, g: 0x00, b: 0x00, a: 0xff }, // Black
        }
    }
}