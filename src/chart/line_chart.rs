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

    use crate::element::line::DrawChartLine;
    use crate::element::point::DrawPoint;
    use crate::element::triangle::DrawTriangle;
    use crate::element::grid::DrawGridLine;

    pub LineChart = {{LineChart}} {
        width: Fill,
        height: Fill,
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct LineChart {
    #[live]
    #[deref]
    view: View,

    #[live]
    draw_line: DrawChartLine,

    #[live]
    draw_point: DrawPoint,

    #[live]
    draw_fill: DrawTriangle,

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

    /// Point radius
    #[rust(4.0)]
    point_radius: f64,

    /// Whether to show points
    #[rust(true)]
    show_points: bool,

    /// Whether to fill area under line
    #[rust(false)]
    fill: bool,

    /// Line tension (0 = straight lines, 0.4 = smooth curves)
    #[rust(0.0)]
    tension: f64,

    /// Hovered point index (-1 for none)
    #[rust(-1)]
    hovered_point: i32,

    /// Stepped line mode: "none", "before", "after", "middle"
    #[rust]
    stepped: SteppedMode,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum SteppedMode {
    #[default]
    None,
    Before,  // Step happens before the point
    After,   // Step happens after the point
    Middle,  // Step happens in the middle
}

impl Widget for LineChart {
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

            self.draw_background(cx, rect);
            self.draw_grid_lines(cx);
            self.draw_axes(cx);
            self.draw_lines(cx);
            if self.show_points {
                self.draw_points(cx);
            }
        }

        DrawStep::done()
    }
}


impl LineChart {
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

    /// Enable/disable points
    pub fn set_show_points(&mut self, show: bool) {
        self.show_points = show;
    }

    /// Set point radius
    pub fn set_point_radius(&mut self, radius: f64) {
        self.point_radius = radius;
    }

    /// Enable/disable area fill
    pub fn set_fill(&mut self, fill: bool) {
        self.fill = fill;
    }

    /// Set line tension (smoothness)
    pub fn set_tension(&mut self, tension: f64) {
        self.tension = tension.clamp(0.0, 1.0);
    }

    /// Set stepped mode
    pub fn set_stepped(&mut self, mode: SteppedMode) {
        self.stepped = mode;
    }

    fn setup_coord_from_data(&mut self) {
        // Set up category scale for X axis
        let labels: Vec<String> = self.data.labels.clone();

        let category_scale = CategoryScale::new()
            .with_labels(labels)
            .with_offset(false); // Lines go through grid lines

        self.coord = CartesianCoord::new()
            .with_x_scale(ScaleType::Category(category_scale))
            .with_y_scale(ScaleType::Linear(
                LinearScale::new()
                    .with_begin_at_zero(self.options.scales.y.begin_at_zero)
                    .with_nice(true)
            ));

        // Set Y data range from data extent
        if let Some((min, max)) = self.data.get_y_extent() {
            let min = if self.options.scales.y.begin_at_zero && min > 0.0 { 0.0 } else { min };
            self.coord.set_y_data_range(min, max);
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

    fn draw_background(&mut self, _cx: &mut Cx2d, _rect: Rect) {
        // Background handled by view
    }

    fn draw_grid_lines(&mut self, cx: &mut Cx2d) {
        let grid_color = vec4(0.9, 0.9, 0.9, 1.0);
        let grid_width = 0.5;

        self.draw_grid.color = grid_color;

        // Draw horizontal grid lines based on Y ticks
        let tick_options = TickOptions::default();
        let y_ticks = self.coord.build_y_ticks(&tick_options);

        for tick in &y_ticks {
            let (p1, p2) = self.coord.get_horizontal_grid_line(tick.value);
            self.draw_grid.draw_line(cx, p1, p2, grid_width);
        }
    }

    fn draw_axes(&mut self, cx: &mut Cx2d) {
        let axis_color = vec4(0.7, 0.7, 0.7, 1.0);
        let axis_width = 0.5;

        self.draw_grid.color = axis_color;

        let area = self.coord.chart_area();

        // X axis at the bottom
        self.draw_grid.draw_line(cx, dvec2(area.left, area.bottom), dvec2(area.right, area.bottom), axis_width);

        // Y axis at the left
        self.draw_grid.draw_line(cx, dvec2(area.left, area.bottom), dvec2(area.left, area.top), axis_width);
    }

    fn draw_lines(&mut self, cx: &mut Cx2d) {
        let progress = self.animator.get_progress();
        let line_width = 3.0;
        let base_y = self.coord.y_scale().get_pixel_for_value(0.0);

        for (dataset_idx, dataset) in self.data.datasets.iter().enumerate() {
            if dataset.data.len() < 2 {
                continue;
            }

            let color = dataset.border_color
                .or(dataset.background_color)
                .unwrap_or_else(|| get_color(dataset_idx));

            // Calculate points with animation
            let points: Vec<DVec2> = dataset.data.iter().enumerate().map(|(i, point)| {
                let x = self.coord.x_scale().get_pixel_for_value(i as f64);
                let y_value = point.y * progress;
                let y = self.coord.y_scale().get_pixel_for_value(y_value);
                dvec2(x, y)
            }).collect();

            // Draw fill first (behind the line)
            if self.fill && points.len() >= 2 {
                let fill_color = vec4(color.x, color.y, color.z, 0.3);
                self.draw_fill.color = fill_color;

                // Draw triangles from each line segment down to the base
                for i in 0..points.len() - 1 {
                    let p1 = points[i];
                    let p2 = points[i + 1];
                    let b1 = dvec2(p1.x, base_y);
                    let b2 = dvec2(p2.x, base_y);

                    // Two triangles to fill the area
                    self.draw_fill.draw_triangle(cx, p1, p2, b1);
                    self.draw_fill.draw_triangle(cx, p2, b2, b1);
                }
            }

            // Draw line segments
            self.draw_line.color = color;
            match self.stepped {
                SteppedMode::None => {
                    // Standard lines connecting points
                    for i in 0..points.len() - 1 {
                        let p1 = points[i];
                        let p2 = points[i + 1];
                        self.draw_line.draw_line(cx, p1, p2, line_width);
                    }
                }
                SteppedMode::Before => {
                    // Step before: vertical first, then horizontal
                    for i in 0..points.len() - 1 {
                        let p1 = points[i];
                        let p2 = points[i + 1];
                        let mid = dvec2(p1.x, p2.y);
                        self.draw_line.draw_line(cx, p1, mid, line_width);
                        self.draw_line.draw_line(cx, mid, p2, line_width);
                    }
                }
                SteppedMode::After => {
                    // Step after: horizontal first, then vertical
                    for i in 0..points.len() - 1 {
                        let p1 = points[i];
                        let p2 = points[i + 1];
                        let mid = dvec2(p2.x, p1.y);
                        self.draw_line.draw_line(cx, p1, mid, line_width);
                        self.draw_line.draw_line(cx, mid, p2, line_width);
                    }
                }
                SteppedMode::Middle => {
                    // Step in middle: horizontal, vertical, horizontal
                    for i in 0..points.len() - 1 {
                        let p1 = points[i];
                        let p2 = points[i + 1];
                        let mid_x = (p1.x + p2.x) / 2.0;
                        let mid1 = dvec2(mid_x, p1.y);
                        let mid2 = dvec2(mid_x, p2.y);
                        self.draw_line.draw_line(cx, p1, mid1, line_width);
                        self.draw_line.draw_line(cx, mid1, mid2, line_width);
                        self.draw_line.draw_line(cx, mid2, p2, line_width);
                    }
                }
            }
        }
    }

    fn draw_points(&mut self, cx: &mut Cx2d) {
        let progress = self.animator.get_progress();

        for (dataset_idx, dataset) in self.data.datasets.iter().enumerate() {
            let color = dataset.background_color.unwrap_or_else(|| get_color(dataset_idx));
            self.draw_point.color = color;

            for (data_idx, point) in dataset.data.iter().enumerate() {
                let x = self.coord.x_scale().get_pixel_for_value(data_idx as f64);
                let y_value = point.y * progress;
                let y = self.coord.y_scale().get_pixel_for_value(y_value);

                let is_hovered = self.hovered_point >= 0 &&
                    self.hovered_point as usize == data_idx;

                let radius = if is_hovered {
                    self.point_radius * 1.5
                } else {
                    self.point_radius
                };

                let rect = Rect {
                    pos: dvec2(x - radius, y - radius),
                    size: dvec2(radius * 2.0, radius * 2.0),
                };
                self.draw_point.draw_point(cx, rect);
            }
        }
    }

    fn handle_mouse_move(&mut self, cx: &mut Cx, pos: DVec2) {
        let old_hovered = self.hovered_point;
        self.hovered_point = -1;

        if !self.coord.contains_pixel(pos.x, pos.y) {
            if old_hovered != self.hovered_point {
                self.redraw(cx);
            }
            return;
        }

        // Find nearest point
        let mut min_dist = f64::MAX;

        for dataset in &self.data.datasets {
            for (i, point) in dataset.data.iter().enumerate() {
                let px = self.coord.x_scale().get_pixel_for_value(i as f64);
                let py = self.coord.y_scale().get_pixel_for_value(point.y);

                let dx = pos.x - px;
                let dy = pos.y - py;
                let dist = (dx * dx + dy * dy).sqrt();

                if dist < min_dist && dist < 20.0 {
                    min_dist = dist;
                    self.hovered_point = i as i32;
                }
            }
        }

        if old_hovered != self.hovered_point {
            self.redraw(cx);
        }
    }
}

impl LineChartRef {
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

    pub fn set_fill(&self, fill: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_fill(fill);
        }
    }

    pub fn set_stepped(&self, mode: SteppedMode) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_stepped(mode);
        }
    }

    pub fn redraw(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.redraw(cx);
        }
    }
}
