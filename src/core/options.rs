use makepad_widgets::*;
use super::types::{ChartAlign, ChartPadding, LegendPosition, EasingType, InteractionMode, InteractionAxis};

/// Main chart options
#[derive(Clone, Debug)]
pub struct ChartOptions {
    /// Enable responsive resizing
    pub responsive: bool,
    /// Maintain aspect ratio when resizing
    pub maintain_aspect_ratio: bool,
    /// Aspect ratio (width / height)
    pub aspect_ratio: f64,
    /// Chart padding
    pub padding: ChartPadding,

    /// Title options
    pub title: TitleOptions,
    /// Subtitle options
    pub subtitle: TitleOptions,
    /// Legend options
    pub legend: LegendOptions,
    /// Tooltip options
    pub tooltip: TooltipOptions,
    /// Animation options
    pub animation: AnimationOptions,
    /// Interaction options
    pub interaction: InteractionOptions,
    /// Scale options (for cartesian charts)
    pub scales: ScalesOptions,
}

impl Default for ChartOptions {
    fn default() -> Self {
        Self {
            responsive: true,
            maintain_aspect_ratio: true,
            aspect_ratio: 2.0,
            padding: ChartPadding::all(10.0),
            title: TitleOptions::default(),
            subtitle: TitleOptions {
                font_size: 12.0,
                ..Default::default()
            },
            legend: LegendOptions::default(),
            tooltip: TooltipOptions::default(),
            animation: AnimationOptions::default(),
            interaction: InteractionOptions::default(),
            scales: ScalesOptions::default(),
        }
    }
}

impl ChartOptions {
    /// Create new default options
    pub fn new() -> Self {
        Self::default()
    }

    /// Set title text
    pub fn with_title(mut self, text: impl Into<String>) -> Self {
        self.title.display = true;
        self.title.text = text.into();
        self
    }

    /// Set subtitle text
    pub fn with_subtitle(mut self, text: impl Into<String>) -> Self {
        self.subtitle.display = true;
        self.subtitle.text = text.into();
        self
    }

    /// Configure legend
    pub fn with_legend(mut self, display: bool, position: LegendPosition) -> Self {
        self.legend.display = display;
        self.legend.position = position;
        self
    }

    /// Enable/disable tooltip
    pub fn with_tooltip(mut self, enabled: bool) -> Self {
        self.tooltip.enabled = enabled;
        self
    }

    /// Set animation duration in milliseconds
    pub fn with_animation_duration(mut self, duration: f64) -> Self {
        self.animation.duration = duration;
        self
    }

    /// Disable animations
    pub fn without_animation(mut self) -> Self {
        self.animation.duration = 0.0;
        self
    }

    /// Set Y axis to begin at zero
    pub fn with_begin_at_zero(mut self, begin_at_zero: bool) -> Self {
        self.scales.y.begin_at_zero = begin_at_zero;
        self
    }
}

/// Title and subtitle options
#[derive(Clone, Debug)]
pub struct TitleOptions {
    /// Display the title
    pub display: bool,
    /// Title text
    pub text: String,
    /// Text color
    pub color: Vec4,
    /// Font size in pixels
    pub font_size: f64,
    /// Padding around title
    pub padding: f64,
    /// Text alignment
    pub align: ChartAlign,
}

impl Default for TitleOptions {
    fn default() -> Self {
        Self {
            display: false,
            text: String::new(),
            color: vec4(0.2, 0.2, 0.2, 1.0),
            font_size: 16.0,
            padding: 10.0,
            align: ChartAlign::Center,
        }
    }
}

/// Legend options
#[derive(Clone, Debug)]
pub struct LegendOptions {
    /// Display the legend
    pub display: bool,
    /// Legend position
    pub position: LegendPosition,
    /// Alignment within position
    pub align: ChartAlign,
    /// Reverse order of items
    pub reverse: bool,
    /// Legend label options
    pub labels: LegendLabelOptions,
}

impl Default for LegendOptions {
    fn default() -> Self {
        Self {
            display: true,
            position: LegendPosition::Top,
            align: ChartAlign::Center,
            reverse: false,
            labels: LegendLabelOptions::default(),
        }
    }
}

/// Legend label styling options
#[derive(Clone, Debug)]
pub struct LegendLabelOptions {
    /// Color box width
    pub box_width: f64,
    /// Color box height
    pub box_height: f64,
    /// Label text color
    pub color: Vec4,
    /// Font size
    pub font_size: f64,
    /// Padding between items
    pub padding: f64,
    /// Use point style instead of box
    pub use_point_style: bool,
}

impl Default for LegendLabelOptions {
    fn default() -> Self {
        Self {
            box_width: 40.0,
            box_height: 12.0,
            color: vec4(0.4, 0.4, 0.4, 1.0),
            font_size: 12.0,
            padding: 10.0,
            use_point_style: false,
        }
    }
}

/// Tooltip options
#[derive(Clone, Debug)]
pub struct TooltipOptions {
    /// Enable tooltips
    pub enabled: bool,
    /// Tooltip trigger mode
    pub mode: InteractionMode,
    /// Only show when directly intersecting element
    pub intersect: bool,
    /// Background color
    pub background_color: Vec4,
    /// Title text color
    pub title_color: Vec4,
    /// Body text color
    pub body_color: Vec4,
    /// Border color
    pub border_color: Vec4,
    /// Border width
    pub border_width: f64,
    /// Corner radius
    pub corner_radius: f64,
    /// Padding inside tooltip
    pub padding: f64,
    /// Title font size
    pub title_font_size: f64,
    /// Body font size
    pub body_font_size: f64,
}

impl Default for TooltipOptions {
    fn default() -> Self {
        Self {
            enabled: true,
            mode: InteractionMode::Nearest,
            intersect: true,
            background_color: vec4(0.0, 0.0, 0.0, 0.8),
            title_color: vec4(1.0, 1.0, 1.0, 1.0),
            body_color: vec4(1.0, 1.0, 1.0, 0.9),
            border_color: vec4(0.0, 0.0, 0.0, 0.0),
            border_width: 0.0,
            corner_radius: 6.0,
            padding: 10.0,
            title_font_size: 13.0,
            body_font_size: 12.0,
        }
    }
}

/// Animation options
#[derive(Clone, Debug)]
pub struct AnimationOptions {
    /// Animation duration in milliseconds
    pub duration: f64,
    /// Easing function
    pub easing: EasingType,
    /// Delay before starting in milliseconds
    pub delay: f64,
    /// Loop animation continuously
    pub loop_animation: bool,
}

impl Default for AnimationOptions {
    fn default() -> Self {
        Self {
            duration: 400.0,
            easing: EasingType::EaseOutQuart,
            delay: 0.0,
            loop_animation: false,
        }
    }
}

impl AnimationOptions {
    /// Create instant (no animation)
    pub fn none() -> Self {
        Self {
            duration: 0.0,
            ..Default::default()
        }
    }

    /// Create fast animation
    pub fn fast() -> Self {
        Self {
            duration: 200.0,
            ..Default::default()
        }
    }

    /// Create slow animation
    pub fn slow() -> Self {
        Self {
            duration: 800.0,
            ..Default::default()
        }
    }
}

/// Interaction options
#[derive(Clone, Debug)]
pub struct InteractionOptions {
    /// Interaction mode
    pub mode: InteractionMode,
    /// Only trigger when directly over element
    pub intersect: bool,
    /// Which axis to consider for interaction
    pub axis: InteractionAxis,
}

impl Default for InteractionOptions {
    fn default() -> Self {
        Self {
            mode: InteractionMode::Nearest,
            intersect: true,
            axis: InteractionAxis::XY,
        }
    }
}

/// Combined scales options for X and Y axes
#[derive(Clone, Debug, Default)]
pub struct ScalesOptions {
    /// X axis options
    pub x: AxisOptions,
    /// Y axis options
    pub y: AxisOptions,
}

/// Individual axis options
#[derive(Clone, Debug)]
pub struct AxisOptions {
    /// Display the axis
    pub display: bool,
    /// Axis title
    pub title: AxisTitleOptions,
    /// Grid line options
    pub grid: GridOptions,
    /// Tick mark options
    pub ticks: TickOptions,
    /// Fixed minimum value
    pub min: Option<f64>,
    /// Fixed maximum value
    pub max: Option<f64>,
    /// Suggested minimum (may be exceeded by data)
    pub suggested_min: Option<f64>,
    /// Suggested maximum (may be exceeded by data)
    pub suggested_max: Option<f64>,
    /// Start scale at zero
    pub begin_at_zero: bool,
    /// Reverse the axis direction
    pub reverse: bool,
    /// Stack values in this axis
    pub stacked: bool,
}

impl Default for AxisOptions {
    fn default() -> Self {
        Self {
            display: true,
            title: AxisTitleOptions::default(),
            grid: GridOptions::default(),
            ticks: TickOptions::default(),
            min: None,
            max: None,
            suggested_min: None,
            suggested_max: None,
            begin_at_zero: false,
            reverse: false,
            stacked: false,
        }
    }
}

/// Axis title options
#[derive(Clone, Debug)]
pub struct AxisTitleOptions {
    /// Display axis title
    pub display: bool,
    /// Title text
    pub text: String,
    /// Text color
    pub color: Vec4,
    /// Font size
    pub font_size: f64,
    /// Padding from axis
    pub padding: f64,
}

impl Default for AxisTitleOptions {
    fn default() -> Self {
        Self {
            display: false,
            text: String::new(),
            color: vec4(0.4, 0.4, 0.4, 1.0),
            font_size: 12.0,
            padding: 4.0,
        }
    }
}

/// Grid line options
#[derive(Clone, Debug)]
pub struct GridOptions {
    /// Display grid lines
    pub display: bool,
    /// Grid line color
    pub color: Vec4,
    /// Grid line width
    pub line_width: f64,
    /// Draw border around chart area
    pub draw_border: bool,
    /// Draw grid lines in chart area
    pub draw_on_chart_area: bool,
    /// Draw tick marks on axis
    pub draw_ticks: bool,
    /// Tick length in pixels
    pub tick_length: f64,
}

impl Default for GridOptions {
    fn default() -> Self {
        Self {
            display: true,
            color: vec4(0.9, 0.9, 0.9, 1.0),
            line_width: 1.0,
            draw_border: true,
            draw_on_chart_area: true,
            draw_ticks: true,
            tick_length: 6.0,
        }
    }
}

/// Tick mark options
#[derive(Clone, Debug)]
pub struct TickOptions {
    /// Display tick labels
    pub display: bool,
    /// Tick label color
    pub color: Vec4,
    /// Font size
    pub font_size: f64,
    /// Padding from axis line
    pub padding: f64,
    /// Maximum number of ticks
    pub max_ticks_limit: usize,
    /// Fixed step size between ticks
    pub step_size: Option<f64>,
    /// Include bounds in ticks
    pub include_bounds: bool,
    /// Label rotation in degrees
    pub max_rotation: f64,
    /// Minimum rotation in degrees
    pub min_rotation: f64,
}

impl Default for TickOptions {
    fn default() -> Self {
        Self {
            display: true,
            color: vec4(0.4, 0.4, 0.4, 1.0),
            font_size: 11.0,
            padding: 3.0,
            max_ticks_limit: 11,
            step_size: None,
            include_bounds: true,
            max_rotation: 50.0,
            min_rotation: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_options() {
        let options = ChartOptions::default();
        assert!(options.responsive);
        assert!(!options.title.display);
        assert!(options.legend.display);
        assert!(options.tooltip.enabled);
    }

    #[test]
    fn test_options_builder() {
        let options = ChartOptions::new()
            .with_title("Test Chart")
            .with_legend(true, LegendPosition::Bottom)
            .with_begin_at_zero(true);

        assert!(options.title.display);
        assert_eq!(options.title.text, "Test Chart");
        assert_eq!(options.legend.position, LegendPosition::Bottom);
        assert!(options.scales.y.begin_at_zero);
    }
}
