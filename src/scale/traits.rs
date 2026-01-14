use crate::core::TickOptions;

/// A tick mark on a scale axis
#[derive(Clone, Debug)]
pub struct Tick {
    /// Data value at this tick
    pub value: f64,
    /// Formatted label for display
    pub label: String,
    /// Whether this is a major tick (vs minor)
    pub major: bool,
}

impl Tick {
    /// Create a new major tick
    pub fn new(value: f64, label: impl Into<String>) -> Self {
        Self {
            value,
            label: label.into(),
            major: true,
        }
    }

    /// Create a minor tick
    pub fn minor(value: f64, label: impl Into<String>) -> Self {
        Self {
            value,
            label: label.into(),
            major: false,
        }
    }
}

/// Scale trait for converting between data values and pixel positions
pub trait Scale: Send + Sync {
    /// Get the scale type name
    fn scale_type(&self) -> &'static str;

    /// Set the data value range (min, max)
    fn set_data_range(&mut self, min: f64, max: f64);

    /// Set the pixel position range (start, end)
    fn set_pixel_range(&mut self, start: f64, end: f64);

    /// Convert a data value to pixel position
    fn get_pixel_for_value(&self, value: f64) -> f64;

    /// Convert a pixel position to data value
    fn get_value_for_pixel(&self, pixel: f64) -> f64;

    /// Generate tick marks for this scale
    fn build_ticks(&self, options: &TickOptions) -> Vec<Tick>;

    /// Get the current data bounds
    fn get_data_bounds(&self) -> (f64, f64);

    /// Get the current pixel bounds
    fn get_pixel_bounds(&self) -> (f64, f64);

    /// Check if the scale is inverted (end < start in pixels)
    fn is_inverted(&self) -> bool {
        let (start, end) = self.get_pixel_bounds();
        end < start
    }

    /// Get the pixel range size
    fn get_pixel_size(&self) -> f64 {
        let (start, end) = self.get_pixel_bounds();
        (end - start).abs()
    }

    /// Get the data range size
    fn get_data_size(&self) -> f64 {
        let (min, max) = self.get_data_bounds();
        max - min
    }

    /// Normalize a value to 0-1 range based on data bounds
    fn normalize(&self, value: f64) -> f64 {
        let (min, max) = self.get_data_bounds();
        let range = max - min;
        if range == 0.0 {
            0.5
        } else {
            (value - min) / range
        }
    }

    /// Denormalize a 0-1 value back to data range
    fn denormalize(&self, normalized: f64) -> f64 {
        let (min, max) = self.get_data_bounds();
        min + normalized * (max - min)
    }
}

/// A boxed scale for dynamic dispatch
pub type BoxedScale = Box<dyn Scale>;
