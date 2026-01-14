use makepad_widgets::*;
use std::f64::consts::PI;
use crate::core::*;
use crate::coord::*;
use crate::element::*;
use crate::animation::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::element::arc::DrawArc;

    pub PieChart = {{PieChart}} {
        width: Fill,
        height: Fill,
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct PieChart {
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
    coord: PolarCoord,

    #[rust]
    animator: ChartAnimator,

    #[rust]
    initialized: bool,

    /// Whether this is a doughnut chart (has inner radius)
    #[rust(false)]
    is_doughnut: bool,

    /// Inner radius ratio (0.0-0.99) for doughnut charts
    #[rust(0.5)]
    inner_radius_ratio: f64,

    /// Padding around the pie
    #[rust(20.0)]
    padding: f64,

    /// Hovered slice index (-1 for none)
    #[rust(-1)]
    hovered_slice: i32,

    /// Precomputed slice info
    #[rust]
    slices: Vec<SliceInfo>,

    /// Gradient enabled
    #[rust(false)]
    gradient_enabled: bool,

    /// Gradient type: 0 = radial, 1 = angular
    #[rust(0)]
    gradient_type: u8,
}

#[derive(Clone, Debug)]
struct SliceInfo {
    start_value: f64,
    end_value: f64,
    value: f64,
    percentage: f64,
    color: Vec4,
    label: String,
}

impl Widget for PieChart {
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
                self.compute_slices();
                self.start_animation(cx);
                self.initialized = true;
            }

            self.draw_background(cx, rect);
            self.draw_slices(cx);
        }

        DrawStep::done()
    }
}


impl PieChart {
    /// Set chart data
    pub fn set_data(&mut self, data: ChartData) {
        self.data = data;
        self.initialized = false;
    }

    /// Set chart options
    pub fn set_options(&mut self, options: ChartOptions) {
        self.options = options;
    }

    /// Convert to doughnut chart
    pub fn set_doughnut(&mut self, is_doughnut: bool) {
        self.is_doughnut = is_doughnut;
    }

    /// Set inner radius ratio for doughnut
    pub fn set_inner_radius_ratio(&mut self, ratio: f64) {
        self.inner_radius_ratio = ratio.clamp(0.0, 0.99);
    }

    /// Set padding
    pub fn set_padding(&mut self, padding: f64) {
        self.padding = padding;
    }

    /// Enable radial gradient (inner color at center/inner radius, outer color at edge)
    pub fn set_radial_gradient(&mut self, enabled: bool) {
        self.gradient_enabled = enabled;
        self.gradient_type = 0;
    }

    /// Enable angular gradient (colors interpolate along each slice)
    pub fn set_angular_gradient(&mut self, enabled: bool) {
        self.gradient_enabled = enabled;
        self.gradient_type = 1;
    }

    /// Disable gradient
    pub fn disable_gradient(&mut self) {
        self.gradient_enabled = false;
    }

    fn update_coord(&mut self, rect: Rect) {
        self.coord.update(rect, self.padding);
        if self.is_doughnut {
            self.coord.set_inner_radius_ratio(self.inner_radius_ratio);
        }
    }

    fn compute_slices(&mut self) {
        self.slices.clear();

        // Get total from first dataset
        let Some(dataset) = self.data.datasets.first() else {
            return;
        };

        let total: f64 = dataset.data.iter().map(|p| p.y.max(0.0)).sum();
        if total == 0.0 {
            return;
        }

        let mut cumulative = 0.0;

        for (i, point) in dataset.data.iter().enumerate() {
            let value = point.y.max(0.0);
            let percentage = value / total;

            let start_value = cumulative;
            let end_value = cumulative + percentage;

            let label = if i < self.data.labels.len() {
                self.data.labels[i].clone()
            } else {
                format!("Slice {}", i + 1)
            };

            let color = get_color(i);

            self.slices.push(SliceInfo {
                start_value,
                end_value,
                value,
                percentage,
                color,
                label,
            });

            cumulative = end_value;
        }
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

    fn draw_slices(&mut self, cx: &mut Cx2d) {
        let progress = self.animator.get_progress();
        let center = self.coord.center();
        let outer_radius = self.coord.outer_radius();
        let inner_radius = self.coord.inner_radius();

        for (i, slice) in self.slices.iter().enumerate() {
            let is_hovered = self.hovered_slice >= 0 && self.hovered_slice as usize == i;

            // Apply animation - slice grows from start
            let animated_end = slice.start_value + (slice.end_value - slice.start_value) * progress;

            // Skip if slice is too small
            if (animated_end - slice.start_value).abs() < 0.001 {
                continue;
            }

            // Calculate angles
            let start_angle = -PI / 2.0 + slice.start_value * 2.0 * PI;
            let end_angle = -PI / 2.0 + animated_end * 2.0 * PI;

            // Offset hovered slice slightly
            let draw_center = if is_hovered {
                let mid_angle = (start_angle + end_angle) / 2.0;
                let offset = 10.0;
                dvec2(
                    center.x + offset * mid_angle.cos(),
                    center.y + offset * mid_angle.sin(),
                )
            } else {
                center
            };

            // Set color with hover effect
            let base_color = if is_hovered {
                lighten(slice.color, 0.1)
            } else {
                slice.color
            };
            self.draw_arc.color = base_color;

            // Apply gradient if enabled
            if self.gradient_enabled {
                // Create gradient colors: inner is lighter, outer is the base color
                let inner_color = lighten(base_color, 0.3);
                let outer_color = darken(base_color, 0.1);

                if self.gradient_type == 0 {
                    // Radial gradient
                    self.draw_arc.set_radial_gradient(inner_color, outer_color);
                } else {
                    // Angular gradient
                    self.draw_arc.set_angular_gradient(inner_color, outer_color);
                }
            } else {
                self.draw_arc.disable_gradient();
            }

            // Set arc parameters
            self.draw_arc.set_arc(start_angle, end_angle - start_angle, inner_radius, outer_radius);

            // Draw the arc as a rect covering the pie area
            let rect = Rect {
                pos: dvec2(draw_center.x - outer_radius, draw_center.y - outer_radius),
                size: dvec2(outer_radius * 2.0, outer_radius * 2.0),
            };
            self.draw_arc.draw_arc(cx, rect);
        }
    }

    fn handle_mouse_move(&mut self, cx: &mut Cx, pos: DVec2) {
        let old_hovered = self.hovered_slice;
        self.hovered_slice = -1;

        if !self.coord.contains(pos) {
            if old_hovered != self.hovered_slice {
                self.redraw(cx);
            }
            return;
        }

        // Find which slice contains this point
        let (angle, _radius) = self.coord.pixel_to_polar(pos);

        // Convert angle to value (0-1)
        let mut normalized_angle = angle + PI / 2.0; // Adjust for our start angle
        if normalized_angle < 0.0 {
            normalized_angle += 2.0 * PI;
        }
        if normalized_angle > 2.0 * PI {
            normalized_angle -= 2.0 * PI;
        }
        let value = normalized_angle / (2.0 * PI);

        for (i, slice) in self.slices.iter().enumerate() {
            if value >= slice.start_value && value < slice.end_value {
                self.hovered_slice = i as i32;
                break;
            }
        }

        if old_hovered != self.hovered_slice {
            self.redraw(cx);
        }
    }

    fn handle_mouse_down(&mut self, _cx: &mut Cx, pos: DVec2) {
        if self.hovered_slice >= 0 {
            let slice = &self.slices[self.hovered_slice as usize];
            log!("Slice clicked: {} ({}%)", slice.label, (slice.percentage * 100.0).round());
        }
    }
}

impl PieChartRef {
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

    pub fn set_doughnut(&self, is_doughnut: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_doughnut(is_doughnut);
        }
    }

    pub fn set_radial_gradient(&self, enabled: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_radial_gradient(enabled);
        }
    }

    pub fn set_angular_gradient(&self, enabled: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_angular_gradient(enabled);
        }
    }

    pub fn disable_gradient(&self) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.disable_gradient();
        }
    }

    pub fn redraw(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.redraw(cx);
        }
    }
}
