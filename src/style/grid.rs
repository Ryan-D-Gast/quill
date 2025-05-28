use raqote::SolidSource;

#[derive(Clone, Debug)]
pub struct GridConfig {
    pub color: SolidSource,
    pub line_width: f32,
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            color: SolidSource { r: 0xcc, g: 0xcc, b: 0xcc, a: 0xff }, // Light gray
            line_width: 0.5,
        }
    }
}