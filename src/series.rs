use bon::Builder;
use crate::{Line, Marker};

#[derive(Builder)]
pub struct Series {
    pub data: Vec<(f32, f32)>,
    #[builder(default = "".to_string())]
    pub name: String,
    #[builder(default = "Black".to_string())]
    pub color: String,
    #[builder(default = Line::Solid)]
    pub line: Line,
    #[builder(default = Marker::None)]
    pub marker: Marker,
}