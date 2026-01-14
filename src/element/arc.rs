use makepad_widgets::*;

live_design! {
    use link::shaders::*;

    pub DrawArc = {{DrawArc}} {
        fn pixel(self) -> vec4 {
            let pi_val = 3.14159265;
            let two_pi_val = 6.28318530;

            // Center of the quad is at (0.5, 0.5), map to (-0.5, 0.5)
            let px = self.pos.x - 0.5;
            let py = self.pos.y - 0.5;

            // Calculate distance from center
            let distance = sqrt(px * px + py * py);

            // Radii in normalized space (0.5 = full quad radius)
            let inner_rad = self.inner_radius * 0.5;
            let outer_rad = 0.5;

            // Distance mask: 1.0 if in ring, 0.0 otherwise
            let dist_mask = step(inner_rad, distance) * step(distance, outer_rad);

            // Calculate angle using atan2
            let pixel_ang = atan(py, px);

            // Angle calculation
            let sweep_val = self.end_angle - self.start_angle;
            let rel_ang = pixel_ang - self.start_angle;
            let norm_ang = rel_ang + two_pi_val * 4.0;
            let wrap_ang = mod(norm_ang, two_pi_val);

            // Angle mask: 1.0 if within sweep
            let ang_mask = step(wrap_ang, sweep_val) * step(0.001, sweep_val);

            // Combined mask
            let final_mask = dist_mask * ang_mask;

            // Anti-aliased edges
            let edge_aa = 0.005;
            let outer_aa = 1.0 - smoothstep(outer_rad - edge_aa, outer_rad + edge_aa, distance);
            let inner_aa = smoothstep(inner_rad - edge_aa, inner_rad + edge_aa, distance);
            let aa_alpha = outer_aa * inner_aa;

            let alpha_val = final_mask * aa_alpha;

            // Calculate final color with gradient support
            let final_color = self.color;

            // Gradient mode: 0 = none, 1 = radial (inner to outer), 2 = angular (along arc)
            if self.gradient_enabled > 0.5 {
                if self.gradient_type < 0.5 {
                    // Radial gradient: interpolate from inner to outer radius
                    let ring_width = outer_rad - inner_rad;
                    let t = clamp((distance - inner_rad) / ring_width, 0.0, 1.0);
                    let final_color = mix(self.gradient_inner_color, self.gradient_outer_color, t);
                    return vec4(final_color.rgb * alpha_val, final_color.a * alpha_val);
                } else {
                    // Angular gradient: interpolate along the arc sweep
                    let t = clamp(wrap_ang / sweep_val, 0.0, 1.0);
                    let final_color = mix(self.gradient_inner_color, self.gradient_outer_color, t);
                    return vec4(final_color.rgb * alpha_val, final_color.a * alpha_val);
                }
            }

            return vec4(final_color.rgb * alpha_val, final_color.a * alpha_val);
        }
    }
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawArc {
    #[deref] pub draw_super: DrawQuad,
    #[live] pub color: Vec4,
    #[live] pub start_angle: f32,
    #[live] pub end_angle: f32,
    #[live(0.0)] pub inner_radius: f32,
    #[live(1.0)] pub outer_radius: f32,
    /// Gradient enabled (0.0 = no, 1.0 = yes)
    #[live(0.0)] pub gradient_enabled: f32,
    /// Gradient type (0.0 = radial, 1.0 = angular)
    #[live(0.0)] pub gradient_type: f32,
    /// Inner color for gradient (inner radius or arc start)
    #[live] pub gradient_inner_color: Vec4,
    /// Outer color for gradient (outer radius or arc end)
    #[live] pub gradient_outer_color: Vec4,
}

impl DrawArc {
    pub fn set_arc(&mut self, start: f64, sweep: f64, inner: f64, outer: f64) {
        self.start_angle = start as f32;
        self.end_angle = (start + sweep) as f32;
        // Store inner radius as ratio (0-1) where 1 means same as outer
        if outer > 0.0 {
            self.inner_radius = (inner / outer) as f32;
        } else {
            self.inner_radius = 0.0;
        }
        self.outer_radius = 1.0;
    }

    pub fn draw_arc(&mut self, cx: &mut Cx2d, rect: Rect) {
        self.draw_abs(cx, rect);
    }

    /// Enable radial gradient (inner to outer radius)
    pub fn set_radial_gradient(&mut self, inner_color: Vec4, outer_color: Vec4) {
        self.gradient_enabled = 1.0;
        self.gradient_type = 0.0;
        self.gradient_inner_color = inner_color;
        self.gradient_outer_color = outer_color;
    }

    /// Enable angular gradient (along arc sweep)
    pub fn set_angular_gradient(&mut self, start_color: Vec4, end_color: Vec4) {
        self.gradient_enabled = 1.0;
        self.gradient_type = 1.0;
        self.gradient_inner_color = start_color;
        self.gradient_outer_color = end_color;
    }

    /// Disable gradient (use solid color)
    pub fn disable_gradient(&mut self) {
        self.gradient_enabled = 0.0;
    }
}
