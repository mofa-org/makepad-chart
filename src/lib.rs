pub use makepad_widgets;
use makepad_widgets::*;

// Core modules
pub mod core;
pub mod scale;
pub mod coord;
pub mod element;
pub mod component;
pub mod animation;
pub mod interaction;
pub mod chart;

// Re-exports for convenience
pub use core::{
    ChartData, Dataset, DataPoint,
    ChartOptions, TitleOptions, LegendOptions, LegendPosition,
    TooltipOptions, AnimationOptions, EasingType, InteractionMode,
    AxisOptions, GridOptions, TickOptions,
    get_color, get_color_alpha, lighten, darken, CHART_COLORS,
};

pub use scale::{Scale, Tick, LinearScale, CategoryScale};
pub use coord::{CartesianCoord, PolarCoord, ChartArea, ScaleType};
pub use element::{DrawBar, DrawChartLine, DrawPoint, DrawArc, PointStyle};
pub use component::{ChartAxis, ChartGrid, ChartLegend, ChartTooltip, ChartTitle};
pub use chart::{BarChart, LineChart, PieChart, ScatterChart};
pub use animation::{ChartAnimator, AnimationManager, apply_easing, interpolate};
pub use interaction::{HitTester, HitRegion, HitData};

/// Register all live designs for makepad-charts
pub fn live_design(cx: &mut Cx) {
    crate::element::live_design(cx);
    crate::component::live_design(cx);
    crate::chart::live_design(cx);
}
