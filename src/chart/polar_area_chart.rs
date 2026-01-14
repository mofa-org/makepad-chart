use makepad_widgets::*;
use std::f64::consts::PI;
use crate::core::*;
use crate::element::*;
use crate::animation::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::element::arc::DrawArc;

    pub PolarAreaChart = {{PolarAreaChart}} {
        width: Fill,
        height: Fill,
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct PolarAreaChart {
    #[live]
    #[deref]
    view: View,

    #[live]
    draw_arc: DrawArc,

    #[rust]
    data: ChartData,

    #[rust]
    options: ChartOptions,

    #[rust]
    animator: ChartAnimator,

    #[rust]
    initialized: bool,

    #[rust]
    center: DVec2,

    #[rust]
    max_radius: f64,

    #[rust(20.0)]
    padding: f64,

    #[rust(-1)]
    hovered_segment: i32,
}

#[derive(Clone, Debug)]
struct SegmentInfo {
    start_angle: f64,
    end_angle: f64,
    radius_ratio: f64,
    color: Vec4,
    label: String,
}

impl Widget for PolarAreaChart {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        match event {
            Event::MouseMove(e) => {
                self.handle_mouse_move(cx, e.abs);
            }
            Event::NextFrame(_) => {
                if self.animator.is_running() {
                    let time = cx.seconds_since_app_start();
                    if self.animator.update(time) {
                        self.redraw(cx);
                    }
                }
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk_all(cx, scope, walk);

        let rect = cx.turtle().rect();
        if rect.size.x > 0.0 && rect.size.y > 0.0 {
            self.update_layout(rect);

            if !self.initialized {
                self.start_animation(cx);
                self.initialized = true;
            }

            self.draw_segments(cx);
        }

        DrawStep::done()
    }
}

impl PolarAreaChart {
    pub fn set_data(&mut self, data: ChartData) {
        self.data = data;
        self.initialized = false;
    }

    pub fn set_options(&mut self, options: ChartOptions) {
        self.options = options;
    }

    fn update_layout(&mut self, rect: Rect) {
        let size = rect.size.x.min(rect.size.y) - self.padding * 2.0;
        self.max_radius = size / 2.0;
        self.center = dvec2(
            rect.pos.x + rect.size.x / 2.0,
            rect.pos.y + rect.size.y / 2.0,
        );
    }

    fn start_animation(&mut self, cx: &mut Cx) {
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(self.options.animation.duration)
            .with_easing(self.options.animation.easing);
        self.animator.start(time);
        cx.new_next_frame();
    }

    fn compute_segments(&self) -> Vec<SegmentInfo> {
        let mut segments = Vec::new();

        let Some(dataset) = self.data.datasets.first() else {
            return segments;
        };

        let num_segments = dataset.data.len();
        if num_segments == 0 {
            return segments;
        }

        // Find max value for radius scaling
        let max_value = dataset.data.iter()
            .map(|p| p.y.max(0.0))
            .fold(0.0f64, |a, b| a.max(b));

        if max_value == 0.0 {
            return segments;
        }

        // Equal angles for each segment
        let angle_per_segment = 2.0 * PI / num_segments as f64;

        for (i, point) in dataset.data.iter().enumerate() {
            let start_angle = -PI / 2.0 + i as f64 * angle_per_segment;
            let end_angle = start_angle + angle_per_segment;
            let radius_ratio = point.y.max(0.0) / max_value;

            let label = if i < self.data.labels.len() {
                self.data.labels[i].clone()
            } else {
                format!("Segment {}", i + 1)
            };

            segments.push(SegmentInfo {
                start_angle,
                end_angle,
                radius_ratio,
                color: get_color(i),
                label,
            });
        }

        segments
    }

    fn draw_segments(&mut self, cx: &mut Cx2d) {
        let progress = self.animator.get_progress();
        let segments = self.compute_segments();

        for (i, segment) in segments.iter().enumerate() {
            let is_hovered = self.hovered_segment >= 0 && self.hovered_segment as usize == i;

            // Animate radius
            let animated_radius = segment.radius_ratio * progress * self.max_radius;

            if animated_radius < 1.0 {
                continue;
            }

            // Hover effect
            let draw_radius = if is_hovered {
                animated_radius + 5.0
            } else {
                animated_radius
            };

            self.draw_arc.color = if is_hovered {
                lighten(segment.color, 0.1)
            } else {
                segment.color
            };

            let sweep = segment.end_angle - segment.start_angle;
            self.draw_arc.set_arc(segment.start_angle, sweep, 0.0, draw_radius);

            let rect = Rect {
                pos: dvec2(self.center.x - draw_radius, self.center.y - draw_radius),
                size: dvec2(draw_radius * 2.0, draw_radius * 2.0),
            };
            self.draw_arc.draw_arc(cx, rect);
        }
    }

    fn handle_mouse_move(&mut self, cx: &mut Cx, pos: DVec2) {
        let old_hovered = self.hovered_segment;
        self.hovered_segment = -1;

        let dx = pos.x - self.center.x;
        let dy = pos.y - self.center.y;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist > self.max_radius {
            if old_hovered != self.hovered_segment {
                self.redraw(cx);
            }
            return;
        }

        let angle = dy.atan2(dx);
        let segments = self.compute_segments();

        for (i, segment) in segments.iter().enumerate() {
            // Normalize angle to check range
            let mut check_angle = angle;
            let mut start = segment.start_angle;
            let mut end = segment.end_angle;

            // Normalize all to 0-2PI
            while check_angle < 0.0 { check_angle += 2.0 * PI; }
            while start < 0.0 { start += 2.0 * PI; }
            while end < 0.0 { end += 2.0 * PI; }

            if check_angle >= start && check_angle < end {
                let segment_radius = segment.radius_ratio * self.max_radius;
                if dist <= segment_radius {
                    self.hovered_segment = i as i32;
                    break;
                }
            }
        }

        if old_hovered != self.hovered_segment {
            self.redraw(cx);
        }
    }
}

impl PolarAreaChartRef {
    pub fn set_data(&self, data: ChartData) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_data(data);
        }
    }

    pub fn set_options(&self, options: ChartOptions) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_options(options);
        }
    }
}
