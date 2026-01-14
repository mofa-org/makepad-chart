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

    pub BarChart = {{BarChart}} {
        width: Fill,
        height: Fill,
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct BarChart {
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

    /// Per-bar animators for delay animation
    #[rust]
    bar_animators: Vec<ChartAnimator>,

    #[rust]
    initialized: bool,

    /// Bar width as percentage of band (0.0 - 1.0)
    #[rust(0.8)]
    bar_percent: f64,

    /// Hover state - index of hovered bar (-1 for none)
    #[rust(-1)]
    hovered_bar: i32,

    /// Whether to stack bars on top of each other
    #[rust(false)]
    stacked: bool,

    /// Enable delay animation (bars animate in sequence)
    #[rust(false)]
    delay_animation: bool,

    /// Delay per data point in ms (default 100ms like Chart.js)
    #[rust(100.0)]
    delay_per_index: f64,

    /// Delay per dataset in ms (default 50ms)
    #[rust(50.0)]
    delay_per_dataset: f64,
}

impl Widget for BarChart {
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
                let time = cx.seconds_since_app_start();
                let mut needs_redraw = false;
                let mut any_running = false;

                if self.delay_animation {
                    // Update all bar animators
                    for animator in &mut self.bar_animators {
                        if animator.is_running() {
                            any_running = true;
                            if animator.update(time) {
                                needs_redraw = true;
                            }
                        }
                    }
                } else if self.animator.is_running() {
                    any_running = true;
                    if self.animator.update(time) {
                        needs_redraw = true;
                    }
                }

                if needs_redraw {
                    self.redraw(cx);
                }
                if any_running {
                    cx.new_next_frame();
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
            self.draw_bars(cx);
            self.draw_labels(cx);
        }

        DrawStep::done()
    }
}


impl BarChart {
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

    /// Get a reference to the data
    pub fn data(&self) -> &ChartData {
        &self.data
    }

    /// Set bar width percentage
    pub fn set_bar_percent(&mut self, percent: f64) {
        self.bar_percent = percent.clamp(0.1, 1.0);
    }

    /// Set stacked mode
    pub fn set_stacked(&mut self, stacked: bool) {
        self.stacked = stacked;
        self.initialized = false;
    }

    /// Enable delay animation (bars animate in sequence)
    pub fn set_delay_animation(&mut self, enabled: bool) {
        self.delay_animation = enabled;
        self.initialized = false;
    }

    /// Set delay timing (delay_per_index in ms, delay_per_dataset in ms)
    pub fn set_delay_timing(&mut self, per_index: f64, per_dataset: f64) {
        self.delay_per_index = per_index;
        self.delay_per_dataset = per_dataset;
    }

    /// Replay the animation from the beginning
    pub fn replay_animation(&mut self, cx: &mut Cx) {
        // Reset state
        self.initialized = false;
        self.bar_animators.clear();
        self.animator.reset();

        // Force immediate animation restart by starting it now
        // (will be re-started in draw_walk if needed)
        let time = cx.seconds_since_app_start();
        if self.delay_animation && !self.data.datasets.is_empty() {
            // Pre-create animators so animation starts immediately
            let num_datasets = self.data.datasets.len();
            let num_points = self.data.datasets.first().map(|d| d.data.len()).unwrap_or(0);

            for dataset_idx in 0..num_datasets {
                for data_idx in 0..num_points {
                    let delay = data_idx as f64 * self.delay_per_index
                        + dataset_idx as f64 * self.delay_per_dataset;
                    let mut animator = ChartAnimator::new(self.options.animation.duration)
                        .with_easing(self.options.animation.easing)
                        .with_delay(delay);
                    animator.start(time);
                    self.bar_animators.push(animator);
                }
            }
            self.initialized = true;
        } else {
            self.animator = ChartAnimator::new(self.options.animation.duration)
                .with_easing(self.options.animation.easing);
            self.animator.start(time);
            self.initialized = true;
        }

        cx.new_next_frame();
        self.redraw(cx);
    }

    fn setup_coord_from_data(&mut self) {
        // Set up category scale for X axis
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

        // Set Y data range from data extent
        if self.stacked {
            // For stacked charts, calculate sum of all values at each index
            let num_points = self.data.datasets.first().map(|d| d.data.len()).unwrap_or(0);
            let mut max_sum = 0.0f64;
            for i in 0..num_points {
                let sum: f64 = self.data.datasets.iter()
                    .filter_map(|d| d.data.get(i))
                    .map(|p| p.y.max(0.0))
                    .sum();
                max_sum = max_sum.max(sum);
            }
            self.coord.set_y_data_range(0.0, max_sum);
        } else if let Some((min, max)) = self.data.get_y_extent() {
            let min = if self.options.scales.y.begin_at_zero && min > 0.0 { 0.0 } else { min };
            self.coord.set_y_data_range(min, max);
        }
    }

    fn update_coord(&mut self, rect: Rect) {
        self.coord.update(rect);
    }

    fn start_animation(&mut self, cx: &mut Cx) {
        let time = cx.seconds_since_app_start();

        if self.delay_animation {
            // Create per-bar animators with staggered delays
            self.bar_animators.clear();

            let num_datasets = self.data.datasets.len();
            let num_points = self.data.datasets.first().map(|d| d.data.len()).unwrap_or(0);

            for dataset_idx in 0..num_datasets {
                for data_idx in 0..num_points {
                    // Chart.js formula: delay = dataIndex * 300 + datasetIndex * 100
                    let delay = data_idx as f64 * self.delay_per_index
                        + dataset_idx as f64 * self.delay_per_dataset;

                    let mut animator = ChartAnimator::new(self.options.animation.duration)
                        .with_easing(self.options.animation.easing)
                        .with_delay(delay);
                    animator.start(time);
                    self.bar_animators.push(animator);
                }
            }
        } else {
            self.animator = ChartAnimator::new(self.options.animation.duration)
                .with_easing(self.options.animation.easing);
            self.animator.start(time);
        }

        cx.new_next_frame();
    }

    /// Get the animation progress for a specific bar
    fn get_bar_progress(&self, dataset_idx: usize, data_idx: usize) -> f64 {
        if self.delay_animation {
            let num_points = self.data.datasets.first().map(|d| d.data.len()).unwrap_or(0);
            let animator_idx = dataset_idx * num_points + data_idx;
            self.bar_animators.get(animator_idx)
                .map(|a| a.get_progress())
                .unwrap_or(1.0)
        } else {
            self.animator.get_progress()
        }
    }

    fn draw_background(&mut self, _cx: &mut Cx2d, _rect: Rect) {
        // Background is handled by the view
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

        // For bar charts, X axis is always at the bottom (baseline for bars)
        self.draw_grid.draw_line(cx, dvec2(area.left, area.bottom), dvec2(area.right, area.bottom), axis_width);

        // Y axis is always at the left
        self.draw_grid.draw_line(cx, dvec2(area.left, area.bottom), dvec2(area.left, area.top), axis_width);
    }

    fn draw_bars(&mut self, cx: &mut Cx2d) {
        let bar_width = self.coord.get_bar_width(self.bar_percent);
        let num_datasets = self.data.datasets.len();

        if self.stacked {
            // Stacked bars - all bars have same width, stacked on top of each other
            let num_points = self.data.datasets.first().map(|d| d.data.len()).unwrap_or(0);

            for data_idx in 0..num_points {
                let x_center = self.coord.x_scale().get_pixel_for_value(data_idx as f64);
                let bar_x = x_center - bar_width / 2.0;

                let mut cumulative_value = 0.0;

                for (dataset_idx, dataset) in self.data.datasets.iter().enumerate() {
                    let color = dataset.background_color.unwrap_or_else(|| get_color(dataset_idx));
                    self.draw_bar.color = color;

                    if let Some(point) = dataset.data.get(data_idx) {
                        let progress = self.get_bar_progress(dataset_idx, data_idx);
                        let y_value = point.y.max(0.0) * progress;

                        if y_value > 0.0 {
                            // Calculate base position (top of previous bars)
                            let base_value = cumulative_value;
                            let top_value = cumulative_value + y_value;

                            let base_y = self.coord.y_scale().get_pixel_for_value(base_value);
                            let top_y = self.coord.y_scale().get_pixel_for_value(top_value);

                            let bar_height = base_y - top_y;

                            if bar_height > 0.5 {
                                let bar_rect = Rect {
                                    pos: dvec2(bar_x, top_y),
                                    size: dvec2(bar_width, bar_height),
                                };
                                self.draw_bar.draw_bar(cx, bar_rect);
                            }

                            cumulative_value = top_value;
                        }
                    }
                }
            }
        } else {
            // Grouped bars (original behavior)
            let group_bar_width = if num_datasets > 0 {
                bar_width / num_datasets as f64
            } else {
                bar_width
            };

            for (dataset_idx, dataset) in self.data.datasets.iter().enumerate() {
                let color = dataset.background_color.unwrap_or_else(|| get_color(dataset_idx));
                self.draw_bar.color = color;

                let border_radius = 4.0;
                self.draw_bar.set_top_radius(border_radius);

                for (data_idx, point) in dataset.data.iter().enumerate() {
                    let progress = self.get_bar_progress(dataset_idx, data_idx);
                    let x_center = self.coord.x_scale().get_pixel_for_value(data_idx as f64);

                    // Offset for grouped bars
                    let group_offset = if num_datasets > 1 {
                        let total_group_width = bar_width;
                        let start_offset = -total_group_width / 2.0 + group_bar_width / 2.0;
                        start_offset + dataset_idx as f64 * group_bar_width
                    } else {
                        0.0
                    };

                    let bar_x = x_center + group_offset - group_bar_width / 2.0;

                    // Get y value with animation
                    let y_value = point.y * progress;
                    let y_pixel = self.coord.y_scale().get_pixel_for_value(y_value);

                    // For floating bars, use y_min as base; otherwise use 0
                    let base_value = point.y_min.map(|m| m * progress).unwrap_or(0.0);
                    let base_y = self.coord.y_scale().get_pixel_for_value(base_value);

                    // Calculate bar height (y_pixel is at top since Y is inverted)
                    let bar_height = (base_y - y_pixel).abs();
                    let bar_top = y_pixel.min(base_y);

                    // Check if this bar is hovered
                    let is_hovered = self.hovered_bar >= 0 &&
                        self.hovered_bar as usize == data_idx &&
                        num_datasets == 1; // Simple hover for single dataset

                    if is_hovered {
                        // Slightly brighter on hover
                        self.draw_bar.color = lighten(color, 0.15);
                    } else {
                        self.draw_bar.color = color;
                    }

                    if bar_height > 0.5 {
                        let bar_rect = Rect {
                            pos: dvec2(bar_x, bar_top),
                            size: dvec2(group_bar_width, bar_height),
                        };

                        self.draw_bar.draw_bar(cx, bar_rect);
                    }
                }
            }
        }
    }

    fn draw_labels(&mut self, _cx: &mut Cx2d) {
        // TODO: Draw axis labels using text rendering
        // For now, labels are handled through the Makepad text system
        // This would require integrating with DrawText
    }

    fn handle_mouse_move(&mut self, cx: &mut Cx, pos: DVec2) {
        let old_hovered = self.hovered_bar;

        if self.coord.contains_pixel(pos.x, pos.y) {
            // Find which bar is being hovered
            let x_value = self.coord.x_scale().get_value_for_pixel(pos.x);
            self.hovered_bar = x_value.round() as i32;
        } else {
            self.hovered_bar = -1;
        }

        if old_hovered != self.hovered_bar {
            self.redraw(cx);
        }
    }

    fn handle_mouse_down(&mut self, _cx: &mut Cx, pos: DVec2) {
        if self.coord.contains_pixel(pos.x, pos.y) {
            let x_value = self.coord.x_scale().get_value_for_pixel(pos.x);
            let bar_index = x_value.round() as usize;

            // Could emit an action here for bar click events
            log!("Bar clicked: index={}", bar_index);
        }
    }
}

impl BarChartRef {
    /// Set chart data
    pub fn set_data(&self, data: ChartData) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_data(data);
        }
    }

    /// Set chart options
    pub fn set_options(&self, options: ChartOptions) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_options(options);
        }
    }

    /// Set stacked mode
    pub fn set_stacked(&self, stacked: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_stacked(stacked);
        }
    }

    /// Enable delay animation (bars animate in sequence)
    pub fn set_delay_animation(&self, enabled: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_delay_animation(enabled);
        }
    }

    /// Set delay timing (delay_per_index in ms, delay_per_dataset in ms)
    pub fn set_delay_timing(&self, per_index: f64, per_dataset: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_delay_timing(per_index, per_dataset);
        }
    }

    /// Replay the animation from the beginning
    pub fn replay_animation(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.replay_animation(cx);
        }
    }

    /// Trigger redraw
    pub fn redraw(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.redraw(cx);
        }
    }
}
