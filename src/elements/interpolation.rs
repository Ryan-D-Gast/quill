#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Interpolation {
    /// Connect points with straight lines (default)
    Linear,
    /// Smooth curve using cubic Bezier interpolation
    Bezier,
    /// Step function - horizontal then vertical lines
    Step,
    /// Smooth spline interpolation
    Spline,
}
