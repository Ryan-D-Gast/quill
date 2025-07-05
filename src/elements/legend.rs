#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Legend {
    TopRightInside,
    TopRightOutside,
    TopLeftInside,
    BottomRightInside,
    BottomRightOutside,
    BottomLeftInside,
    RightCenterInside,
    RightCenterOutside,
    LeftCenterInside,
    TopCenter,
    BottomCenter,
    None,
}
