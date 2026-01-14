use makepad_widgets::*;

live_design! {
    use link::shaders::*;

    pub DrawTriangle = {{DrawTriangle}} {
        fn pixel(self) -> vec4 {
            // Triangle vertices in normalized coordinates (0-1)
            let v0 = vec2(self.v0x, self.v0y);
            let v1 = vec2(self.v1x, self.v1y);
            let v2 = vec2(self.v2x, self.v2y);

            let p = self.pos;

            // Compute barycentric coordinates
            let d00 = dot(v1 - v0, v1 - v0);
            let d01 = dot(v1 - v0, v2 - v0);
            let d11 = dot(v2 - v0, v2 - v0);
            let d20 = dot(p - v0, v1 - v0);
            let d21 = dot(p - v0, v2 - v0);

            let denom = d00 * d11 - d01 * d01;
            if abs(denom) < 0.0001 {
                return vec4(0.0, 0.0, 0.0, 0.0);
            }

            let inv_denom = 1.0 / denom;
            let u = (d11 * d20 - d01 * d21) * inv_denom;
            let v = (d00 * d21 - d01 * d20) * inv_denom;

            // Check if point is inside triangle
            if u >= 0.0 && v >= 0.0 && (u + v) <= 1.0 {
                // Calculate final color with gradient support
                if self.gradient_enabled > 0.5 {
                    // Radial gradient: v0 is center, v1/v2 are edges
                    // Barycentric weight at v0 is (1 - u - v)
                    let center_weight = 1.0 - u - v;
                    let final_color = mix(self.gradient_outer_color, self.gradient_center_color, center_weight);
                    return vec4(final_color.rgb * final_color.a, final_color.a);
                }
                return vec4(self.color.rgb * self.color.a, self.color.a);
            }

            return vec4(0.0, 0.0, 0.0, 0.0);
        }
    }
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawTriangle {
    #[deref] pub draw_super: DrawQuad,
    #[live] pub color: Vec4,
    #[live] pub v0x: f32,
    #[live] pub v0y: f32,
    #[live] pub v1x: f32,
    #[live] pub v1y: f32,
    #[live] pub v2x: f32,
    #[live] pub v2y: f32,
    /// Gradient enabled (0.0 = no, 1.0 = yes)
    #[live(0.0)] pub gradient_enabled: f32,
    /// Center color for radial gradient (at v0)
    #[live] pub gradient_center_color: Vec4,
    /// Outer color for radial gradient (at v1, v2)
    #[live] pub gradient_outer_color: Vec4,
}

impl DrawTriangle {
    pub fn draw_triangle(&mut self, cx: &mut Cx2d, p0: DVec2, p1: DVec2, p2: DVec2) {
        // Calculate bounding box
        let min_x = p0.x.min(p1.x).min(p2.x);
        let max_x = p0.x.max(p1.x).max(p2.x);
        let min_y = p0.y.min(p1.y).min(p2.y);
        let max_y = p0.y.max(p1.y).max(p2.y);

        let width = max_x - min_x;
        let height = max_y - min_y;

        if width < 1.0 || height < 1.0 {
            return;
        }

        // Convert to normalized coordinates within bounding box
        self.v0x = ((p0.x - min_x) / width) as f32;
        self.v0y = ((p0.y - min_y) / height) as f32;
        self.v1x = ((p1.x - min_x) / width) as f32;
        self.v1y = ((p1.y - min_y) / height) as f32;
        self.v2x = ((p2.x - min_x) / width) as f32;
        self.v2y = ((p2.y - min_y) / height) as f32;

        let rect = Rect {
            pos: dvec2(min_x, min_y),
            size: dvec2(width, height),
        };

        self.draw_abs(cx, rect);
    }

    /// Enable radial gradient (center color at v0, outer color at v1/v2)
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
