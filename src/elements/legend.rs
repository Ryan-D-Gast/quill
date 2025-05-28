#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Legend {
    TopRightInside,
    TopRightOutside,
    TopLeftInside,
    TopLeftOutside,
    BottomRightInside,
    BottomRightOutside,
    BottomLeftInside,
    BottomLeftOutside,
    RightCenterInside,
    RightCenterOutside,
    LeftCenterInside,
    LeftCenterOutside,
    TopCenter,
    BottomCenter,
    None,
}