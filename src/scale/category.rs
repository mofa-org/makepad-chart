use super::traits::{Scale, Tick};
use crate::core::TickOptions;

/// Category scale for discrete labels (e.g., bar chart x-axis)
#[derive(Clone, Debug)]
pub struct CategoryScale {
    // Category labels
    labels: Vec<String>,

    // Pixel range
    pixel_start: f64,
    pixel_end: f64,

    // Options
    /// Offset the bars/points from grid lines
    offset: bool,
}

impl CategoryScale {
    /// Create a new category scale
    pub fn new() -> Self {
        Self {
            labels: Vec::new(),
            pixel_start: 0.0,
            pixel_end: 100.0,
            offset: true, // Default: center items between grid lines
        }
    }

    /// Set whether to offset items from grid lines
    /// When true, items are centered between grid lines (good for bar charts)
    /// When false, items are placed on grid lines (good for line charts)
    pub fn with_offset(mut self, offset: bool) -> Self {
        self.offset = offset;
        self
    }

    /// Set labels directly
    pub fn with_labels(mut self, labels: Vec<String>) -> Self {
        self.labels = labels;
        self
    }

    /// Get the number of categories
    pub fn len(&self) -> usize {
        self.labels.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.labels.is_empty()
    }

    /// Get label at index
    pub fn get_label(&self, index: usize) -> Option<&str> {
        self.labels.get(index).map(|s| s.as_str())
    }

    /// Get all labels
    pub fn labels(&self) -> &[String] {
        &self.labels
    }

    /// Get the band width (space allocated for each category)
    pub fn get_band_width(&self) -> f64 {
        if self.labels.is_empty() {
            return 0.0;
        }

        let pixel_range = (self.pixel_end - self.pixel_start).abs();
        pixel_range / self.labels.len() as f64
    }

    /// Get the bar width (actual bar width, accounting for padding)
    /// bar_percent is typically 0.8 (80% of band width)
    pub fn get_bar_width(&self, bar_percent: f64) -> f64 {
        self.get_band_width() * bar_percent.clamp(0.1, 1.0)
    }

    /// Get pixel position for a category index
    pub fn get_pixel_for_index(&self, index: usize) -> f64 {
        if self.labels.is_empty() {
            return self.pixel_start;
        }

        let band_width = self.get_band_width();
        let base_pixel = self.pixel_start + (index as f64 * band_width);

        if self.offset {
            // Center of the band
            base_pixel + band_width / 2.0
        } else {
            // Start of the band (on grid line)
            base_pixel
        }
    }

    /// Get category index for a pixel position
    pub fn get_index_for_pixel(&self, pixel: f64) -> usize {
        if self.labels.is_empty() {
            return 0;
        }

        let band_width = self.get_band_width();
        if band_width == 0.0 {
            return 0;
        }

        let adjusted_pixel = if self.offset {
            pixel - band_width / 2.0
        } else {
            pixel
        };

        let index = ((adjusted_pixel - self.pixel_start) / band_width).round() as i64;
        index.clamp(0, (self.labels.len() - 1) as i64) as usize
    }
}

impl Default for CategoryScale {
    fn default() -> Self {
        Self::new()
    }
}

impl Scale for CategoryScale {
    fn scale_type(&self) -> &'static str {
        "category"
    }

    fn set_data_range(&mut self, _min: f64, _max: f64) {
        // Category scale doesn't use numeric data range
        // Labels are set directly via with_labels()
    }

    fn set_pixel_range(&mut self, start: f64, end: f64) {
        self.pixel_start = start;
        self.pixel_end = end;
    }

    fn get_pixel_for_value(&self, value: f64) -> f64 {
        // Value is interpreted as category index
        self.get_pixel_for_index(value.round() as usize)
    }

    fn get_value_for_pixel(&self, pixel: f64) -> f64 {
        // Returns category index as f64
        self.get_index_for_pixel(pixel) as f64
    }

    fn build_ticks(&self, options: &TickOptions) -> Vec<Tick> {
        let mut ticks = Vec::with_capacity(self.labels.len());

        // Apply max_ticks_limit
        let step = if self.labels.len() > options.max_ticks_limit && options.max_ticks_limit > 0 {
            (self.labels.len() as f64 / options.max_ticks_limit as f64).ceil() as usize
        } else {
            1
        };

        for (i, label) in self.labels.iter().enumerate() {
            if i % step == 0 {
                ticks.push(Tick::new(i as f64, label.clone()));
            }
        }

        ticks
    }

    fn get_data_bounds(&self) -> (f64, f64) {
        // Return index range
        (0.0, (self.labels.len().saturating_sub(1)) as f64)
    }

    fn get_pixel_bounds(&self) -> (f64, f64) {
        (self.pixel_start, self.pixel_end)
    }
}

/// Builder for creating category scale from data labels
pub struct CategoryScaleBuilder {
    labels: Vec<String>,
    offset: bool,
}

impl CategoryScaleBuilder {
    pub fn new() -> Self {
        Self {
            labels: Vec::new(),
            offset: true,
        }
    }

    /// Add a single label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.labels.push(label.into());
        self
    }

    /// Add multiple labels
    pub fn labels<I, S>(mut self, labels: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.labels.extend(labels.into_iter().map(|s| s.into()));
        self
    }

    /// Set offset mode
    pub fn offset(mut self, offset: bool) -> Self {
        self.offset = offset;
        self
    }

    /// Build the scale
    pub fn build(self) -> CategoryScale {
        CategoryScale {
            labels: self.labels,
            pixel_start: 0.0,
            pixel_end: 100.0,
            offset: self.offset,
        }
    }
}

impl Default for CategoryScaleBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_scale_basic() {
        let mut scale = CategoryScale::new()
            .with_labels(vec!["A".into(), "B".into(), "C".into(), "D".into()]);
        scale.set_pixel_range(0.0, 400.0);

        assert_eq!(scale.len(), 4);
        assert_eq!(scale.get_band_width(), 100.0);
    }

    #[test]
    fn test_category_scale_with_offset() {
        let mut scale = CategoryScale::new()
            .with_offset(true)
            .with_labels(vec!["A".into(), "B".into(), "C".into(), "D".into()]);
        scale.set_pixel_range(0.0, 400.0);

        // With offset, items are centered in bands
        assert_eq!(scale.get_pixel_for_index(0), 50.0);  // Center of first band
        assert_eq!(scale.get_pixel_for_index(1), 150.0); // Center of second band
        assert_eq!(scale.get_pixel_for_index(2), 250.0); // Center of third band
        assert_eq!(scale.get_pixel_for_index(3), 350.0); // Center of fourth band
    }

    #[test]
    fn test_category_scale_without_offset() {
        let mut scale = CategoryScale::new()
            .with_offset(false)
            .with_labels(vec!["A".into(), "B".into(), "C".into(), "D".into()]);
        scale.set_pixel_range(0.0, 400.0);

        // Without offset, items are on grid lines
        assert_eq!(scale.get_pixel_for_index(0), 0.0);
        assert_eq!(scale.get_pixel_for_index(1), 100.0);
        assert_eq!(scale.get_pixel_for_index(2), 200.0);
        assert_eq!(scale.get_pixel_for_index(3), 300.0);
    }

    #[test]
    fn test_category_scale_tick_generation() {
        let mut scale = CategoryScale::new()
            .with_labels(vec!["Jan".into(), "Feb".into(), "Mar".into(), "Apr".into()]);
        scale.set_pixel_range(0.0, 400.0);

        let options = TickOptions::default();
        let ticks = scale.build_ticks(&options);

        assert_eq!(ticks.len(), 4);
        assert_eq!(ticks[0].label, "Jan");
        assert_eq!(ticks[1].label, "Feb");
        assert_eq!(ticks[2].label, "Mar");
        assert_eq!(ticks[3].label, "Apr");
    }

    #[test]
    fn test_category_scale_builder() {
        let scale = CategoryScaleBuilder::new()
            .labels(["Mon", "Tue", "Wed", "Thu", "Fri"])
            .offset(true)
            .build();

        assert_eq!(scale.len(), 5);
        assert_eq!(scale.get_label(0), Some("Mon"));
        assert_eq!(scale.get_label(4), Some("Fri"));
    }

    #[test]
    fn test_get_index_for_pixel() {
        let mut scale = CategoryScale::new()
            .with_offset(true)
            .with_labels(vec!["A".into(), "B".into(), "C".into(), "D".into()]);
        scale.set_pixel_range(0.0, 400.0);

        // Click near center of each band should return correct index
        assert_eq!(scale.get_index_for_pixel(50.0), 0);
        assert_eq!(scale.get_index_for_pixel(150.0), 1);
        assert_eq!(scale.get_index_for_pixel(250.0), 2);
        assert_eq!(scale.get_index_for_pixel(350.0), 3);
    }

    #[test]
    fn test_bar_width() {
        let mut scale = CategoryScale::new()
            .with_labels(vec!["A".into(), "B".into(), "C".into(), "D".into()]);
        scale.set_pixel_range(0.0, 400.0);

        // Band width is 100, bar at 80% would be 80
        assert_eq!(scale.get_bar_width(0.8), 80.0);
        assert_eq!(scale.get_bar_width(0.5), 50.0);
    }
}
