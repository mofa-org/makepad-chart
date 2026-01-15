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

    use crate::element::bar::DrawBar;
    use crate::element::line::DrawChartLine;
    use crate::element::point::DrawPoint;
    use crate::element::grid::DrawGridLine;

    pub ComboChart = {{ComboChart}} {
        width: Fill,
        height: Fill,
    }
}

/// Dataset type for combo charts
#[derive(Clone, Debug, Default, PartialEq)]
pub enum DatasetType {
    #[default]
    Bar,
    Line,
}

#[derive(Live, LiveHook, Widget)]
pub struct ComboChart {
    #[live]
    #[deref]
    view: View,

    #[live]
    draw_bar: DrawBar,

    #[live]
    draw_line: DrawChartLine,

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

    #[rust(0.6)]
    bar_percent: f64,

    #[rust(4.0)]
    point_radius: f64,

    #[rust]
    dataset_types: Vec<DatasetType>,

    /// Enable gradient for bars (vertical gradient)
    #[rust(false)]
    gradient_enabled: bool,
}

impl Widget for ComboChart {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        match event {
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
                // Force redraw on window resize
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

            self.draw_grid_lines(cx);
            self.draw_axes(cx);
            self.draw_bars(cx);
            self.draw_lines(cx);
        }

        DrawStep::done()
    }
}

impl ComboChart {
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

    /// Set the type for each dataset (Bar or Line)
    pub fn set_dataset_types(&mut self, types: Vec<DatasetType>) {
        self.dataset_types = types;
    }

    /// Enable gradient for bars
    pub fn set_gradient(&mut self, enabled: bool) {
        self.gradient_enabled = enabled;
    }

    fn get_dataset_type(&self, idx: usize) -> DatasetType {
        self.dataset_types.get(idx).cloned().unwrap_or(DatasetType::Bar)
    }

    fn setup_coord_from_data(&mut self) {
        let labels: Vec<String> = self.data.labels.clone();

        let category_scale = CategoryScale::new()
            .with_labels(labels)
            .with_offset(true);

        self.coord = CartesianCoord::new()
            .with_x_scale(ScaleType::Category(category_scale))
            .with_y_scale(ScaleType::Linear(
                LinearScale::new()
                    .with_begin_at_zero(self.options.scales.y.begin_at_zero)
                    .with_nice(true)
            ));

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

    fn draw_grid_lines(&mut self, cx: &mut Cx2d) {
        let grid_color = vec4(0.9, 0.9, 0.9, 1.0);
        let grid_width = 0.5;

        self.draw_grid.color = grid_color;

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

    fn draw_bars(&mut self, cx: &mut Cx2d) {
        let progress = self.animator.get_progress();
        let bar_width = self.coord.get_bar_width(self.bar_percent);

        // Count bar datasets
        let bar_datasets: Vec<usize> = (0..self.data.datasets.len())
            .filter(|&i| self.get_dataset_type(i) == DatasetType::Bar)
            .collect();

        let num_bar_datasets = bar_datasets.len();
        let group_bar_width = if num_bar_datasets > 0 {
            bar_width / num_bar_datasets as f64
        } else {
            bar_width
        };

        for (bar_idx, &dataset_idx) in bar_datasets.iter().enumerate() {
            let dataset = &self.data.datasets[dataset_idx];
            let color = dataset.background_color.unwrap_or_else(|| get_color(dataset_idx));
            self.draw_bar.color = color;

            // Apply gradient if enabled
            if self.gradient_enabled {
                let lighter = lighten(color, 0.3);
                self.draw_bar.set_vertical_gradient(color, lighter);
            } else {
                self.draw_bar.disable_gradient();
            }

            for (data_idx, point) in dataset.data.iter().enumerate() {
                let x_center = self.coord.x_scale().get_pixel_for_value(data_idx as f64);

                let group_offset = if num_bar_datasets > 1 {
                    let start_offset = -bar_width / 2.0 + group_bar_width / 2.0;
                    start_offset + bar_idx as f64 * group_bar_width
                } else {
                    0.0
                };

                let bar_x = x_center + group_offset - group_bar_width / 2.0;

                let y_value = point.y * progress;
                let y_pixel = self.coord.y_scale().get_pixel_for_value(y_value);
                let base_y = self.coord.y_scale().get_pixel_for_value(0.0);

                let bar_height = base_y - y_pixel;

                if bar_height > 0.0 {
                    let bar_rect = Rect {
                        pos: dvec2(bar_x, y_pixel),
                        size: dvec2(group_bar_width, bar_height),
                    };
                    self.draw_bar.draw_bar(cx, bar_rect);
                }
            }
        }
    }

    fn draw_lines(&mut self, cx: &mut Cx2d) {
        let progress = self.animator.get_progress();
        let line_width = 3.0;

        for (dataset_idx, dataset) in self.data.datasets.iter().enumerate() {
            if self.get_dataset_type(dataset_idx) != DatasetType::Line {
                continue;
            }

            if dataset.data.len() < 2 {
                continue;
            }

            let color = dataset.border_color
                .or(dataset.background_color)
                .unwrap_or_else(|| get_color(dataset_idx));
            self.draw_line.color = color;

            let points: Vec<DVec2> = dataset.data.iter().enumerate().map(|(i, point)| {
                let x = self.coord.x_scale().get_pixel_for_value(i as f64);
                let y_value = point.y * progress;
                let y = self.coord.y_scale().get_pixel_for_value(y_value);
                dvec2(x, y)
            }).collect();

            // Draw lines
            for i in 0..points.len() - 1 {
                self.draw_line.draw_line(cx, points[i], points[i + 1], line_width);
            }

            // Draw points
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

impl ComboChartRef {
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

    pub fn set_dataset_types(&self, types: Vec<DatasetType>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_dataset_types(types);
        }
    }

    pub fn set_gradient(&self, enabled: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_gradient(enabled);
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
