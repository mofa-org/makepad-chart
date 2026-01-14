use makepad_widgets::*;

live_design! {
    use link::shaders::*;

    pub DrawBar = {{DrawBar}} {
        fn pixel(self) -> vec4 {
            return vec4(self.color.rgb * self.color.a, self.color.a);
        }
    }
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawBar {
    #[deref] pub draw_super: DrawQuad,
    #[live] pub color: Vec4,
}

impl DrawBar {
    pub fn draw_bar(&mut self, cx: &mut Cx2d, rect: Rect) {
        self.draw_abs(cx, rect);
    }

    pub fn set_top_radius(&mut self, _radius: f32) {
        // Simplified - no rounded corners for now
    }
}
