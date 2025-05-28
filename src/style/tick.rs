use raqote::SolidSource;

#[derive(Clone, Debug)]
pub struct TickConfig {
    pub font_size: f32,
    pub label_color: SolidSource,
    pub line_color: SolidSource,
    pub length: f32,
    pub text_padding: f32,
    pub density_x: f32, // Target pixels per tick for X-axis
    pub density_y: f32, // Target pixels per tick for Y-axis
}

impl Default for TickConfig {
    fn default() -> Self {
        Self {
            font_size: 10.0,
            label_color: SolidSource { r: 0x00, g: 0x00, b: 0x00, a: 0xff }, // Black
            line_color: SolidSource { r: 0x00, g: 0x00, b: 0x00, a: 0xff },  // Black
            length: 5.0,
            text_padding: 3.0,
            density_x: 50.0,
            density_y: 50.0,
        }
    }
}