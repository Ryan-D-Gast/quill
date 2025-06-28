use bon::Builder;
use crate::{Line, Marker};

#[derive(Builder)]
pub struct Series<T = f32> {
    pub data: Vec<(T, T)>,
    #[builder(default = "".to_string())]
    pub name: String,
    #[builder(default = "Black".to_string())]
    pub color: String,
    #[builder(default = Line::Solid)]
    pub line: Line,
    #[builder(default = Marker::None)]
    pub marker: Marker,
    #[builder(default = 1.0)]
    pub marker_size: f32,
    #[builder(default = 1.0)]
    pub line_width: f32,
}