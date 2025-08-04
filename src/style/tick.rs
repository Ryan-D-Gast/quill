use crate::color::Color;

#[derive(Clone, Debug)]
pub struct TickConfig {
    pub font_size: f32,
    pub label_color: Color,
    pub line_color: Color,
    pub length: f32,
    pub text_padding: f32,
    pub density_x: f32,
    pub density_y: f32,
    pub minor_tick_length: f32,
    pub minor_tick_color: Color,
}

impl Default for TickConfig {
    fn default() -> Self {
        Self {
            font_size: 10.0,
            label_color: Color::Black,
            line_color: Color::Black,
            length: 5.0,
            text_padding: 3.0,
            density_x: 50.0,
            density_y: 50.0,
            minor_tick_length: 3.0,
            minor_tick_color: Color::Black,
        }
    }
}
