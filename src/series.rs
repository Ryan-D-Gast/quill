use crate::{Interpolation, Line, Marker};
use bon::Builder;

#[derive(Builder)]
pub struct Series<'a, T = f32> {
    pub data: Vec<(T, T)>,
    #[builder(default = "")]
    pub name: &'a str,
    #[builder(default = "Black")]
    pub color: &'a str,
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
