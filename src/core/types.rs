use makepad_widgets::*;

/// Chart alignment options
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ChartAlign {
    Start,
    #[default]
    Center,
    End,
}

/// Legend position
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum LegendPosition {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

/// Interaction mode for tooltips and hover
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum InteractionMode {
    /// Single nearest point
    Point,
    /// Nearest element of any type
    #[default]
    Nearest,
    /// All elements at same data index
    Index,
    /// All elements in same dataset
    Dataset,
    /// All elements at same x position
    X,
    /// All elements at same y position
    Y,
}

/// Interaction axis
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum InteractionAxis {
    X,
    Y,
    #[default]
    XY,
}

/// Easing function types for animations
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum EasingType {
    Linear,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    EaseInQuart,
    #[default]
    EaseOutQuart,
    EaseInOutQuart,
    EaseInQuint,
    EaseOutQuint,
    EaseInOutQuint,
    EaseInSine,
    EaseOutSine,
    EaseInOutSine,
    EaseInExpo,
    EaseOutExpo,
    EaseInOutExpo,
    EaseInCirc,
    EaseOutCirc,
    EaseInOutCirc,
    EaseInBack,
    EaseOutBack,
    EaseInOutBack,
    EaseInElastic,
    EaseOutElastic,
    EaseInOutElastic,
    EaseInBounce,
    EaseOutBounce,
    EaseInOutBounce,
}

/// Axis orientation
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum AxisOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// Chart padding structure
#[derive(Clone, Copy, Debug, Default)]
pub struct ChartPadding {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

impl ChartPadding {
    /// Create padding with same value for all sides
    pub fn all(value: f64) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }

    /// Create padding with horizontal and vertical values
    pub fn symmetric(horizontal: f64, vertical: f64) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }

    /// Create padding with individual values
    pub fn new(top: f64, right: f64, bottom: f64, left: f64) -> Self {
        Self { top, right, bottom, left }
    }
}
