use pigment::{Color, color};

#[derive(Clone, Debug)]
pub struct LegendConfig {
    pub font_size: f32,
    pub text_color: Color,
    pub border_color: Color,
    pub padding: f32,
    pub item_height: f32,
    pub color_swatch_width: f32,
    pub text_offset: f32,
}

impl Default for LegendConfig {
    fn default() -> Self {
        Self {
            font_size: 12.0,
            text_color: color("black").unwrap(),
            border_color: color("black").unwrap(),
            padding: 10.0,
            item_height: 18.0,
            color_swatch_width: 15.0,
            text_offset: 5.0,
        }
    }
}