#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MinorGrid {
    None,
    XAxis,
    YAxis,
    Both,
}

impl Default for MinorGrid {
    fn default() -> Self {
        Self::None
    }
}
