#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Range {
    Auto,
    Manual { min: f32, max: f32 },
}