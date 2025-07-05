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
            bottom: 60.0,
            left: 60.0,
            right: 30.0,
        }
    }
}

impl Margin {
    pub fn new(top: f32, bottom: f32, left: f32, right: f32) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
        }
    }

    pub fn with_top(mut self, top: f32) -> Self {
        self.top = top;
        self
    }

    pub fn with_bottom(mut self, bottom: f32) -> Self {
        self.bottom = bottom;
        self
    }

    pub fn with_left(mut self, left: f32) -> Self {
        self.left = left;
        self
    }

    pub fn with_right(mut self, right: f32) -> Self {
        self.right = right;
        self
    }

    pub fn add_top(mut self, top: f32) -> Self {
        self.top += top;
        self
    }

    pub fn add_bottom(mut self, bottom: f32) -> Self {
        self.bottom += bottom;
        self
    }

    pub fn add_left(mut self, left: f32) -> Self {
        self.left += left;
        self
    }

    pub fn add_right(mut self, right: f32) -> Self {
        self.right += right;
        self
    }
}
