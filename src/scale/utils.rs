/// Calculate a "nice" step value for tick generation
///
/// This algorithm finds a step size that produces round, human-friendly
/// tick values (like 1, 2, 5, 10, 20, 50, etc.)
pub fn nice_step(span: f64, max_ticks: usize) -> f64 {
    if span == 0.0 || max_ticks == 0 {
        return 1.0;
    }

    let raw_step = span / max_ticks as f64;
    let magnitude = 10f64.powf(raw_step.log10().floor());
    let residual = raw_step / magnitude;

    let nice = if residual <= 1.0 {
        1.0
    } else if residual <= 2.0 {
        2.0
    } else if residual <= 5.0 {
        5.0
    } else {
        10.0
    };

    nice * magnitude
}

/// Calculate nice bounds for a range
///
/// Expands the range to nice round numbers suitable for axis bounds
pub fn nice_bounds(min: f64, max: f64) -> (f64, f64) {
    let span = max - min;

    if span == 0.0 {
        return (min - 1.0, max + 1.0);
    }

    let magnitude = 10f64.powf(span.log10().floor());
    let nice_min = (min / magnitude).floor() * magnitude;
    let nice_max = (max / magnitude).ceil() * magnitude;

    (nice_min, nice_max)
}

/// Format a number for display as a tick label
pub fn format_number(value: f64) -> String {
    // Handle very small numbers
    if value.abs() < 1e-10 {
        return "0".to_string();
    }

    // Format large numbers with suffixes
    if value.abs() >= 1_000_000_000.0 {
        format!("{:.1}B", value / 1_000_000_000.0)
    } else if value.abs() >= 1_000_000.0 {
        format!("{:.1}M", value / 1_000_000.0)
    } else if value.abs() >= 1_000.0 {
        format!("{:.1}K", value / 1_000.0)
    } else if value.fract().abs() < 1e-10 {
        // Integer value
        format!("{:.0}", value)
    } else if value.abs() < 0.01 {
        // Very small decimal
        format!("{:.4}", value)
    } else if value.abs() < 1.0 {
        // Small decimal
        format!("{:.2}", value)
    } else {
        // Normal decimal
        let formatted = format!("{:.2}", value);
        // Remove trailing zeros after decimal
        formatted.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}

/// Format a number with a specific number of decimal places
pub fn format_number_precision(value: f64, precision: usize) -> String {
    format!("{:.1$}", value, precision)
}

/// Format a percentage (0-1 range to 0-100%)
pub fn format_percent(value: f64) -> String {
    format!("{:.1}%", value * 100.0)
}

/// Format a percentage with custom precision
pub fn format_percent_precision(value: f64, precision: usize) -> String {
    format!("{:.1$}%", value * 100.0, precision)
}

/// Calculate the number of decimal places needed for a step size
pub fn decimal_places_for_step(step: f64) -> usize {
    if step >= 1.0 {
        0
    } else {
        let log = -step.log10().floor() as usize;
        log.min(10)
    }
}

/// Linear interpolation
#[inline]
pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

/// Inverse linear interpolation (find t given a, b, and value)
#[inline]
pub fn inverse_lerp(a: f64, b: f64, value: f64) -> f64 {
    if (b - a).abs() < f64::EPSILON {
        0.0
    } else {
        (value - a) / (b - a)
    }
}

/// Clamp a value between min and max
#[inline]
pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    value.max(min).min(max)
}

/// Map a value from one range to another
pub fn map_range(value: f64, from_min: f64, from_max: f64, to_min: f64, to_max: f64) -> f64 {
    let t = inverse_lerp(from_min, from_max, value);
    lerp(to_min, to_max, t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nice_step() {
        assert_eq!(nice_step(100.0, 10), 10.0);
        assert_eq!(nice_step(95.0, 10), 10.0);
        assert_eq!(nice_step(23.0, 5), 5.0);
        assert_eq!(nice_step(0.7, 7), 0.1);
    }

    #[test]
    fn test_nice_bounds() {
        let (min, max) = nice_bounds(3.2, 97.8);
        assert!(min <= 3.2);
        assert!(max >= 97.8);
        // Should be nice round numbers
        assert_eq!(min, 0.0);
        assert_eq!(max, 100.0);
    }

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(0.0), "0");
        assert_eq!(format_number(1.0), "1");
        assert_eq!(format_number(10.0), "10");
        assert_eq!(format_number(100.0), "100");
        assert_eq!(format_number(1000.0), "1.0K");
        assert_eq!(format_number(1500.0), "1.5K");
        assert_eq!(format_number(1000000.0), "1.0M");
        assert_eq!(format_number(0.5), "0.50");
    }

    #[test]
    fn test_lerp() {
        assert_eq!(lerp(0.0, 100.0, 0.0), 0.0);
        assert_eq!(lerp(0.0, 100.0, 0.5), 50.0);
        assert_eq!(lerp(0.0, 100.0, 1.0), 100.0);
    }

    #[test]
    fn test_inverse_lerp() {
        assert_eq!(inverse_lerp(0.0, 100.0, 0.0), 0.0);
        assert_eq!(inverse_lerp(0.0, 100.0, 50.0), 0.5);
        assert_eq!(inverse_lerp(0.0, 100.0, 100.0), 1.0);
    }

    #[test]
    fn test_map_range() {
        assert_eq!(map_range(5.0, 0.0, 10.0, 0.0, 100.0), 50.0);
        assert_eq!(map_range(0.0, 0.0, 10.0, 100.0, 200.0), 100.0);
    }
}
