use makepad_widgets::*;
use std::f64::consts::PI;
use crate::core::*;
use crate::element::*;
use crate::animation::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::element::line::DrawChartLine;
    use crate::element::point::DrawPoint;
    use crate::element::triangle::DrawTriangle;

    pub RadarChart = {{RadarChart}} {
        width: Fill,
        height: Fill,
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct RadarChart {
    #[live]
    #[deref]
    view: View,

    #[live]
    draw_line: DrawChartLine,

    #[live]
    draw_point: DrawPoint,

    #[live]
    draw_grid: DrawChartLine,

    #[live]
    draw_fill: DrawTriangle,

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
    radius: f64,

    #[rust(20.0)]
    padding: f64,

    #[rust]
    grid_levels: usize,

    #[rust(true)]
    show_grid: bool,

    #[rust(true)]
    show_points: bool,

    #[rust(true)]
    show_fill: bool,

    #[rust(4.0)]
    point_radius: f64,

    #[rust(0.3)]
    fill_opacity: f64,

    /// Enable gradient fill (radial from center to edges)
    #[rust(false)]
    gradient_enabled: bool,
}

impl Widget for RadarChart {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if let Event::NextFrame(_) = event {
            if self.animator.is_running() {
                let time = cx.seconds_since_app_start();
                if self.animator.update(time) {
                    self.redraw(cx);
                }
            }
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

            if self.show_grid {
                self.draw_grid_lines(cx);
            }
            self.draw_datasets(cx);
        }

        DrawStep::done()
    }
}

impl RadarChart {
    pub fn set_data(&mut self, data: ChartData) {
        self.data = data;
        self.initialized = false;
    }

    pub fn set_options(&mut self, options: ChartOptions) {
        self.options = options;
    }

    pub fn set_fill(&mut self, show_fill: bool) {
        self.show_fill = show_fill;
    }

    pub fn set_fill_opacity(&mut self, opacity: f64) {
        self.fill_opacity = opacity.clamp(0.0, 1.0);
    }

    /// Enable radial gradient fill (from center to edges)
    pub fn set_gradient(&mut self, enabled: bool) {
        self.gradient_enabled = enabled;
    }

    fn update_layout(&mut self, rect: Rect) {
        let size = rect.size.x.min(rect.size.y) - self.padding * 2.0;
        self.radius = size / 2.0;
        self.center = dvec2(
            rect.pos.x + rect.size.x / 2.0,
            rect.pos.y + rect.size.y / 2.0,
        );
        if self.grid_levels == 0 {
            self.grid_levels = 5;
        }
    }

    fn start_animation(&mut self, cx: &mut Cx) {
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(self.options.animation.duration)
            .with_easing(self.options.animation.easing);
        self.animator.start(time);
        cx.new_next_frame();
    }

    fn get_num_axes(&self) -> usize {
        self.data.labels.len().max(
            self.data.datasets.first()
                .map(|d| d.data.len())
                .unwrap_or(0)
        )
    }

    fn get_angle(&self, index: usize, total: usize) -> f64 {
        -PI / 2.0 + (index as f64 / total as f64) * 2.0 * PI
    }

    fn get_point(&self, angle: f64, value: f64, max_value: f64) -> DVec2 {
        let normalized = if max_value > 0.0 { value / max_value } else { 0.0 };
        let dist = normalized * self.radius;
        dvec2(
            self.center.x + dist * angle.cos(),
            self.center.y + dist * angle.sin(),
        )
    }

    fn draw_grid_lines(&mut self, cx: &mut Cx2d) {
        let num_axes = self.get_num_axes();
        if num_axes < 3 {
            return;
        }

        self.draw_grid.color = vec4(0.8, 0.8, 0.8, 1.0);

        // Draw concentric polygons
        for level in 1..=self.grid_levels {
            let ratio = level as f64 / self.grid_levels as f64;

            for i in 0..num_axes {
                let angle1 = self.get_angle(i, num_axes);
                let angle2 = self.get_angle((i + 1) % num_axes, num_axes);

                let p1 = dvec2(
                    self.center.x + self.radius * ratio * angle1.cos(),
                    self.center.y + self.radius * ratio * angle1.sin(),
                );
                let p2 = dvec2(
                    self.center.x + self.radius * ratio * angle2.cos(),
                    self.center.y + self.radius * ratio * angle2.sin(),
                );

                self.draw_grid.draw_line(cx, p1, p2, 1.0);
            }
        }

        // Draw axis lines from center
        for i in 0..num_axes {
            let angle = self.get_angle(i, num_axes);
            let outer = dvec2(
                self.center.x + self.radius * angle.cos(),
                self.center.y + self.radius * angle.sin(),
            );
            self.draw_grid.draw_line(cx, self.center, outer, 1.0);
        }
    }

    fn draw_datasets(&mut self, cx: &mut Cx2d) {
        let progress = self.animator.get_progress();
        let num_axes = self.get_num_axes();

        if num_axes < 3 {
            return;
        }

        // Find max value across all datasets
        let max_value = self.data.datasets.iter()
            .flat_map(|d| d.data.iter())
            .map(|p| p.y)
            .fold(0.0f64, |a, b| a.max(b));

        for (dataset_idx, dataset) in self.data.datasets.iter().enumerate() {
            let color = dataset.border_color
                .or(dataset.background_color)
                .unwrap_or_else(|| get_color(dataset_idx));

            // Collect points
            let points: Vec<DVec2> = (0..num_axes).map(|i| {
                let value = dataset.data.get(i).map(|p| p.y).unwrap_or(0.0);
                let animated_value = value * progress;
                let angle = self.get_angle(i, num_axes);
                self.get_point(angle, animated_value, max_value)
            }).collect();

            // Draw fill first (behind lines)
            if self.show_fill {
                let fill_color = vec4(
                    color.x,
                    color.y,
                    color.z,
                    self.fill_opacity as f32
                );
                self.draw_fill.color = fill_color;

                // Apply gradient if enabled
                if self.gradient_enabled {
                    // Center is lighter, edges are the base color
                    let center_color = vec4(
                        (color.x + (1.0 - color.x) * 0.5).min(1.0),
                        (color.y + (1.0 - color.y) * 0.5).min(1.0),
                        (color.z + (1.0 - color.z) * 0.5).min(1.0),
                        self.fill_opacity as f32,
                    );
                    let outer_color = vec4(
                        color.x * 0.8,
                        color.y * 0.8,
                        color.z * 0.8,
                        self.fill_opacity as f32 * 0.6,
                    );
                    self.draw_fill.set_radial_gradient(center_color, outer_color);
                } else {
                    self.draw_fill.disable_gradient();
                }

                // Draw triangles from center to each edge
                for i in 0..points.len() {
                    let p1 = points[i];
                    let p2 = points[(i + 1) % points.len()];
                    self.draw_fill.draw_triangle(cx, self.center, p1, p2);
                }
            }

            // Draw polygon lines
            self.draw_line.color = color;
            for i in 0..points.len() {
                let p1 = points[i];
                let p2 = points[(i + 1) % points.len()];
                self.draw_line.draw_line(cx, p1, p2, 2.0);
            }

            // Draw points
            if self.show_points {
                self.draw_point.color = color;
                for point in &points {
                    let rect = Rect {
                        pos: dvec2(point.x - self.point_radius, point.y - self.point_radius),
                        size: dvec2(self.point_radius * 2.0, self.point_radius * 2.0),
                    };
                    self.draw_point.draw_point(cx, rect);
                }
            }
        }
    }
}

impl RadarChartRef {
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

    pub fn set_fill(&self, show_fill: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_fill(show_fill);
        }
    }

    pub fn set_gradient(&self, enabled: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_gradient(enabled);
        }
    }
}
