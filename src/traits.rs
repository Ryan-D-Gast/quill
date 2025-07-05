use std::fmt::Display;

/// Trait for types that can be used as plot values
pub trait PlotValue:
    Copy
    + PartialOrd
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + Display
    + 'static
{
    /// Convert to f32 for SVG coordinate calculations
    fn to_f32(self) -> f32;

    /// Maximum value for this type
    fn max_value() -> Self;

    /// Minimum value for this type  
    fn min_value() -> Self;

    /// Epsilon value for floating point comparisons
    fn epsilon() -> Self;

    /// Create from f32 (for calculations)
    fn from_f32(val: f32) -> Self;
}

impl PlotValue for f32 {
    fn to_f32(self) -> f32 {
        self
    }
    fn max_value() -> Self {
        f32::MAX
    }
    fn min_value() -> Self {
        f32::MIN
    }
    fn epsilon() -> Self {
        f32::EPSILON
    }
    fn from_f32(val: f32) -> Self {
        val
    }
}

impl PlotValue for f64 {
    fn to_f32(self) -> f32 {
        self as f32
    }
    fn max_value() -> Self {
        f64::MAX
    }
    fn min_value() -> Self {
        f64::MIN
    }
    fn epsilon() -> Self {
        f64::EPSILON
    }
    fn from_f32(val: f32) -> Self {
        val as f64
    }
}

impl PlotValue for i32 {
    fn to_f32(self) -> f32 {
        self as f32
    }
    fn max_value() -> Self {
        i32::MAX
    }
    fn min_value() -> Self {
        i32::MIN
    }
    fn epsilon() -> Self {
        1
    }
    fn from_f32(val: f32) -> Self {
        val as i32
    }
}

impl PlotValue for i64 {
    fn to_f32(self) -> f32 {
        self as f32
    }
    fn max_value() -> Self {
        i64::MAX
    }
    fn min_value() -> Self {
        i64::MIN
    }
    fn epsilon() -> Self {
        1
    }
    fn from_f32(val: f32) -> Self {
        val as i64
    }
}
