use raqote::SolidSource;

#[derive(Clone, Debug)]
pub struct LegendConfig {
    pub font_size: f32,
    pub text_color: SolidSource,
    pub border_color: SolidSource,
    pub padding: f32,
    pub item_height: f32,
    pub color_swatch_width: f32,
    pub text_offset: f32,
}

impl Default for LegendConfig {
    fn default() -> Self {
        Self {
            font_size: 12.0,
            text_color: SolidSource { r: 0x00, g: 0x00, b: 0x00, a: 0xff }, // Black
            border_color: SolidSource { r: 0x00, g: 0x00, b: 0x00, a: 0xff }, // Black
            padding: 10.0,
            item_height: 18.0,
            color_swatch_width: 15.0,
            text_offset: 5.0,
        }
    }
}