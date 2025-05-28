use raqote::SolidSource;

#[derive(Clone, Debug)]
pub struct AxisConfig {
    pub color: SolidSource,
    pub line_width: f32,
}

impl Default for AxisConfig {
    fn default() -> Self {
        Self {
            color: SolidSource { r: 0x00, g: 0x00, b: 0x00, a: 0xff }, // Black
            line_width: 1.5,
        }
    }
}