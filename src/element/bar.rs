use makepad_widgets::*;

live_design! {
    use link::shaders::*;

    pub DrawBar = {{DrawBar}} {
        fn pixel(self) -> vec4 {
            let uv = self.pos;

            // Calculate final color with gradient support
            if self.gradient_enabled > 0.5 {
                // Vertical gradient: bottom color at bottom, top color at top
                // Note: UV y=0 is top, y=1 is bottom in screen space
                let t = 1.0 - uv.y; // Invert so gradient goes bottom to top
                let final_color = mix(self.gradient_bottom_color, self.gradient_top_color, t);
                return vec4(final_color.rgb * final_color.a, final_color.a);
            }

            return vec4(self.color.rgb * self.color.a, self.color.a);
        }
    }
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawBar {
    #[deref] pub draw_super: DrawQuad,
    #[live] pub color: Vec4,
    /// Gradient enabled (0.0 = no, 1.0 = yes)
    #[live(0.0)] pub gradient_enabled: f32,
    /// Bottom color for vertical gradient
    #[live] pub gradient_bottom_color: Vec4,
    /// Top color for vertical gradient
    #[live] pub gradient_top_color: Vec4,
}

impl DrawBar {
    pub fn draw_bar(&mut self, cx: &mut Cx2d, rect: Rect) {
        self.draw_abs(cx, rect);
    }

    pub fn set_top_radius(&mut self, _radius: f32) {
        // Simplified - no rounded corners for now
    }

    /// Enable vertical gradient (bottom to top)
    pub fn set_vertical_gradient(&mut self, bottom_color: Vec4, top_color: Vec4) {
        self.gradient_enabled = 1.0;
        self.gradient_bottom_color = bottom_color;
        self.gradient_top_color = top_color;
    }

    /// Disable gradient (use solid color)
    pub fn disable_gradient(&mut self) {
        self.gradient_enabled = 0.0;
    }
}
