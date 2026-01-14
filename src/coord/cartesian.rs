use makepad_widgets::*;
use crate::scale::{Scale, LinearScale, CategoryScale, Tick};
use crate::core::{AxisOptions, TickOptions};

/// Represents the chart area boundaries
#[derive(Clone, Debug, Default)]
pub struct ChartArea {
    /// Left edge of chart area
    pub left: f64,
    /// Top edge of chart area
    pub top: f64,
    /// Right edge of chart area
    pub right: f64,
    /// Bottom edge of chart area
    pub bottom: f64,
}

impl ChartArea {
    /// Create a new chart area
    pub fn new(left: f64, top: f64, right: f64, bottom: f64) -> Self {
        Self { left, top, right, bottom }
    }

    /// Get the width of the chart area
    pub fn width(&self) -> f64 {
        self.right - self.left
    }

    /// Get the height of the chart area
    pub fn height(&self) -> f64 {
        self.bottom - self.top
    }

    /// Check if a point is inside the chart area
    pub fn contains(&self, x: f64, y: f64) -> bool {
        x >= self.left && x <= self.right && y >= self.top && y <= self.bottom
    }

    /// Get center point
    pub fn center(&self) -> DVec2 {
        dvec2(
            (self.left + self.right) / 2.0,
            (self.top + self.bottom) / 2.0,
        )
    }

    /// Convert to Rect
    pub fn to_rect(&self) -> Rect {
        Rect {
            pos: dvec2(self.left, self.top),
            size: dvec2(self.width(), self.height()),
        }
    }
}

/// Cartesian coordinate system for 2D charts
pub struct CartesianCoord {
    /// Total widget area
    total_rect: Rect,
    /// Computed chart area (after padding for axes)
    chart_area: ChartArea,
    /// X scale (can be category or linear)
    x_scale: ScaleType,
    /// Y scale (typically linear)
    y_scale: ScaleType,
    /// Padding for left axis labels
    left_padding: f64,
    /// Padding for bottom axis labels
    bottom_padding: f64,
    /// Padding for right axis labels
    right_padding: f64,
    /// Padding for top (title, etc.)
    top_padding: f64,
}

/// Enum to hold different scale types
pub enum ScaleType {
    Linear(LinearScale),
    Category(CategoryScale),
}

impl ScaleType {
    /// Get pixel for value
    pub fn get_pixel_for_value(&self, value: f64) -> f64 {
        match self {
            ScaleType::Linear(s) => s.get_pixel_for_value(value),
            ScaleType::Category(s) => s.get_pixel_for_value(value),
        }
    }

    /// Get value for pixel
    pub fn get_value_for_pixel(&self, pixel: f64) -> f64 {
        match self {
            ScaleType::Linear(s) => s.get_value_for_pixel(pixel),
            ScaleType::Category(s) => s.get_value_for_pixel(pixel),
        }
    }

    /// Set pixel range
    pub fn set_pixel_range(&mut self, start: f64, end: f64) {
        match self {
            ScaleType::Linear(s) => s.set_pixel_range(start, end),
            ScaleType::Category(s) => s.set_pixel_range(start, end),
        }
    }

    /// Set data range
    pub fn set_data_range(&mut self, min: f64, max: f64) {
        match self {
            ScaleType::Linear(s) => s.set_data_range(min, max),
            ScaleType::Category(s) => s.set_data_range(min, max),
        }
    }

    /// Build ticks
    pub fn build_ticks(&self, options: &TickOptions) -> Vec<Tick> {
        match self {
            ScaleType::Linear(s) => s.build_ticks(options),
            ScaleType::Category(s) => s.build_ticks(options),
        }
    }

    /// Get data bounds
    pub fn get_data_bounds(&self) -> (f64, f64) {
        match self {
            ScaleType::Linear(s) => s.get_data_bounds(),
            ScaleType::Category(s) => s.get_data_bounds(),
        }
    }

    /// Check if inverted
    pub fn is_inverted(&self) -> bool {
        match self {
            ScaleType::Linear(s) => s.is_inverted(),
            ScaleType::Category(s) => s.is_inverted(),
        }
    }

    /// Get as linear scale reference
    pub fn as_linear(&self) -> Option<&LinearScale> {
        match self {
            ScaleType::Linear(s) => Some(s),
            _ => None,
        }
    }

    /// Get as category scale reference
    pub fn as_category(&self) -> Option<&CategoryScale> {
        match self {
            ScaleType::Category(s) => Some(s),
            _ => None,
        }
    }

    /// Get as mutable linear scale
    pub fn as_linear_mut(&mut self) -> Option<&mut LinearScale> {
        match self {
            ScaleType::Linear(s) => Some(s),
            _ => None,
        }
    }

    /// Get as mutable category scale
    pub fn as_category_mut(&mut self) -> Option<&mut CategoryScale> {
        match self {
            ScaleType::Category(s) => Some(s),
            _ => None,
        }
    }
}

impl CartesianCoord {
    /// Create a new cartesian coordinate system
    pub fn new() -> Self {
        Self {
            total_rect: Rect::default(),
            chart_area: ChartArea::default(),
            x_scale: ScaleType::Category(CategoryScale::new()),
            y_scale: ScaleType::Linear(LinearScale::new().with_begin_at_zero(true)),
            left_padding: 50.0,
            bottom_padding: 30.0,
            right_padding: 20.0,
            top_padding: 20.0,
        }
    }

    /// Set the X scale
    pub fn with_x_scale(mut self, scale: ScaleType) -> Self {
        self.x_scale = scale;
        self
    }

    /// Set the Y scale
    pub fn with_y_scale(mut self, scale: ScaleType) -> Self {
        self.y_scale = scale;
        self
    }

    /// Set axis padding values
    pub fn with_padding(mut self, left: f64, top: f64, right: f64, bottom: f64) -> Self {
        self.left_padding = left;
        self.top_padding = top;
        self.right_padding = right;
        self.bottom_padding = bottom;
        self
    }

    /// Update the coordinate system with new bounds
    pub fn update(&mut self, rect: Rect) {
        self.total_rect = rect;

        // Calculate chart area after padding
        self.chart_area = ChartArea::new(
            rect.pos.x + self.left_padding,
            rect.pos.y + self.top_padding,
            rect.pos.x + rect.size.x - self.right_padding,
            rect.pos.y + rect.size.y - self.bottom_padding,
        );

        // Update scale pixel ranges
        // X axis goes left to right
        self.x_scale.set_pixel_range(self.chart_area.left, self.chart_area.right);
        // Y axis goes bottom to top (inverted pixel coordinates)
        self.y_scale.set_pixel_range(self.chart_area.bottom, self.chart_area.top);
    }

    /// Set the data ranges for both axes
    pub fn set_data_ranges(&mut self, x_min: f64, x_max: f64, y_min: f64, y_max: f64) {
        self.x_scale.set_data_range(x_min, x_max);
        self.y_scale.set_data_range(y_min, y_max);
    }

    /// Set X data range only
    pub fn set_x_data_range(&mut self, min: f64, max: f64) {
        self.x_scale.set_data_range(min, max);
    }

    /// Set Y data range only
    pub fn set_y_data_range(&mut self, min: f64, max: f64) {
        self.y_scale.set_data_range(min, max);
    }

    /// Convert data coordinates to pixel coordinates
    pub fn data_to_pixel(&self, x: f64, y: f64) -> DVec2 {
        dvec2(
            self.x_scale.get_pixel_for_value(x),
            self.y_scale.get_pixel_for_value(y),
        )
    }

    /// Convert pixel coordinates to data coordinates
    pub fn pixel_to_data(&self, x: f64, y: f64) -> DVec2 {
        dvec2(
            self.x_scale.get_value_for_pixel(x),
            self.y_scale.get_value_for_pixel(y),
        )
    }

    /// Get the chart area
    pub fn chart_area(&self) -> &ChartArea {
        &self.chart_area
    }

    /// Get the total rect
    pub fn total_rect(&self) -> Rect {
        self.total_rect
    }

    /// Get X scale reference
    pub fn x_scale(&self) -> &ScaleType {
        &self.x_scale
    }

    /// Get Y scale reference
    pub fn y_scale(&self) -> &ScaleType {
        &self.y_scale
    }

    /// Get mutable X scale
    pub fn x_scale_mut(&mut self) -> &mut ScaleType {
        &mut self.x_scale
    }

    /// Get mutable Y scale
    pub fn y_scale_mut(&mut self) -> &mut ScaleType {
        &mut self.y_scale
    }

    /// Build X axis ticks
    pub fn build_x_ticks(&self, options: &TickOptions) -> Vec<Tick> {
        self.x_scale.build_ticks(options)
    }

    /// Build Y axis ticks
    pub fn build_y_ticks(&self, options: &TickOptions) -> Vec<Tick> {
        self.y_scale.build_ticks(options)
    }

    /// Get X axis line (for drawing)
    pub fn get_x_axis_line(&self) -> (DVec2, DVec2) {
        (
            dvec2(self.chart_area.left, self.chart_area.bottom),
            dvec2(self.chart_area.right, self.chart_area.bottom),
        )
    }

    /// Get Y axis line (for drawing)
    pub fn get_y_axis_line(&self) -> (DVec2, DVec2) {
        (
            dvec2(self.chart_area.left, self.chart_area.bottom),
            dvec2(self.chart_area.left, self.chart_area.top),
        )
    }

    /// Get horizontal grid line at a Y value
    pub fn get_horizontal_grid_line(&self, y_value: f64) -> (DVec2, DVec2) {
        let y_pixel = self.y_scale.get_pixel_for_value(y_value);
        (
            dvec2(self.chart_area.left, y_pixel),
            dvec2(self.chart_area.right, y_pixel),
        )
    }

    /// Get vertical grid line at an X value
    pub fn get_vertical_grid_line(&self, x_value: f64) -> (DVec2, DVec2) {
        let x_pixel = self.x_scale.get_pixel_for_value(x_value);
        (
            dvec2(x_pixel, self.chart_area.top),
            dvec2(x_pixel, self.chart_area.bottom),
        )
    }

    /// Check if a pixel point is within the chart area
    pub fn contains_pixel(&self, x: f64, y: f64) -> bool {
        self.chart_area.contains(x, y)
    }

    /// Get the bar width for category scales
    pub fn get_bar_width(&self, bar_percent: f64) -> f64 {
        if let Some(cat_scale) = self.x_scale.as_category() {
            cat_scale.get_bar_width(bar_percent)
        } else {
            // For linear scales, estimate based on data range
            let (min, max) = self.x_scale.get_data_bounds();
            let data_points = (max - min).max(1.0);
            let band_width = self.chart_area.width() / data_points;
            band_width * bar_percent.clamp(0.1, 1.0)
        }
    }
}

impl Default for CartesianCoord {
    fn default() -> Self {
        Self::new()
    }
}

/// Axis position for rendering
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AxisPosition {
    Left,
    Right,
    Top,
    Bottom,
}

/// Information about an axis for rendering
pub struct AxisInfo {
    pub position: AxisPosition,
    pub ticks: Vec<Tick>,
    pub line_start: DVec2,
    pub line_end: DVec2,
    pub options: AxisOptions,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cartesian_coord_basic() {
        let mut coord = CartesianCoord::new();
        coord.update(Rect {
            pos: dvec2(0.0, 0.0),
            size: dvec2(400.0, 300.0),
        });

        // Chart area should be smaller than total rect due to padding
        let area = coord.chart_area();
        assert!(area.width() < 400.0);
        assert!(area.height() < 300.0);
    }

    #[test]
    fn test_data_to_pixel_conversion() {
        let mut coord = CartesianCoord::new()
            .with_x_scale(ScaleType::Linear(LinearScale::new().with_nice(false)))
            .with_y_scale(ScaleType::Linear(LinearScale::new().with_nice(false)));

        coord.update(Rect {
            pos: dvec2(0.0, 0.0),
            size: dvec2(400.0, 300.0),
        });
        coord.set_data_ranges(0.0, 100.0, 0.0, 100.0);

        let area = coord.chart_area();

        // Data point at (0, 0) should be at bottom-left of chart area
        let p1 = coord.data_to_pixel(0.0, 0.0);
        assert!((p1.x - area.left).abs() < 0.1);
        assert!((p1.y - area.bottom).abs() < 0.1);

        // Data point at (100, 100) should be at top-right of chart area
        let p2 = coord.data_to_pixel(100.0, 100.0);
        assert!((p2.x - area.right).abs() < 0.1);
        assert!((p2.y - area.top).abs() < 0.1);
    }

    #[test]
    fn test_chart_area_contains() {
        let area = ChartArea::new(50.0, 20.0, 350.0, 280.0);

        assert!(area.contains(100.0, 100.0));
        assert!(area.contains(50.0, 20.0)); // Edge
        assert!(!area.contains(0.0, 0.0));
        assert!(!area.contains(400.0, 300.0));
    }
}
