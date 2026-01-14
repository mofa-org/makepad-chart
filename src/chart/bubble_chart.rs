use makepad_widgets::*;
use crate::core::*;
use crate::coord::*;
use crate::scale::*;
use crate::element::*;
use crate::animation::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::element::point::DrawPoint;
    use crate::element::grid::DrawGridLine;

    pub BubbleChart = {{BubbleChart}} {
        width: Fill,
        height: Fill,
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct BubbleChart {
    #[live]
    #[deref]
    view: View,

    #[live]
    draw_point: DrawPoint,

    #[live]
    draw_grid: DrawGridLine,

    #[rust]
    data: ChartData,

    #[rust]
    options: ChartOptions,

    #[rust]
    coord: CartesianCoord,

    #[rust]
    animator: ChartAnimator,

    #[rust]
    initialized: bool,

    #[rust(5.0)]
    min_radius: f64,

    #[rust(40.0)]
    max_radius: f64,

    #[rust(-1)]
    hovered_bubble: i32,

    #[rust(-1)]
    hovered_dataset: i32,
}

impl Widget for BubbleChart {
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
            self.update_coord(rect);

            if !self.initialized {
                self.start_animation(cx);
                self.initialized = true;
            }

            self.draw_grid_lines(cx);
            self.draw_axes(cx);
            self.draw_bubbles(cx);
        }

        DrawStep::done()
    }
}

impl BubbleChart {
    pub fn set_data(&mut self, data: ChartData) {
        self.data = data;
        self.initialized = false;
        self.setup_coord_from_data();
    }

    pub fn set_options(&mut self, options: ChartOptions) {
        self.options = options;
        // Re-setup coordinate system with new options
        if !self.data.datasets.is_empty() {
            self.setup_coord_from_data();
        }
    }

    pub fn set_radius_range(&mut self, min: f64, max: f64) {
        self.min_radius = min;
        self.max_radius = max;
    }

    fn setup_coord_from_data(&mut self) {
        self.coord = CartesianCoord::new()
            .with_x_scale(ScaleType::Linear(
                LinearScale::new()
                    .with_begin_at_zero(self.options.scales.x.begin_at_zero)
                    .with_nice(true)
            ))
            .with_y_scale(ScaleType::Linear(
                LinearScale::new()
                    .with_begin_at_zero(self.options.scales.y.begin_at_zero)
                    .with_nice(true)
            ));

        // Calculate data ranges
        let mut x_min = f64::MAX;
        let mut x_max = f64::MIN;
        let mut y_min = f64::MAX;
        let mut y_max = f64::MIN;

        for dataset in &self.data.datasets {
            for (idx, point) in dataset.data.iter().enumerate() {
                let x = point.x.unwrap_or(idx as f64);
                x_min = x_min.min(x);
                x_max = x_max.max(x);
                y_min = y_min.min(point.y);
                y_max = y_max.max(point.y);
            }
        }

        if x_min < x_max {
            let x_min = if self.options.scales.x.begin_at_zero && x_min > 0.0 { 0.0 } else { x_min };
            self.coord.set_x_data_range(x_min, x_max);
        }
        if y_min < y_max {
            let y_min = if self.options.scales.y.begin_at_zero && y_min > 0.0 { 0.0 } else { y_min };
            self.coord.set_y_data_range(y_min, y_max);
        }
    }

    fn update_coord(&mut self, rect: Rect) {
        self.coord.update(rect);
    }

    fn start_animation(&mut self, cx: &mut Cx) {
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(self.options.animation.duration)
            .with_easing(self.options.animation.easing);
        self.animator.start(time);
        cx.new_next_frame();
    }

    fn draw_grid_lines(&mut self, cx: &mut Cx2d) {
        let grid_color = vec4(0.9, 0.9, 0.9, 1.0);
        let grid_width = 0.5;

        self.draw_grid.color = grid_color;

        let tick_options = TickOptions::default();

        // Draw horizontal grid lines based on Y ticks
        let y_ticks = self.coord.build_y_ticks(&tick_options);
        for tick in &y_ticks {
            let (p1, p2) = self.coord.get_horizontal_grid_line(tick.value);
            self.draw_grid.draw_line(cx, p1, p2, grid_width);
        }

        // Draw vertical grid lines based on X ticks
        let x_ticks = self.coord.build_x_ticks(&tick_options);
        for tick in &x_ticks {
            let (p1, p2) = self.coord.get_vertical_grid_line(tick.value);
            self.draw_grid.draw_line(cx, p1, p2, grid_width);
        }
    }

    fn draw_axes(&mut self, cx: &mut Cx2d) {
        let axis_color = vec4(0.7, 0.7, 0.7, 1.0);
        let axis_width = 0.5;

        self.draw_grid.color = axis_color;

        let area = self.coord.chart_area();

        // Draw X axis at y=0 if in range, otherwise at bottom
        let (y_min, y_max) = self.coord.y_scale().get_data_bounds();
        let x_axis_y = if y_min <= 0.0 && y_max >= 0.0 {
            self.coord.y_scale().get_pixel_for_value(0.0)
        } else {
            area.bottom
        };
        self.draw_grid.draw_line(cx, dvec2(area.left, x_axis_y), dvec2(area.right, x_axis_y), axis_width);

        // Draw Y axis at x=0 if in range, otherwise at left
        let (x_min, x_max) = self.coord.x_scale().get_data_bounds();
        let y_axis_x = if x_min <= 0.0 && x_max >= 0.0 {
            self.coord.x_scale().get_pixel_for_value(0.0)
        } else {
            area.left
        };
        self.draw_grid.draw_line(cx, dvec2(y_axis_x, area.bottom), dvec2(y_axis_x, area.top), axis_width);
    }

    fn get_bubble_radius(&self, r_value: f64, max_r: f64) -> f64 {
        if max_r <= 0.0 {
            return self.min_radius;
        }
        let normalized = r_value / max_r;
        self.min_radius + normalized * (self.max_radius - self.min_radius)
    }

    fn draw_bubbles(&mut self, cx: &mut Cx2d) {
        let progress = self.animator.get_progress();

        // Find max R value for scaling
        let max_r = self.data.datasets.iter()
            .flat_map(|d| d.data.iter())
            .map(|p| p.r.unwrap_or(1.0))
            .fold(0.0f64, |a, b| a.max(b));

        for (dataset_idx, dataset) in self.data.datasets.iter().enumerate() {
            let color = dataset.background_color.unwrap_or_else(|| {
                let mut c = get_color(dataset_idx);
                c.w = 0.6; // Semi-transparent
                c
            });

            for (point_idx, point) in dataset.data.iter().enumerate() {
                let x_val = point.x.unwrap_or(point_idx as f64);
                let x = self.coord.x_scale().get_pixel_for_value(x_val);
                let y = self.coord.y_scale().get_pixel_for_value(point.y);

                let r_value = point.r.unwrap_or(1.0);
                let base_radius = self.get_bubble_radius(r_value, max_r);
                let radius = base_radius * progress;

                let is_hovered = self.hovered_dataset == dataset_idx as i32
                    && self.hovered_bubble == point_idx as i32;

                let draw_radius = if is_hovered { radius * 1.1 } else { radius };

                self.draw_point.color = if is_hovered {
                    lighten(color, 0.1)
                } else {
                    color
                };

                let rect = Rect {
                    pos: dvec2(x - draw_radius, y - draw_radius),
                    size: dvec2(draw_radius * 2.0, draw_radius * 2.0),
                };
                self.draw_point.draw_point(cx, rect);
            }
        }
    }

    fn handle_mouse_move(&mut self, cx: &mut Cx, pos: DVec2) {
        let old_hovered_bubble = self.hovered_bubble;
        let old_hovered_dataset = self.hovered_dataset;
        self.hovered_bubble = -1;
        self.hovered_dataset = -1;

        if !self.coord.contains_pixel(pos.x, pos.y) {
            if old_hovered_bubble != self.hovered_bubble {
                self.redraw(cx);
            }
            return;
        }

        // Find max R for scaling
        let max_r = self.data.datasets.iter()
            .flat_map(|d| d.data.iter())
            .map(|p| p.r.unwrap_or(1.0))
            .fold(0.0f64, |a, b| a.max(b));

        let mut min_dist = f64::MAX;

        for (dataset_idx, dataset) in self.data.datasets.iter().enumerate() {
            for (point_idx, point) in dataset.data.iter().enumerate() {
                let x_val = point.x.unwrap_or(point_idx as f64);
                let x = self.coord.x_scale().get_pixel_for_value(x_val);
                let y = self.coord.y_scale().get_pixel_for_value(point.y);

                let r_value = point.r.unwrap_or(1.0);
                let radius = self.get_bubble_radius(r_value, max_r);

                let dx = pos.x - x;
                let dy = pos.y - y;
                let dist = (dx * dx + dy * dy).sqrt();

                if dist < radius && dist < min_dist {
                    min_dist = dist;
                    self.hovered_bubble = point_idx as i32;
                    self.hovered_dataset = dataset_idx as i32;
                }
            }
        }

        if old_hovered_bubble != self.hovered_bubble || old_hovered_dataset != self.hovered_dataset {
            self.redraw(cx);
        }
    }
}

impl BubbleChartRef {
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
