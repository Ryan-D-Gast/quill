use crate::{Interpolation, Line, Marker, Color};
use bon::Builder;

#[derive(Clone, Builder)]
pub struct Series<'a, T = f32> {
    pub data: Vec<(T, T)>,
    #[builder(default = "")]
    pub name: &'a str,
    #[builder(default = Color::Black, into)]
    pub color: Color,
    #[builder(default = Line::Solid)]
    pub line: Line,
    #[builder(default = Marker::None)]
    pub marker: Marker,
    #[builder(default = 1.0)]
    pub marker_size: f32,
    #[builder(default = 1.0)]
    pub line_width: f32,
    #[builder(default = Interpolation::Linear)]
    pub interpolation: Interpolation,
}

impl<'a, T> Default for Series<'a, T> {
    fn default() -> Self {
        Series::builder().data(vec![]).build()
    }
}