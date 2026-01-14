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
    use crate::element::grid::DrawGridLine;

    pub HorizontalBarChart = {{HorizontalBarChart}} {
        width: Fill,
        height: Fill,
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct HorizontalBarChart {
    #[live]
    #[deref]
    view: View,

    #[live]
    draw_bar: DrawBar,

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

    /// Bar height as percentage of band (0.0 - 1.0)
    #[rust(0.8)]
    bar_percent: f64,

    /// Hover state - index of hovered bar (-1 for none)
    #[rust(-1)]
    hovered_bar: i32,
}

impl Widget for HorizontalBarChart {
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
            self.draw_bars(cx);
        }

        DrawStep::done()
    }
}

impl HorizontalBarChart {
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

    pub fn set_bar_percent(&mut self, percent: f64) {
        self.bar_percent = percent.clamp(0.1, 1.0);
    }

    fn setup_coord_from_data(&mut self) {
        // For horizontal bars: categories on Y axis, values on X axis
        let labels: Vec<String> = self.data.labels.clone();

        let category_scale = CategoryScale::new()
            .with_labels(labels)
            .with_offset(true);

        self.coord = CartesianCoord::new()
            .with_x_scale(ScaleType::Linear(
                LinearScale::new()
                    .with_begin_at_zero(false)
                    .with_nice(true)
            ))
            .with_y_scale(ScaleType::Category(category_scale));

        // Set X data range from data extent (values) - include negative values
        if let Some((min, max)) = self.data.get_y_extent() {
            // For horizontal bars, we want to show zero line if data crosses zero
            // or if begin_at_zero is set
            let adj_min = if self.options.scales.y.begin_at_zero && min > 0.0 {
                0.0
            } else {
                min
            };
            let adj_max = if self.options.scales.y.begin_at_zero && max < 0.0 {
                0.0
            } else {
                max
            };
            self.coord.set_x_data_range(adj_min, adj_max);
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

        // For horizontal bar charts, draw vertical grid lines based on X ticks
        let tick_options = TickOptions::default();
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

        // Draw X axis at bottom
        self.draw_grid.draw_line(cx, dvec2(area.left, area.bottom), dvec2(area.right, area.bottom), axis_width);

        // Draw Y axis at x=0 if in range, otherwise at left
        let (x_min, x_max) = self.coord.x_scale().get_data_bounds();
        let y_axis_x = if x_min <= 0.0 && x_max >= 0.0 {
            self.coord.x_scale().get_pixel_for_value(0.0)
        } else {
            area.left
        };
        self.draw_grid.draw_line(cx, dvec2(y_axis_x, area.bottom), dvec2(y_axis_x, area.top), axis_width);
    }

    fn draw_bars(&mut self, cx: &mut Cx2d) {
        let progress = self.animator.get_progress();
        let bar_height = self.get_bar_height();

        let num_datasets = self.data.datasets.len();
        let group_bar_height = if num_datasets > 0 {
            bar_height / num_datasets as f64
        } else {
            bar_height
        };

        for (dataset_idx, dataset) in self.data.datasets.iter().enumerate() {
            let color = dataset.background_color.unwrap_or_else(|| get_color(dataset_idx));
            self.draw_bar.color = color;

            for (data_idx, point) in dataset.data.iter().enumerate() {
                // Y position based on category index
                let y_center = self.coord.y_scale().get_pixel_for_value(data_idx as f64);

                // Offset for grouped bars
                let group_offset = if num_datasets > 1 {
                    let total_group_height = bar_height;
                    let start_offset = -total_group_height / 2.0 + group_bar_height / 2.0;
                    start_offset + dataset_idx as f64 * group_bar_height
                } else {
                    0.0
                };

                let bar_y = y_center + group_offset - group_bar_height / 2.0;

                // Get x value with animation (bar grows from zero)
                let x_value = point.y * progress;
                let x_pixel = self.coord.x_scale().get_pixel_for_value(x_value);
                let base_x = self.coord.x_scale().get_pixel_for_value(0.0);

                // Calculate bar width - can be negative for negative values
                let bar_width = x_pixel - base_x;

                // Check if this bar is hovered
                let is_hovered = self.hovered_bar >= 0 &&
                    self.hovered_bar as usize == data_idx &&
                    num_datasets == 1;

                if is_hovered {
                    self.draw_bar.color = lighten(color, 0.15);
                } else {
                    self.draw_bar.color = color;
                }

                // Handle both positive and negative bars
                if bar_width.abs() > 0.5 {
                    let (bar_x, abs_width) = if bar_width >= 0.0 {
                        (base_x, bar_width)
                    } else {
                        // Negative value: bar extends left from zero
                        (x_pixel, -bar_width)
                    };

                    let bar_rect = Rect {
                        pos: dvec2(bar_x, bar_y),
                        size: dvec2(abs_width, group_bar_height),
                    };

                    self.draw_bar.draw_bar(cx, bar_rect);
                }
            }
        }
    }

    fn get_bar_height(&self) -> f64 {
        let chart_area = self.coord.chart_area();
        let num_categories = self.data.labels.len().max(
            self.data.datasets.first().map(|d| d.data.len()).unwrap_or(1)
        );

        if num_categories > 0 {
            (chart_area.height() / num_categories as f64) * self.bar_percent
        } else {
            chart_area.height() * self.bar_percent
        }
    }

    fn handle_mouse_move(&mut self, cx: &mut Cx, pos: DVec2) {
        let old_hovered = self.hovered_bar;

        if self.coord.contains_pixel(pos.x, pos.y) {
            let y_value = self.coord.y_scale().get_value_for_pixel(pos.y);
            self.hovered_bar = y_value.round() as i32;
        } else {
            self.hovered_bar = -1;
        }

        if old_hovered != self.hovered_bar {
            self.redraw(cx);
        }
    }

    fn handle_mouse_down(&mut self, _cx: &mut Cx, pos: DVec2) {
        if self.coord.contains_pixel(pos.x, pos.y) {
            let y_value = self.coord.y_scale().get_value_for_pixel(pos.y);
            let bar_index = y_value.round() as usize;
            log!("Horizontal bar clicked: index={}", bar_index);
        }
    }
}

impl HorizontalBarChartRef {
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

    pub fn redraw(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.redraw(cx);
        }
    }
}
