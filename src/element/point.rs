use makepad_widgets::*;

live_design! {
    use link::shaders::*;

    pub DrawPoint = {{DrawPoint}} {
        fn pixel(self) -> vec4 {
            let uv = self.pos;
            let center = vec2(0.5, 0.5);
            let dist = distance(uv, center) * 2.0; // 0 at center, 1 at edge

            // Check if inside circle
            if dist > 1.0 {
                return vec4(0.0, 0.0, 0.0, 0.0);
            }

            // Anti-alias the edge
            let aa = 0.02;
            let alpha = 1.0 - smoothstep(1.0 - aa, 1.0, dist);

            // Calculate color with gradient support
            if self.gradient_enabled > 0.5 {
                // Radial gradient: center color at center, outer color at edge
                let final_color = mix(self.gradient_center_color, self.gradient_outer_color, dist);
                return vec4(final_color.rgb * final_color.a * alpha, final_color.a * alpha);
            }

            return vec4(self.color.rgb * self.color.a * alpha, self.color.a * alpha);
        }
    }
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawPoint {
    #[deref] pub draw_super: DrawQuad,
    #[live] pub color: Vec4,
    /// Gradient enabled (0.0 = no, 1.0 = yes)
    #[live(0.0)] pub gradient_enabled: f32,
    /// Center color for radial gradient
    #[live] pub gradient_center_color: Vec4,
    /// Outer color for radial gradient
    #[live] pub gradient_outer_color: Vec4,
}

impl DrawPoint {
    pub fn draw_point(&mut self, cx: &mut Cx2d, rect: Rect) {
        self.draw_abs(cx, rect);
    }

    /// Enable radial gradient (center to edge)
    pub fn set_radial_gradient(&mut self, center_color: Vec4, outer_color: Vec4) {
        self.gradient_enabled = 1.0;
        self.gradient_center_color = center_color;
        self.gradient_outer_color = outer_color;
    }

    /// Disable gradient (use solid color)
    pub fn disable_gradient(&mut self) {
        self.gradient_enabled = 0.0;
    }
}

/// Point styles for scatter charts
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum PointStyle {
    #[default]
    Circle,
    Square,
    Triangle,
    Diamond,
    Cross,
}
