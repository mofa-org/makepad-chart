use makepad_widgets::*;
use super::colors::get_color;

/// Point styles for scatter/line charts
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum DataPointStyle {
    #[default]
    Circle,
    Square,
    Triangle,
    Cross,
    Diamond,
}

/// Single data point with x, y coordinates and optional metadata
#[derive(Clone, Debug, Default)]
pub struct DataPoint {
    /// X coordinate value (None means use index)
    pub x: Option<f64>,
    /// Y coordinate value (top of bar for floating bars)
    pub y: f64,
    /// Y minimum value for floating bars (base of bar, None means use 0)
    pub y_min: Option<f64>,
    /// Bubble radius (for bubble charts)
    pub r: Option<f64>,
    /// Optional label for this point
    pub label: Option<String>,
    /// Optional metadata (for tooltips, etc.)
    pub meta: Option<String>,
}

impl DataPoint {
    /// Create a new data point with explicit x value
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x: Some(x),
            y,
            y_min: None,
            r: None,
            label: None,
            meta: None,
        }
    }

    /// Create from y value only (x will be index)
    pub fn from_y(y: f64) -> Self {
        Self {
            x: None,
            y,
            y_min: None,
            r: None,
            label: None,
            meta: None,
        }
    }

    /// Create a floating bar data point with min and max values
    pub fn floating(y_min: f64, y_max: f64) -> Self {
        Self {
            x: None,
            y: y_max,
            y_min: Some(y_min),
            r: None,
            label: None,
            meta: None,
        }
    }

    /// Create a bubble data point with x, y, and radius
    pub fn bubble(x: f64, y: f64, r: f64) -> Self {
        Self {
            x: Some(x),
            y,
            y_min: None,
            r: Some(r),
            label: None,
            meta: None,
        }
    }

    /// Add a label to the data point
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Add metadata to the data point
    pub fn with_meta(mut self, meta: impl Into<String>) -> Self {
        self.meta = Some(meta.into());
        self
    }
}

/// Dataset containing multiple data points with styling options
#[derive(Clone, Debug)]
pub struct Dataset {
    /// Display label for the dataset
    pub label: String,
    /// Data points in the dataset
    pub data: Vec<DataPoint>,
    /// Background/fill color
    pub background_color: Option<Vec4>,
    /// Border/stroke color
    pub border_color: Option<Vec4>,
    /// Border width in pixels
    pub border_width: f64,
    /// Whether this dataset is hidden
    pub hidden: bool,

    // Line chart specific
    /// Fill area under line
    pub fill: bool,
    /// Bezier curve tension (0 = straight lines, 1 = smooth)
    pub tension: f64,

    // Point options
    /// Point radius in pixels
    pub point_radius: f64,
    /// Point shape style
    pub point_style: DataPointStyle,
    /// Point background color (defaults to dataset color)
    pub point_background_color: Option<Vec4>,
    /// Point border color
    pub point_border_color: Option<Vec4>,
    /// Point border width
    pub point_border_width: f64,
    /// Hover point radius
    pub point_hover_radius: f64,

    // Bar chart specific
    /// Fixed bar thickness (None = auto)
    pub bar_thickness: Option<f64>,
    /// Bar width as percentage of category (0-1)
    pub bar_percentage: f64,
    /// Category width as percentage of available space (0-1)
    pub category_percentage: f64,
    /// Bar border radius
    pub bar_border_radius: f64,

    // Pie chart specific
    /// Offset when hovered
    pub hover_offset: f64,
}

impl Default for Dataset {
    fn default() -> Self {
        Self {
            label: String::new(),
            data: Vec::new(),
            background_color: None,
            border_color: None,
            border_width: 1.0,
            hidden: false,
            fill: false,
            tension: 0.0,
            point_radius: 3.0,
            point_style: DataPointStyle::Circle,
            point_background_color: None,
            point_border_color: None,
            point_border_width: 1.0,
            point_hover_radius: 5.0,
            bar_thickness: None,
            bar_percentage: 0.9,
            category_percentage: 0.8,
            bar_border_radius: 0.0,
            hover_offset: 10.0,
        }
    }
}

impl Dataset {
    /// Create a new dataset with a label
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            ..Default::default()
        }
    }

    /// Set data from a vector of y values (x = index)
    pub fn with_data(mut self, data: Vec<f64>) -> Self {
        self.data = data
            .into_iter()
            .map(|y| DataPoint::from_y(y))
            .collect();
        self
    }

    /// Set data from (x, y) tuples
    pub fn with_xy_data(mut self, data: Vec<(f64, f64)>) -> Self {
        self.data = data
            .into_iter()
            .map(|(x, y)| DataPoint {
                x: Some(x),
                y,
                y_min: None,
                r: None,
                label: None,
                meta: None,
            })
            .collect();
        self
    }

    /// Set data from (min, max) tuples for floating bar charts
    pub fn with_floating_data(mut self, data: Vec<(f64, f64)>) -> Self {
        self.data = data
            .into_iter()
            .map(|(y_min, y_max)| DataPoint::floating(y_min, y_max))
            .collect();
        self
    }

    /// Set data from (x, y, r) tuples for bubble charts
    pub fn with_bubble_data(mut self, data: Vec<(f64, f64, f64)>) -> Self {
        self.data = data
            .into_iter()
            .map(|(x, y, r)| DataPoint::bubble(x, y, r))
            .collect();
        self
    }

    /// Set data from DataPoint vector
    pub fn with_points(mut self, data: Vec<DataPoint>) -> Self {
        self.data = data;
        self
    }

    /// Set background color
    pub fn with_color(mut self, color: Vec4) -> Self {
        self.background_color = Some(color);
        self
    }

    /// Set background color from hex value
    pub fn with_hex_color(mut self, hex: u32) -> Self {
        let r = ((hex >> 16) & 0xFF) as f32 / 255.0;
        let g = ((hex >> 8) & 0xFF) as f32 / 255.0;
        let b = (hex & 0xFF) as f32 / 255.0;
        self.background_color = Some(vec4(r, g, b, 1.0));
        self
    }

    /// Set border color
    pub fn with_border_color(mut self, color: Vec4) -> Self {
        self.border_color = Some(color);
        self
    }

    /// Set border width
    pub fn with_border_width(mut self, width: f64) -> Self {
        self.border_width = width;
        self
    }

    /// Enable area fill (for line charts)
    pub fn with_fill(mut self, fill: bool) -> Self {
        self.fill = fill;
        self
    }

    /// Set line tension (0 = straight, 0.4 = smooth)
    pub fn with_tension(mut self, tension: f64) -> Self {
        self.tension = tension.clamp(0.0, 1.0);
        self
    }

    /// Set point radius
    pub fn with_point_radius(mut self, radius: f64) -> Self {
        self.point_radius = radius;
        self
    }

    /// Set point style
    pub fn with_point_style(mut self, style: DataPointStyle) -> Self {
        self.point_style = style;
        self
    }

    /// Set bar thickness
    pub fn with_bar_thickness(mut self, thickness: f64) -> Self {
        self.bar_thickness = Some(thickness);
        self
    }

    /// Set bar border radius
    pub fn with_bar_border_radius(mut self, radius: f64) -> Self {
        self.bar_border_radius = radius;
        self
    }

    /// Get the effective background color (with fallback)
    pub fn get_background_color(&self, index: usize) -> Vec4 {
        self.background_color.unwrap_or_else(|| get_color(index))
    }

    /// Get the effective border color (with fallback)
    pub fn get_border_color(&self, index: usize) -> Vec4 {
        self.border_color.unwrap_or_else(|| {
            let bg = self.get_background_color(index);
            // Darken the background color for border
            vec4(bg.x * 0.8, bg.y * 0.8, bg.z * 0.8, bg.w)
        })
    }
}

/// Complete chart data container
#[derive(Clone, Debug, Default)]
pub struct ChartData {
    /// Labels for categories (x-axis for bar charts, legend for pie)
    pub labels: Vec<String>,
    /// Datasets to render
    pub datasets: Vec<Dataset>,
}

impl ChartData {
    /// Create new empty chart data
    pub fn new() -> Self {
        Self::default()
    }

    /// Set category labels
    pub fn with_labels<S: Into<String>>(mut self, labels: Vec<S>) -> Self {
        self.labels = labels.into_iter().map(|l| l.into()).collect();
        self
    }

    /// Add a dataset
    pub fn add_dataset(mut self, dataset: Dataset) -> Self {
        self.datasets.push(dataset);
        self
    }

    /// Add multiple datasets
    pub fn with_datasets(mut self, datasets: Vec<Dataset>) -> Self {
        self.datasets = datasets;
        self
    }

    /// Get min/max Y values across all visible datasets
    pub fn get_y_extent(&self) -> Option<(f64, f64)> {
        let mut min = f64::MAX;
        let mut max = f64::MIN;

        for dataset in &self.datasets {
            if dataset.hidden {
                continue;
            }
            for point in &dataset.data {
                // Consider y_min for floating bars
                if let Some(y_min) = point.y_min {
                    min = min.min(y_min);
                }
                min = min.min(point.y);
                max = max.max(point.y);
            }
        }

        // Handle empty data
        if min == f64::MAX || max == f64::MIN {
            return None;
        }

        // Ensure min != max
        if (max - min).abs() < f64::EPSILON {
            min -= 1.0;
            max += 1.0;
        }

        Some((min, max))
    }

    /// Get min/max X values across all visible datasets
    pub fn get_x_extent(&self) -> Option<(f64, f64)> {
        let mut min = f64::MAX;
        let mut max = f64::MIN;

        for dataset in &self.datasets {
            if dataset.hidden {
                continue;
            }
            for (idx, point) in dataset.data.iter().enumerate() {
                let x = point.x.unwrap_or(idx as f64);
                min = min.min(x);
                max = max.max(x);
            }
        }

        // Handle empty data
        if min == f64::MAX || max == f64::MIN {
            return None;
        }

        // Ensure min != max
        if (max - min).abs() < f64::EPSILON {
            min -= 1.0;
            max += 1.0;
        }

        Some((min, max))
    }

    /// Get total value for pie charts
    pub fn get_total(&self) -> f64 {
        if self.datasets.is_empty() {
            return 0.0;
        }

        self.datasets[0]
            .data
            .iter()
            .filter(|p| p.y > 0.0)
            .map(|p| p.y)
            .sum()
    }

    /// Get number of data points in first dataset
    pub fn len(&self) -> usize {
        self.datasets.first().map(|d| d.data.len()).unwrap_or(0)
    }

    /// Check if data is empty
    pub fn is_empty(&self) -> bool {
        self.datasets.is_empty() || self.datasets.iter().all(|d| d.data.is_empty())
    }

    /// Get number of visible datasets
    pub fn visible_dataset_count(&self) -> usize {
        self.datasets.iter().filter(|d| !d.hidden).count()
    }

    /// Toggle visibility of a dataset
    pub fn toggle_dataset(&mut self, index: usize) {
        if let Some(dataset) = self.datasets.get_mut(index) {
            dataset.hidden = !dataset.hidden;
        }
    }

    /// Set visibility of a dataset
    pub fn set_dataset_visible(&mut self, index: usize, visible: bool) {
        if let Some(dataset) = self.datasets.get_mut(index) {
            dataset.hidden = !visible;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_point_creation() {
        let point = DataPoint::new(1.0, 2.0);
        assert_eq!(point.x, Some(1.0));
        assert_eq!(point.y, 2.0);
        assert!(point.label.is_none());
    }

    #[test]
    fn test_data_point_with_label() {
        let point = DataPoint::new(1.0, 2.0).with_label("Test");
        assert_eq!(point.label, Some("Test".to_string()));
    }

    #[test]
    fn test_dataset_with_data() {
        let dataset = Dataset::new("Test").with_data(vec![10.0, 20.0, 30.0]);
        assert_eq!(dataset.data.len(), 3);
        assert!(dataset.data[0].x.is_none());
        assert_eq!(dataset.data[0].y, 10.0);
        assert!(dataset.data[2].x.is_none());
        assert_eq!(dataset.data[2].y, 30.0);
    }

    #[test]
    fn test_chart_data_extent() {
        let data = ChartData::new()
            .add_dataset(Dataset::new("Test").with_data(vec![10.0, 50.0, 30.0]));

        let (min, max) = data.get_y_extent().unwrap();
        assert_eq!(min, 10.0);
        assert_eq!(max, 50.0);
    }

    #[test]
    fn test_chart_data_total() {
        let data = ChartData::new()
            .add_dataset(Dataset::new("Test").with_data(vec![10.0, 20.0, 30.0]));

        assert_eq!(data.get_total(), 60.0);
    }

    #[test]
    fn test_empty_data_handling() {
        let data = ChartData::new();
        assert!(data.get_y_extent().is_none());
    }
}
