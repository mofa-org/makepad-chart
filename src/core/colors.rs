use makepad_widgets::*;

/// Default color palette (Chart.js inspired, vibrant colors)
pub const CHART_COLORS: [Vec4; 10] = [
    vec4(0.290, 0.753, 0.753, 1.0),  // Teal      #4ac0c0
    vec4(1.000, 0.388, 0.384, 1.0),  // Coral     #ff6362
    vec4(1.000, 0.808, 0.298, 1.0),  // Gold      #ffce4c
    vec4(0.608, 0.349, 0.714, 1.0),  // Purple    #9b59b6
    vec4(0.204, 0.596, 0.859, 1.0),  // Blue      #3498db
    vec4(0.180, 0.800, 0.443, 1.0),  // Green     #2ecc71
    vec4(0.902, 0.494, 0.133, 1.0),  // Orange    #e67e22
    vec4(0.878, 0.416, 0.604, 1.0),  // Pink      #e06a9a
    vec4(0.447, 0.533, 0.600, 1.0),  // Slate     #728899
    vec4(0.608, 0.678, 0.282, 1.0),  // Olive     #9bad48
];

/// Pastel color palette (softer variant)
pub const PASTEL_COLORS: [Vec4; 10] = [
    vec4(0.647, 0.878, 0.878, 1.0),  // Light Teal
    vec4(1.000, 0.694, 0.692, 1.0),  // Light Coral
    vec4(1.000, 0.906, 0.647, 1.0),  // Light Gold
    vec4(0.804, 0.671, 0.859, 1.0),  // Light Purple
    vec4(0.600, 0.796, 0.929, 1.0),  // Light Blue
    vec4(0.588, 0.900, 0.722, 1.0),  // Light Green
    vec4(0.949, 0.749, 0.569, 1.0),  // Light Orange
    vec4(0.937, 0.706, 0.800, 1.0),  // Light Pink
    vec4(0.722, 0.769, 0.800, 1.0),  // Light Slate
    vec4(0.804, 0.839, 0.639, 1.0),  // Light Olive
];

/// Get color at index (wraps around if index > palette size)
#[inline]
pub fn get_color(index: usize) -> Vec4 {
    CHART_COLORS[index % CHART_COLORS.len()]
}

/// Get color with custom alpha
#[inline]
pub fn get_color_alpha(index: usize, alpha: f32) -> Vec4 {
    let c = get_color(index);
    vec4(c.x, c.y, c.z, alpha)
}

/// Get pastel color at index
#[inline]
pub fn get_pastel_color(index: usize) -> Vec4 {
    PASTEL_COLORS[index % PASTEL_COLORS.len()]
}

/// Get pastel color with custom alpha
#[inline]
pub fn get_pastel_color_alpha(index: usize, alpha: f32) -> Vec4 {
    let c = get_pastel_color(index);
    vec4(c.x, c.y, c.z, alpha)
}

/// Lighten a color by a factor (0.0 - 1.0)
pub fn lighten(color: Vec4, amount: f32) -> Vec4 {
    vec4(
        (color.x + (1.0 - color.x) * amount).min(1.0),
        (color.y + (1.0 - color.y) * amount).min(1.0),
        (color.z + (1.0 - color.z) * amount).min(1.0),
        color.w,
    )
}

/// Darken a color by a factor (0.0 - 1.0)
pub fn darken(color: Vec4, amount: f32) -> Vec4 {
    vec4(
        (color.x * (1.0 - amount)).max(0.0),
        (color.y * (1.0 - amount)).max(0.0),
        (color.z * (1.0 - amount)).max(0.0),
        color.w,
    )
}

/// Set alpha value of a color
pub fn with_alpha(color: Vec4, alpha: f32) -> Vec4 {
    vec4(color.x, color.y, color.z, alpha)
}

/// Convert hex color to Vec4
pub fn hex_to_vec4(hex: u32) -> Vec4 {
    let r = ((hex >> 16) & 0xFF) as f32 / 255.0;
    let g = ((hex >> 8) & 0xFF) as f32 / 255.0;
    let b = (hex & 0xFF) as f32 / 255.0;
    vec4(r, g, b, 1.0)
}

/// Convert hex color with alpha to Vec4
pub fn hex_alpha_to_vec4(hex: u32, alpha: f32) -> Vec4 {
    let r = ((hex >> 16) & 0xFF) as f32 / 255.0;
    let g = ((hex >> 8) & 0xFF) as f32 / 255.0;
    let b = (hex & 0xFF) as f32 / 255.0;
    vec4(r, g, b, alpha)
}

/// Convert RGB values (0-255) to Vec4
pub fn rgb(r: u8, g: u8, b: u8) -> Vec4 {
    vec4(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0)
}

/// Convert RGBA values (0-255, alpha 0.0-1.0) to Vec4
pub fn rgba(r: u8, g: u8, b: u8, a: f32) -> Vec4 {
    vec4(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a)
}

/// Interpolate between two colors
pub fn lerp_color(from: Vec4, to: Vec4, t: f32) -> Vec4 {
    let t = t.clamp(0.0, 1.0);
    vec4(
        from.x + (to.x - from.x) * t,
        from.y + (to.y - from.y) * t,
        from.z + (to.z - from.z) * t,
        from.w + (to.w - from.w) * t,
    )
}

/// Create a color gradient from a list of colors
pub struct ColorGradient {
    colors: Vec<Vec4>,
}

impl ColorGradient {
    /// Create a new gradient from a list of colors
    pub fn new(colors: Vec<Vec4>) -> Self {
        Self { colors }
    }

    /// Create a gradient from the default palette
    pub fn from_palette() -> Self {
        Self {
            colors: CHART_COLORS.to_vec(),
        }
    }

    /// Get color at position t (0.0 - 1.0)
    pub fn at(&self, t: f32) -> Vec4 {
        if self.colors.is_empty() {
            return vec4(0.5, 0.5, 0.5, 1.0);
        }
        if self.colors.len() == 1 {
            return self.colors[0];
        }

        let t = t.clamp(0.0, 1.0);
        let n = self.colors.len() - 1;
        let segment = (t * n as f32).floor() as usize;
        let segment = segment.min(n - 1);
        let local_t = t * n as f32 - segment as f32;

        lerp_color(self.colors[segment], self.colors[segment + 1], local_t)
    }

    /// Generate n colors evenly spaced along the gradient
    pub fn generate(&self, n: usize) -> Vec<Vec4> {
        if n == 0 {
            return Vec::new();
        }
        if n == 1 {
            return vec![self.at(0.5)];
        }

        (0..n)
            .map(|i| self.at(i as f32 / (n - 1) as f32))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_color() {
        let c0 = get_color(0);
        let c1 = get_color(1);
        assert_ne!(c0, c1);

        // Test wrap around
        let c10 = get_color(10);
        assert_eq!(c10, get_color(0));
    }

    #[test]
    fn test_lighten_darken() {
        let color = vec4(0.5, 0.5, 0.5, 1.0);

        let lighter = lighten(color, 0.5);
        assert!(lighter.x > color.x);

        let darker = darken(color, 0.5);
        assert!(darker.x < color.x);
    }

    #[test]
    fn test_hex_to_vec4() {
        let white = hex_to_vec4(0xFFFFFF);
        assert!((white.x - 1.0).abs() < 0.01);
        assert!((white.y - 1.0).abs() < 0.01);
        assert!((white.z - 1.0).abs() < 0.01);

        let black = hex_to_vec4(0x000000);
        assert!((black.x - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_color_gradient() {
        let gradient = ColorGradient::new(vec![
            vec4(1.0, 0.0, 0.0, 1.0), // Red
            vec4(0.0, 1.0, 0.0, 1.0), // Green
            vec4(0.0, 0.0, 1.0, 1.0), // Blue
        ]);

        let start = gradient.at(0.0);
        assert!((start.x - 1.0).abs() < 0.01); // Red

        let end = gradient.at(1.0);
        assert!((end.z - 1.0).abs() < 0.01); // Blue

        let mid = gradient.at(0.5);
        assert!((mid.y - 1.0).abs() < 0.01); // Green
    }
}
