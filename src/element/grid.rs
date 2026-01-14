use makepad_widgets::*;

live_design! {
    use link::shaders::*;

    pub DrawGridLine = {{DrawGridLine}} {
        fn pixel(self) -> vec4 {
            let uv = self.pos;

            // Line endpoints in normalized coordinates (0-1)
            let p1 = vec2(self.x1, self.y1);
            let p2 = vec2(self.x2, self.y2);

            // Current pixel position
            let p = uv;

            // Vector from p1 to p2
            let line_vec = p2 - p1;
            let line_len = length(line_vec);

            if line_len < 0.001 {
                return vec4(0.0, 0.0, 0.0, 0.0);
            }

            // Project point onto line
            let t = clamp(dot(p - p1, line_vec) / (line_len * line_len), 0.0, 1.0);
            let closest = p1 + t * line_vec;

            // Distance from point to line
            let dist = length(p - closest);

            // Line width in normalized coordinates
            let half_width = self.line_width * 0.5;

            // Anti-aliased edge
            let aa = 0.01;
            let alpha = 1.0 - smoothstep(half_width - aa, half_width + aa, dist);

            if alpha < 0.01 {
                return vec4(0.0, 0.0, 0.0, 0.0);
            }

            return vec4(self.color.rgb * self.color.a * alpha, self.color.a * alpha);
        }
    }
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawGridLine {
    #[deref] pub draw_super: DrawQuad,
    #[live] pub color: Vec4,
    #[live] pub x1: f32,
    #[live] pub y1: f32,
    #[live] pub x2: f32,
    #[live] pub y2: f32,
    #[live] pub line_width: f32,
}

impl DrawGridLine {
    pub fn draw_line(&mut self, cx: &mut Cx2d, p1: DVec2, p2: DVec2, width: f64) {
        // Calculate bounding box with padding for line width
        let padding = width * 2.0;
        let min_x = p1.x.min(p2.x) - padding;
        let max_x = p1.x.max(p2.x) + padding;
        let min_y = p1.y.min(p2.y) - padding;
        let max_y = p1.y.max(p2.y) + padding;

        let rect_width = max_x - min_x;
        let rect_height = max_y - min_y;

        if rect_width < 1.0 || rect_height < 1.0 {
            return;
        }

        // Convert absolute positions to normalized (0-1) within bounding box
        self.x1 = ((p1.x - min_x) / rect_width) as f32;
        self.y1 = ((p1.y - min_y) / rect_height) as f32;
        self.x2 = ((p2.x - min_x) / rect_width) as f32;
        self.y2 = ((p2.y - min_y) / rect_height) as f32;

        // Line width relative to smaller dimension
        let min_dim = rect_width.min(rect_height);
        self.line_width = (width / min_dim) as f32;

        let rect = Rect {
            pos: dvec2(min_x, min_y),
            size: dvec2(rect_width, rect_height),
        };

        self.draw_abs(cx, rect);
    }
}
