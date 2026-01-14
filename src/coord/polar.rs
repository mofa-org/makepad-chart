use makepad_widgets::*;
use std::f64::consts::PI;

/// Polar coordinate system for pie/doughnut/radar charts
pub struct PolarCoord {
    /// Center point of the polar coordinate system
    center: DVec2,
    /// Outer radius
    outer_radius: f64,
    /// Inner radius (for doughnut charts)
    inner_radius: f64,
    /// Start angle in radians (0 = 3 o'clock, rotating clockwise)
    start_angle: f64,
    /// Total angle span (2*PI for full circle)
    total_angle: f64,
}

impl PolarCoord {
    /// Create a new polar coordinate system
    pub fn new() -> Self {
        Self {
            center: dvec2(0.0, 0.0),
            outer_radius: 100.0,
            inner_radius: 0.0,
            start_angle: -PI / 2.0, // Start at 12 o'clock
            total_angle: 2.0 * PI,
        }
    }

    /// Set center point
    pub fn with_center(mut self, center: DVec2) -> Self {
        self.center = center;
        self
    }

    /// Set radii
    pub fn with_radii(mut self, inner: f64, outer: f64) -> Self {
        self.inner_radius = inner;
        self.outer_radius = outer;
        self
    }

    /// Set start angle in radians
    pub fn with_start_angle(mut self, angle: f64) -> Self {
        self.start_angle = angle;
        self
    }

    /// Set total angle span
    pub fn with_total_angle(mut self, angle: f64) -> Self {
        self.total_angle = angle;
        self
    }

    /// Update coordinate system to fit within a rect
    pub fn update(&mut self, rect: Rect, padding: f64) {
        self.center = dvec2(
            rect.pos.x + rect.size.x / 2.0,
            rect.pos.y + rect.size.y / 2.0,
        );

        let available_size = rect.size.x.min(rect.size.y) - padding * 2.0;
        self.outer_radius = available_size / 2.0;
    }

    /// Get the center point
    pub fn center(&self) -> DVec2 {
        self.center
    }

    /// Get outer radius
    pub fn outer_radius(&self) -> f64 {
        self.outer_radius
    }

    /// Get inner radius
    pub fn inner_radius(&self) -> f64 {
        self.inner_radius
    }

    /// Set inner radius as a ratio of outer radius (0.0 to 1.0)
    pub fn set_inner_radius_ratio(&mut self, ratio: f64) {
        self.inner_radius = self.outer_radius * ratio.clamp(0.0, 0.99);
    }

    /// Convert normalized value (0-1) to angle
    pub fn value_to_angle(&self, normalized_value: f64) -> f64 {
        self.start_angle + normalized_value * self.total_angle
    }

    /// Convert angle to normalized value (0-1)
    pub fn angle_to_value(&self, angle: f64) -> f64 {
        let mut adjusted = angle - self.start_angle;
        while adjusted < 0.0 {
            adjusted += 2.0 * PI;
        }
        while adjusted > 2.0 * PI {
            adjusted -= 2.0 * PI;
        }
        adjusted / self.total_angle
    }

    /// Convert polar coordinates (angle, radius) to pixel coordinates
    pub fn polar_to_pixel(&self, angle: f64, radius: f64) -> DVec2 {
        dvec2(
            self.center.x + radius * angle.cos(),
            self.center.y + radius * angle.sin(),
        )
    }

    /// Convert pixel coordinates to polar coordinates (angle, radius)
    pub fn pixel_to_polar(&self, pixel: DVec2) -> (f64, f64) {
        let dx = pixel.x - self.center.x;
        let dy = pixel.y - self.center.y;
        let radius = (dx * dx + dy * dy).sqrt();
        let angle = dy.atan2(dx);
        (angle, radius)
    }

    /// Get a point on the outer circle at a given angle
    pub fn get_outer_point(&self, angle: f64) -> DVec2 {
        self.polar_to_pixel(angle, self.outer_radius)
    }

    /// Get a point on the inner circle at a given angle
    pub fn get_inner_point(&self, angle: f64) -> DVec2 {
        self.polar_to_pixel(angle, self.inner_radius)
    }

    /// Get the midpoint between inner and outer radius at a given angle
    pub fn get_mid_point(&self, angle: f64) -> DVec2 {
        let mid_radius = (self.inner_radius + self.outer_radius) / 2.0;
        self.polar_to_pixel(angle, mid_radius)
    }

    /// Check if a point is within the polar area (between inner and outer radius)
    pub fn contains(&self, pixel: DVec2) -> bool {
        let (_, radius) = self.pixel_to_polar(pixel);
        radius >= self.inner_radius && radius <= self.outer_radius
    }

    /// Get arc segment information for a slice
    pub fn get_arc_segment(&self, start_value: f64, end_value: f64) -> ArcSegment {
        let start_angle = self.value_to_angle(start_value);
        let end_angle = self.value_to_angle(end_value);

        ArcSegment {
            center: self.center,
            inner_radius: self.inner_radius,
            outer_radius: self.outer_radius,
            start_angle,
            end_angle,
        }
    }

    /// Get the angle at the middle of a slice
    pub fn get_slice_mid_angle(&self, start_value: f64, end_value: f64) -> f64 {
        let start_angle = self.value_to_angle(start_value);
        let end_angle = self.value_to_angle(end_value);
        (start_angle + end_angle) / 2.0
    }

    /// Get a point suitable for placing a label for a slice
    pub fn get_label_point(&self, start_value: f64, end_value: f64, label_radius_ratio: f64) -> DVec2 {
        let mid_angle = self.get_slice_mid_angle(start_value, end_value);
        let label_radius = self.inner_radius + (self.outer_radius - self.inner_radius) * label_radius_ratio;
        self.polar_to_pixel(mid_angle, label_radius)
    }
}

impl Default for PolarCoord {
    fn default() -> Self {
        Self::new()
    }
}

/// Information about an arc segment (pie slice)
#[derive(Clone, Debug)]
pub struct ArcSegment {
    pub center: DVec2,
    pub inner_radius: f64,
    pub outer_radius: f64,
    pub start_angle: f64,
    pub end_angle: f64,
}

impl ArcSegment {
    /// Get the angle span of this segment
    pub fn angle_span(&self) -> f64 {
        self.end_angle - self.start_angle
    }

    /// Get the middle angle
    pub fn mid_angle(&self) -> f64 {
        (self.start_angle + self.end_angle) / 2.0
    }

    /// Check if a point is within this segment
    pub fn contains(&self, point: DVec2) -> bool {
        let dx = point.x - self.center.x;
        let dy = point.y - self.center.y;
        let radius = (dx * dx + dy * dy).sqrt();

        // Check radius bounds
        if radius < self.inner_radius || radius > self.outer_radius {
            return false;
        }

        // Check angle bounds
        let mut angle = dy.atan2(dx);

        // Normalize angles for comparison
        let mut start = self.start_angle;
        let mut end = self.end_angle;

        // Handle angle wrapping
        while angle < start {
            angle += 2.0 * PI;
        }
        while end < start {
            end += 2.0 * PI;
        }

        angle >= start && angle <= end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polar_coord_basic() {
        let coord = PolarCoord::new()
            .with_center(dvec2(200.0, 150.0))
            .with_radii(0.0, 100.0);

        assert_eq!(coord.center(), dvec2(200.0, 150.0));
        assert_eq!(coord.outer_radius(), 100.0);
        assert_eq!(coord.inner_radius(), 0.0);
    }

    #[test]
    fn test_polar_to_pixel() {
        let coord = PolarCoord::new()
            .with_center(dvec2(100.0, 100.0))
            .with_radii(0.0, 50.0);

        // At angle 0 (3 o'clock), should be to the right of center
        let point = coord.polar_to_pixel(0.0, 50.0);
        assert!((point.x - 150.0).abs() < 0.001);
        assert!((point.y - 100.0).abs() < 0.001);

        // At angle PI/2 (6 o'clock), should be below center
        let point = coord.polar_to_pixel(PI / 2.0, 50.0);
        assert!((point.x - 100.0).abs() < 0.001);
        assert!((point.y - 150.0).abs() < 0.001);
    }

    #[test]
    fn test_value_to_angle() {
        let coord = PolarCoord::new(); // Default starts at -PI/2 (12 o'clock)

        // 0% should be at start angle
        assert!((coord.value_to_angle(0.0) - (-PI / 2.0)).abs() < 0.001);

        // 50% should be at start angle + PI
        assert!((coord.value_to_angle(0.5) - (PI / 2.0)).abs() < 0.001);

        // 100% should be at start angle + 2*PI
        assert!((coord.value_to_angle(1.0) - (3.0 * PI / 2.0)).abs() < 0.001);
    }

    #[test]
    fn test_arc_segment_contains() {
        let segment = ArcSegment {
            center: dvec2(100.0, 100.0),
            inner_radius: 0.0,
            outer_radius: 50.0,
            start_angle: 0.0,
            end_angle: PI / 2.0,
        };

        // Point at center is inside (radius > inner)
        assert!(segment.contains(dvec2(125.0, 100.0))); // To the right, within angle range

        // Point outside radius
        assert!(!segment.contains(dvec2(200.0, 100.0)));

        // Point outside angle range
        assert!(!segment.contains(dvec2(50.0, 100.0))); // To the left
    }

    #[test]
    fn test_doughnut_setup() {
        let mut coord = PolarCoord::new()
            .with_center(dvec2(200.0, 200.0))
            .with_radii(0.0, 100.0);

        coord.set_inner_radius_ratio(0.5);

        assert_eq!(coord.inner_radius(), 50.0);
        assert_eq!(coord.outer_radius(), 100.0);
    }
}
