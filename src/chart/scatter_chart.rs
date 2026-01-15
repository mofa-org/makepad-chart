use makepad_widgets::*;
use crate::core::{ChartData, ChartOptions, TickOptions, get_color, lighten};
use crate::coord::{CartesianCoord, ScaleType};
use crate::scale::LinearScale;
use crate::element::{DrawPoint, PointStyle, DrawGridLine};
use crate::animation::ChartAnimator;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::element::point::DrawPoint;
    use crate::element::grid::DrawGridLine;

    pub ScatterChart = {{ScatterChart}} {
        width: Fill,
        height: Fill,
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ScatterChart {
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

    /// Default point radius
    #[rust(6.0)]
    point_radius: f64,

    /// Point style
    #[rust]
    point_style: PointStyle,

    /// Hovered point info (dataset_idx, point_idx)
    #[rust((-1, -1))]
    hovered_point: (i32, i32),

    /// Enable radial gradient for points (center to edge)
    #[rust(false)]
    gradient_enabled: bool,
}

impl Widget for ScatterChart {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        match event {
            Event::MouseMove(e) => {
                self.handle_mouse_move(cx, e.abs);
            }
            Event::MouseDown(e) => {
                self.handle_mouse_down(cx, e.abs);
            }
            Event::NextFrame(_) => {
                if self.animator.is_running() {
                    let time = cx.seconds_since_app_start();
                    if self.animator.update(time) {
                        self.redraw(cx);
                    }
                    // Keep requesting frames while animation is running
                    cx.new_next_frame();
                }
            }
            Event::WindowGeomChange(_) => {
                self.redraw(cx);
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

            self.draw_background(cx, rect);
            self.draw_grid_lines(cx);
            self.draw_axes(cx);
            self.draw_points(cx);
        }

        DrawStep::done()
    }
}


impl ScatterChart {
    /// Set chart data
    pub fn set_data(&mut self, data: ChartData) {
        self.data = data;
        self.initialized = false;
        self.setup_coord_from_data();
    }

    /// Set chart options
    pub fn set_options(&mut self, options: ChartOptions) {
        self.options = options;
        // Re-setup coordinate system with new options
        if !self.data.datasets.is_empty() {
            self.setup_coord_from_data();
        }
    }

    /// Set point radius
    pub fn set_point_radius(&mut self, radius: f64) {
        self.point_radius = radius;
    }

    /// Set point style
    pub fn set_point_style(&mut self, style: PointStyle) {
        self.point_style = style;
    }

    /// Enable radial gradient for points (center to edge)
    pub fn set_gradient(&mut self, enabled: bool) {
        self.gradient_enabled = enabled;
    }

    fn setup_coord_from_data(&mut self) {
        // Scatter charts use linear scales for both axes
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

        // Set data ranges from extent
        if let Some((x_min, x_max)) = self.data.get_x_extent() {
            self.coord.set_x_data_range(x_min, x_max);
        }
        if let Some((y_min, y_max)) = self.data.get_y_extent() {
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

    /// Replay the animation from the beginning
    pub fn replay_animation(&mut self, cx: &mut Cx) {
        // Reset animation state
        self.initialized = false;
        self.animator.reset();

        // Start animation
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(self.options.animation.duration)
            .with_easing(self.options.animation.easing);
        self.animator.start(time);
        self.initialized = true;

        // Trigger redraw to start animation
        cx.new_next_frame();
        self.redraw(cx);
    }

    /// Check if animation is currently running
    pub fn is_animating(&self) -> bool {
        self.animator.is_running()
    }

    fn draw_background(&mut self, _cx: &mut Cx2d, _rect: Rect) {
        // Background handled by view
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

    fn draw_points(&mut self, cx: &mut Cx2d) {
        let progress = self.animator.get_progress();

        for (dataset_idx, dataset) in self.data.datasets.iter().enumerate() {
            let base_color = dataset.background_color.unwrap_or_else(|| get_color(dataset_idx));

            for (point_idx, point) in dataset.data.iter().enumerate() {
                let is_hovered = self.hovered_point == (dataset_idx as i32, point_idx as i32);

                // Get x value (either from point.x or index)
                let x_value = point.x.unwrap_or(point_idx as f64);

                let px = self.coord.x_scale().get_pixel_for_value(x_value);
                let py = self.coord.y_scale().get_pixel_for_value(point.y);

                // Animation: points fade in and scale up
                let animated_radius = self.point_radius * progress;

                // Hover effect
                let radius = if is_hovered {
                    animated_radius * 1.5
                } else {
                    animated_radius
                };

                let color = if is_hovered {
                    lighten(base_color, 0.15)
                } else {
                    base_color
                };

                self.draw_point.color = color;

                // Apply gradient if enabled
                if self.gradient_enabled {
                    let lighter = lighten(color, 0.4);
                    self.draw_point.set_radial_gradient(lighter, color);
                } else {
                    self.draw_point.disable_gradient();
                }

                let rect = Rect {
                    pos: dvec2(px - radius, py - radius),
                    size: dvec2(radius * 2.0, radius * 2.0),
                };
                self.draw_point.draw_point(cx, rect);
            }
        }
    }

    fn handle_mouse_move(&mut self, cx: &mut Cx, pos: DVec2) {
        let old_hovered = self.hovered_point;
        self.hovered_point = (-1, -1);

        if !self.coord.contains_pixel(pos.x, pos.y) {
            if old_hovered != self.hovered_point {
                self.redraw(cx);
            }
            return;
        }

        // Find nearest point
        let mut min_dist = f64::MAX;
        let hit_radius = self.point_radius * 2.0 + 5.0;

        for (dataset_idx, dataset) in self.data.datasets.iter().enumerate() {
            for (point_idx, point) in dataset.data.iter().enumerate() {
                let x_value = point.x.unwrap_or(point_idx as f64);
                let px = self.coord.x_scale().get_pixel_for_value(x_value);
                let py = self.coord.y_scale().get_pixel_for_value(point.y);

                let dx = pos.x - px;
                let dy = pos.y - py;
                let dist = (dx * dx + dy * dy).sqrt();

                if dist < min_dist && dist < hit_radius {
                    min_dist = dist;
                    self.hovered_point = (dataset_idx as i32, point_idx as i32);
                }
            }
        }

        if old_hovered != self.hovered_point {
            self.redraw(cx);
        }
    }

    fn handle_mouse_down(&mut self, _cx: &mut Cx, _pos: DVec2) {
        let (dataset_idx, point_idx) = self.hovered_point;
        if dataset_idx >= 0 && point_idx >= 0 {
            if let Some(dataset) = self.data.datasets.get(dataset_idx as usize) {
                if let Some(point) = dataset.data.get(point_idx as usize) {
                    let x = point.x.unwrap_or(point_idx as f64);
                    log!("Point clicked: ({}, {})", x, point.y);
                }
            }
        }
    }
}

impl ScatterChartRef {
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

    pub fn replay_animation(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.replay_animation(cx);
        }
    }

    pub fn set_point_style(&self, style: PointStyle) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_point_style(style);
        }
    }

    pub fn set_gradient(&self, enabled: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_gradient(enabled);
        }
    }

    pub fn redraw(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.redraw(cx);
        }
    }

    pub fn is_animating(&self) -> bool {
        if let Some(inner) = self.borrow() {
            inner.is_animating()
        } else {
            false
        }
    }
}
