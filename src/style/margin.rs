#[derive(Clone, Debug, PartialEq)]
pub struct Margin {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

impl Default for Margin {
    fn default() -> Self {
        Self {
            top: 60.0,
            bottom: 70.0,
            left: 80.0,
            right: 30.0,
        }
    }
}