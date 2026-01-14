use super::traits::{Scale, Tick};
use super::utils::{nice_step, nice_bounds, format_number};
use crate::core::TickOptions;

/// Linear scale for continuous numeric data
#[derive(Clone, Debug)]
pub struct LinearScale {
    // Data range
    data_min: f64,
    data_max: f64,

    // Pixel range
    pixel_start: f64,
    pixel_end: f64,

    // Options
    begin_at_zero: bool,
    nice: bool,
    clamp: bool,
}

impl LinearScale {
    /// Create a new linear scale
    pub fn new() -> Self {
        Self {
            data_min: 0.0,
            data_max: 1.0,
            pixel_start: 0.0,
            pixel_end: 100.0,
            begin_at_zero: false,
            nice: true,
            clamp: false,
        }
    }

    /// Set whether to extend the scale to start at zero
    pub fn with_begin_at_zero(mut self, begin_at_zero: bool) -> Self {
        self.begin_at_zero = begin_at_zero;
        self
    }

    /// Set whether to use nice bounds
    pub fn with_nice(mut self, nice: bool) -> Self {
        self.nice = nice;
        self
    }

    /// Set whether to clamp values outside the range
    pub fn with_clamp(mut self, clamp: bool) -> Self {
        self.clamp = clamp;
        self
    }

    /// Create with specific data range
    pub fn with_range(mut self, min: f64, max: f64) -> Self {
        self.set_data_range(min, max);
        self
    }

    /// Get the ratio (0-1) for a data value
    fn get_ratio(&self, value: f64) -> f64 {
        let range = self.data_max - self.data_min;
        if range == 0.0 {
            return 0.5;
        }

        let ratio = (value - self.data_min) / range;

        if self.clamp {
            ratio.clamp(0.0, 1.0)
        } else {
            ratio
        }
    }

    /// Get the data value for a ratio (0-1)
    fn get_value_for_ratio(&self, ratio: f64) -> f64 {
        self.data_min + ratio * (self.data_max - self.data_min)
    }
}

impl Default for LinearScale {
    fn default() -> Self {
        Self::new()
    }
}

impl Scale for LinearScale {
    fn scale_type(&self) -> &'static str {
        "linear"
    }

    fn set_data_range(&mut self, min: f64, max: f64) {
        let mut min = min;
        let mut max = max;

        // Handle begin at zero FIRST (before nice bounds)
        // This ensures y=0 aligns with chart_area.bottom for bar charts
        if self.begin_at_zero {
            if min > 0.0 {
                min = 0.0;
            }
            if max < 0.0 {
                max = 0.0;
            }
        }

        // Apply nice bounds AFTER begin_at_zero adjustment
        if self.nice {
            let (nice_min, nice_max) = nice_bounds(min, max);
            // Only use nice_min if begin_at_zero is not forcing 0
            // Otherwise keep min at 0 to ensure bars align with axis
            if self.begin_at_zero && min == 0.0 {
                max = nice_max;
            } else {
                min = nice_min;
                max = nice_max;
            }
        }

        // Ensure we have a valid range
        if (max - min).abs() < f64::EPSILON {
            min -= 1.0;
            max += 1.0;
        }

        self.data_min = min;
        self.data_max = max;
    }

    fn set_pixel_range(&mut self, start: f64, end: f64) {
        self.pixel_start = start;
        self.pixel_end = end;
    }

    fn get_pixel_for_value(&self, value: f64) -> f64 {
        let ratio = self.get_ratio(value);
        self.pixel_start + ratio * (self.pixel_end - self.pixel_start)
    }

    fn get_value_for_pixel(&self, pixel: f64) -> f64 {
        let pixel_range = self.pixel_end - self.pixel_start;
        if pixel_range.abs() < f64::EPSILON {
            return self.data_min;
        }

        let ratio = (pixel - self.pixel_start) / pixel_range;
        self.get_value_for_ratio(ratio)
    }

    fn build_ticks(&self, options: &TickOptions) -> Vec<Tick> {
        let span = self.data_max - self.data_min;

        // Determine step size
        let step = if let Some(step) = options.step_size {
            step
        } else {
            nice_step(span, options.max_ticks_limit)
        };

        // Calculate starting tick value
        let start = (self.data_min / step).ceil() * step;

        let mut ticks = Vec::new();

        // Add min bound if requested
        if options.include_bounds && start > self.data_min {
            ticks.push(Tick::new(self.data_min, format_number(self.data_min)));
        }

        // Generate ticks
        let mut value = start;
        let epsilon = step * 0.0001;

        while value <= self.data_max + epsilon {
            // Skip if too close to an existing tick
            let skip = ticks.last().map(|t| (t.value - value).abs() < epsilon).unwrap_or(false);

            if !skip {
                ticks.push(Tick::new(value, format_number(value)));
            }
            value += step;
        }

        // Add max bound if requested
        if options.include_bounds {
            let last_value = ticks.last().map(|t| t.value).unwrap_or(f64::MIN);
            if (self.data_max - last_value).abs() > epsilon {
                ticks.push(Tick::new(self.data_max, format_number(self.data_max)));
            }
        }

        ticks
    }

    fn get_data_bounds(&self) -> (f64, f64) {
        (self.data_min, self.data_max)
    }

    fn get_pixel_bounds(&self) -> (f64, f64) {
        (self.pixel_start, self.pixel_end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_scale_mapping() {
        let mut scale = LinearScale::new().with_nice(false);
        scale.set_data_range(0.0, 100.0);
        scale.set_pixel_range(0.0, 500.0);

        assert_eq!(scale.get_pixel_for_value(0.0), 0.0);
        assert_eq!(scale.get_pixel_for_value(50.0), 250.0);
        assert_eq!(scale.get_pixel_for_value(100.0), 500.0);
    }

    #[test]
    fn test_linear_scale_inverse() {
        let mut scale = LinearScale::new().with_nice(false);
        scale.set_data_range(0.0, 100.0);
        scale.set_pixel_range(0.0, 500.0);

        assert_eq!(scale.get_value_for_pixel(0.0), 0.0);
        assert_eq!(scale.get_value_for_pixel(250.0), 50.0);
        assert_eq!(scale.get_value_for_pixel(500.0), 100.0);
    }

    #[test]
    fn test_begin_at_zero() {
        let mut scale = LinearScale::new()
            .with_begin_at_zero(true)
            .with_nice(false);
        scale.set_data_range(10.0, 100.0);

        let (min, _max) = scale.get_data_bounds();
        assert_eq!(min, 0.0);
    }

    #[test]
    fn test_inverted_pixel_range() {
        let mut scale = LinearScale::new().with_nice(false);
        scale.set_data_range(0.0, 100.0);
        // Inverted for Y axis (0 at bottom, 500 at top)
        scale.set_pixel_range(500.0, 0.0);

        assert_eq!(scale.get_pixel_for_value(0.0), 500.0);
        assert_eq!(scale.get_pixel_for_value(100.0), 0.0);
        assert!(scale.is_inverted());
    }

    #[test]
    fn test_tick_generation() {
        let mut scale = LinearScale::new();
        scale.set_data_range(0.0, 100.0);
        scale.set_pixel_range(0.0, 500.0);

        let options = TickOptions::default();
        let ticks = scale.build_ticks(&options);

        assert!(!ticks.is_empty());
        // First tick should be at or near 0
        assert!(ticks[0].value <= 0.0);
        // Last tick should be at or near 100
        assert!(ticks.last().unwrap().value >= 100.0);
    }
}
