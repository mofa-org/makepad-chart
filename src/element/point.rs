use makepad_widgets::*;

live_design! {
    use link::shaders::*;

    pub DrawPoint = {{DrawPoint}} {
        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            let center = self.rect_size * 0.5;
            let radius = min(center.x, center.y);
            sdf.circle(center.x, center.y, radius);
            sdf.fill(self.color);
            return sdf.result;
        }
    }
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawPoint {
    #[deref] pub draw_super: DrawQuad,
    #[live] pub color: Vec4,
}

impl DrawPoint {
    pub fn draw_point(&mut self, cx: &mut Cx2d, rect: Rect) {
        self.draw_abs(cx, rect);
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
