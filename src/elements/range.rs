#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Range<T = f32> {
    Auto,
    Manual { min: T, max: T },
}