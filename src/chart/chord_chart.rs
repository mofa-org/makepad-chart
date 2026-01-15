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
    use crate::element::triangle::DrawTriangle;

    pub ChordChart = {{ChordChart}} {
        width: Fill,
        height: Fill,
    }
}

/// Data structure for chord diagrams
#[derive(Clone, Debug, Default)]
pub struct ChordData {
    pub labels: Vec<String>,
    pub matrix: Vec<Vec<f64>>,
}

impl ChordData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_labels<S: Into<String>>(mut self, labels: Vec<S>) -> Self {
        self.labels = labels.into_iter().map(|s| s.into()).collect();
        self
    }

    pub fn with_matrix(mut self, matrix: Vec<Vec<f64>>) -> Self {
        self.matrix = matrix;
        self
    }
}

/// Computed group arc for layout
#[derive(Clone, Debug)]
struct GroupArc {
    #[allow(dead_code)]
    index: usize,
    start_angle: f64,
    end_angle: f64,
    #[allow(dead_code)]
    value: f64,
    color: Vec4,
    #[allow(dead_code)]
    label: String,
    /// Track how much of this group's angle has been consumed by chords
    source_offset: f64,
    target_offset: f64,
}

/// Computed chord for layout
#[derive(Clone, Debug)]
struct Chord {
    source_index: usize,
    #[allow(dead_code)]
    target_index: usize,
    source_start: f64,
    source_end: f64,
    target_start: f64,
    target_end: f64,
    #[allow(dead_code)]
    value: f64,
}

#[derive(Live, LiveHook, Widget)]
pub struct ChordChart {
    #[live]
    #[deref]
    view: View,

    #[live]
    draw_arc: DrawArc,

    #[live]
    draw_ribbon: DrawTriangle,

    #[rust]
    chord_data: ChordData,

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

    #[rust(0.0)]
    padding: f64,

    /// Gap between groups in radians
    #[rust(0.04)]
    gap_angle: f64,

    /// Outer arc thickness ratio (0.0-1.0)
    #[rust(0.08)]
    arc_thickness: f64,

    #[rust(-1)]
    hovered_group: i32,

    #[rust(-1)]
    hovered_chord: i32,

    /// Enable gradient on ribbons (radial from center)
    #[rust(false)]
    gradient_enabled: bool,

    /// Enable directed mode (asymmetric ribbons - source wider, target narrower)
    #[rust(false)]
    directed_mode: bool,

    /// Enable arc gradient
    #[rust(false)]
    arc_gradient_enabled: bool,

    /// Computed layout
    #[rust]
    groups: Vec<GroupArc>,

    #[rust]
    chords: Vec<Chord>,
}

impl Widget for ChordChart {
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
            self.update_layout(rect);

            if !self.initialized {
                self.compute_chord_layout();
                self.start_animation(cx);
                self.initialized = true;
            }

            self.draw_chords(cx);
            self.draw_group_arcs(cx);
        }

        DrawStep::done()
    }
}

impl ChordChart {
    pub fn set_data(&mut self, data: ChordData) {
        self.chord_data = data;
        self.initialized = false;
    }

    pub fn set_options(&mut self, options: ChartOptions) {
        self.options = options;
    }

    pub fn set_gap_angle(&mut self, gap: f64) {
        self.gap_angle = gap;
        self.initialized = false;
    }

    pub fn set_arc_thickness(&mut self, thickness: f64) {
        self.arc_thickness = thickness.clamp(0.01, 0.5);
        self.initialized = false;
    }

    /// Enable gradient on ribbons (radial from center to edges)
    pub fn set_gradient(&mut self, enabled: bool) {
        self.gradient_enabled = enabled;
    }

    /// Enable directed mode (source end wider, target end narrower like arrows)
    pub fn set_directed(&mut self, enabled: bool) {
        self.directed_mode = enabled;
    }

    /// Enable gradient on outer arcs
    pub fn set_arc_gradient(&mut self, enabled: bool) {
        self.arc_gradient_enabled = enabled;
    }

    fn update_layout(&mut self, rect: Rect) {
        let size = rect.size.x.min(rect.size.y) - self.padding * 2.0;
        self.radius = size / 2.0;
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

    pub fn replay_animation(&mut self, cx: &mut Cx) {
        self.initialized = false;
        self.animator.reset();

        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(self.options.animation.duration)
            .with_easing(self.options.animation.easing);
        self.animator.start(time);
        // Don't set initialized = true here, let draw_walk call compute_chord_layout

        cx.new_next_frame();
        self.redraw(cx);
    }

    pub fn is_animating(&self) -> bool {
        self.animator.is_running()
    }

    fn compute_chord_layout(&mut self) {
        self.groups.clear();
        self.chords.clear();

        let n = self.chord_data.matrix.len();
        if n == 0 {
            return;
        }

        // Calculate row totals (outgoing from each group)
        let mut group_totals: Vec<f64> = vec![0.0; n];
        for i in 0..n {
            for j in 0..n {
                if i < self.chord_data.matrix.len() && j < self.chord_data.matrix[i].len() {
                    group_totals[i] += self.chord_data.matrix[i][j];
                }
            }
        }

        let total_value: f64 = group_totals.iter().sum();
        if total_value == 0.0 {
            return;
        }

        // Calculate available angle (after gaps)
        let total_gap = self.gap_angle * n as f64;
        let available_angle = 2.0 * PI - total_gap;

        // Assign angles to groups
        let mut current_angle = -PI / 2.0;
        for i in 0..n {
            let group_angle = (group_totals[i] / total_value) * available_angle;

            let label = if i < self.chord_data.labels.len() {
                self.chord_data.labels[i].clone()
            } else {
                format!("Group {}", i + 1)
            };

            self.groups.push(GroupArc {
                index: i,
                start_angle: current_angle,
                end_angle: current_angle + group_angle,
                value: group_totals[i],
                color: get_color(i),
                label,
                source_offset: 0.0,
                target_offset: 0.0,
            });

            current_angle += group_angle + self.gap_angle;
        }

        // Generate chords from matrix
        for i in 0..n {
            for j in 0..n {
                if i >= self.chord_data.matrix.len() || j >= self.chord_data.matrix[i].len() {
                    continue;
                }

                let value = self.chord_data.matrix[i][j];
                if value <= 0.0 {
                    continue;
                }

                // Calculate the angular size of this chord at each end
                let source_angle_size = if group_totals[i] > 0.0 {
                    (value / group_totals[i]) * (self.groups[i].end_angle - self.groups[i].start_angle)
                } else {
                    0.0
                };

                let target_angle_size = if group_totals[j] > 0.0 {
                    (value / group_totals[j]) * (self.groups[j].end_angle - self.groups[j].start_angle)
                } else {
                    0.0
                };

                // Calculate positions using offsets
                let source_start = self.groups[i].start_angle + self.groups[i].source_offset;
                let source_end = source_start + source_angle_size;

                let target_start = self.groups[j].start_angle + self.groups[j].target_offset;
                let target_end = target_start + target_angle_size;

                // Update offsets
                self.groups[i].source_offset += source_angle_size;
                self.groups[j].target_offset += target_angle_size;

                self.chords.push(Chord {
                    source_index: i,
                    target_index: j,
                    source_start,
                    source_end,
                    target_start,
                    target_end,
                    value,
                });
            }
        }
    }

    fn draw_group_arcs(&mut self, cx: &mut Cx2d) {
        let progress = self.animator.get_progress();

        let outer_radius = self.radius;
        let inner_radius = self.radius * (1.0 - self.arc_thickness);

        // Collect arc info to avoid borrow issues
        let arc_info: Vec<_> = self.groups.iter().enumerate().map(|(i, group)| {
            let is_hovered = self.hovered_group >= 0 && self.hovered_group as usize == i;
            let sweep = (group.end_angle - group.start_angle) * progress;
            let color = if is_hovered {
                lighten(group.color, 0.2)
            } else {
                group.color
            };
            (group.start_angle, sweep, color)
        }).collect();

        for (start_angle, sweep, color) in arc_info {
            if sweep < 0.001 {
                continue;
            }

            self.draw_arc.color = color;

            if self.arc_gradient_enabled {
                let inner_color = lighten(color, 0.3);
                self.draw_arc.set_radial_gradient(inner_color, color);
            } else {
                self.draw_arc.disable_gradient();
            }

            self.draw_arc.set_arc(start_angle, sweep, inner_radius, outer_radius);

            let rect = Rect {
                pos: dvec2(self.center.x - outer_radius, self.center.y - outer_radius),
                size: dvec2(outer_radius * 2.0, outer_radius * 2.0),
            };
            self.draw_arc.draw_arc(cx, rect);
        }
    }

    fn draw_chords(&mut self, cx: &mut Cx2d) {
        let progress = self.animator.get_progress();

        let inner_radius = self.radius * (1.0 - self.arc_thickness);

        // Collect chord drawing info to avoid borrow issues
        let draw_info: Vec<_> = self.chords.iter().enumerate().map(|(chord_idx, chord)| {
            let is_hovered = self.hovered_chord >= 0 && self.hovered_chord as usize == chord_idx;

            let base_color = if chord.source_index < self.groups.len() {
                self.groups[chord.source_index].color
            } else {
                get_color(chord.source_index)
            };

            let alpha = if is_hovered { 0.8 } else { 0.5 };
            let color = vec4(base_color.x, base_color.y, base_color.z, alpha as f32);

            (chord.source_start, chord.source_end, chord.target_start, chord.target_end, color, base_color)
        }).collect();

        // Animate the ribbon by scaling the radius
        let animated_radius = inner_radius * progress;

        if animated_radius < 1.0 {
            return;
        }

        let directed = self.directed_mode;
        let gradient = self.gradient_enabled;

        for (source_start, source_end, target_start, target_end, color, base_color) in draw_info {
            if directed {
                self.draw_directed_ribbon(
                    cx,
                    source_start,
                    source_end,
                    target_start,
                    target_end,
                    animated_radius,
                    color,
                    base_color,
                    gradient,
                );
            } else {
                self.draw_ribbon_shape(
                    cx,
                    source_start,
                    source_end,
                    target_start,
                    target_end,
                    animated_radius,
                    color,
                    base_color,
                    gradient,
                );
            }
        }
    }

    fn draw_ribbon_shape(
        &mut self,
        cx: &mut Cx2d,
        source_start: f64,
        source_end: f64,
        target_start: f64,
        target_end: f64,
        radius: f64,
        color: Vec4,
        base_color: Vec4,
        gradient: bool,
    ) {
        let segments = 16;

        // Source arc points
        let source_points: Vec<DVec2> = (0..=segments / 2)
            .map(|i| {
                let t = i as f64 / (segments / 2) as f64;
                let angle = source_start + (source_end - source_start) * t;
                dvec2(
                    self.center.x + radius * angle.cos(),
                    self.center.y + radius * angle.sin(),
                )
            })
            .collect();

        // Target arc points (reversed for proper winding)
        let target_points: Vec<DVec2> = (0..=segments / 2)
            .rev()
            .map(|i| {
                let t = i as f64 / (segments / 2) as f64;
                let angle = target_start + (target_end - target_start) * t;
                dvec2(
                    self.center.x + radius * angle.cos(),
                    self.center.y + radius * angle.sin(),
                )
            })
            .collect();

        // Bezier curves connecting
        let curve1 = self.bezier_curve(
            *source_points.last().unwrap(),
            *target_points.first().unwrap(),
            segments / 2,
        );

        let curve2 = self.bezier_curve(
            *target_points.last().unwrap(),
            *source_points.first().unwrap(),
            segments / 2,
        );

        // Combine all points into polygon
        let mut polygon: Vec<DVec2> = Vec::new();
        polygon.extend(&source_points);
        polygon.extend(&curve1[1..]);
        polygon.extend(&target_points[1..]);
        polygon.extend(&curve2[1..curve2.len() - 1]);

        // Draw as triangles from center
        self.draw_ribbon.color = color;

        if gradient {
            let center_color = vec4(
                (base_color.x + 0.3).min(1.0),
                (base_color.y + 0.3).min(1.0),
                (base_color.z + 0.3).min(1.0),
                color.w * 0.8,
            );
            let outer_color = vec4(
                base_color.x * 0.7,
                base_color.y * 0.7,
                base_color.z * 0.7,
                color.w * 0.4,
            );
            self.draw_ribbon.set_radial_gradient(center_color, outer_color);
        } else {
            self.draw_ribbon.disable_gradient();
        }

        for i in 0..polygon.len() {
            let p1 = polygon[i];
            let p2 = polygon[(i + 1) % polygon.len()];
            self.draw_ribbon.draw_triangle(cx, self.center, p1, p2);
        }
    }

    fn draw_directed_ribbon(
        &mut self,
        cx: &mut Cx2d,
        source_start: f64,
        source_end: f64,
        target_start: f64,
        target_end: f64,
        radius: f64,
        color: Vec4,
        base_color: Vec4,
        gradient: bool,
    ) {
        let segments = 16;

        // Source arc points (full width)
        let source_points: Vec<DVec2> = (0..=segments / 2)
            .map(|i| {
                let t = i as f64 / (segments / 2) as f64;
                let angle = source_start + (source_end - source_start) * t;
                dvec2(
                    self.center.x + radius * angle.cos(),
                    self.center.y + radius * angle.sin(),
                )
            })
            .collect();

        // Target: narrower point for directed/arrow effect
        // Calculate target midpoint angle
        let target_mid = (target_start + target_end) / 2.0;
        let target_width = (target_end - target_start) * 0.3; // 30% of original width

        let target_points: Vec<DVec2> = (0..=segments / 4)
            .rev()
            .map(|i| {
                let t = i as f64 / (segments / 4) as f64;
                let angle = (target_mid - target_width / 2.0) + target_width * t;
                dvec2(
                    self.center.x + radius * angle.cos(),
                    self.center.y + radius * angle.sin(),
                )
            })
            .collect();

        // Bezier curves connecting
        let curve1 = self.bezier_curve(
            *source_points.last().unwrap(),
            *target_points.first().unwrap(),
            segments / 2,
        );

        let curve2 = self.bezier_curve(
            *target_points.last().unwrap(),
            *source_points.first().unwrap(),
            segments / 2,
        );

        // Combine all points into polygon
        let mut polygon: Vec<DVec2> = Vec::new();
        polygon.extend(&source_points);
        polygon.extend(&curve1[1..]);
        polygon.extend(&target_points[1..]);
        polygon.extend(&curve2[1..curve2.len() - 1]);

        // Draw as triangles from center
        self.draw_ribbon.color = color;

        if gradient {
            let center_color = vec4(
                (base_color.x + 0.3).min(1.0),
                (base_color.y + 0.3).min(1.0),
                (base_color.z + 0.3).min(1.0),
                color.w * 0.8,
            );
            let outer_color = vec4(
                base_color.x * 0.7,
                base_color.y * 0.7,
                base_color.z * 0.7,
                color.w * 0.4,
            );
            self.draw_ribbon.set_radial_gradient(center_color, outer_color);
        } else {
            self.draw_ribbon.disable_gradient();
        }

        for i in 0..polygon.len() {
            let p1 = polygon[i];
            let p2 = polygon[(i + 1) % polygon.len()];
            self.draw_ribbon.draw_triangle(cx, self.center, p1, p2);
        }
    }

    fn bezier_curve(&self, start: DVec2, end: DVec2, segments: usize) -> Vec<DVec2> {
        // Quadratic bezier with control point at center
        let control = self.center;

        (0..=segments)
            .map(|i| {
                let t = i as f64 / segments as f64;
                let t2 = t * t;
                let mt = 1.0 - t;
                let mt2 = mt * mt;

                dvec2(
                    mt2 * start.x + 2.0 * mt * t * control.x + t2 * end.x,
                    mt2 * start.y + 2.0 * mt * t * control.y + t2 * end.y,
                )
            })
            .collect()
    }

    fn handle_mouse_move(&mut self, cx: &mut Cx, pos: DVec2) {
        let old_group = self.hovered_group;
        let old_chord = self.hovered_chord;
        self.hovered_group = -1;
        self.hovered_chord = -1;

        let dx = pos.x - self.center.x;
        let dy = pos.y - self.center.y;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist > self.radius {
            if old_group != self.hovered_group || old_chord != self.hovered_chord {
                self.redraw(cx);
            }
            return;
        }

        let angle = dy.atan2(dx);
        let outer_radius = self.radius;
        let inner_radius = self.radius * (1.0 - self.arc_thickness);

        // Check if hovering over group arcs
        if dist >= inner_radius && dist <= outer_radius {
            for (i, group) in self.groups.iter().enumerate() {
                if self.is_angle_in_range(angle, group.start_angle, group.end_angle) {
                    self.hovered_group = i as i32;
                    break;
                }
            }
        }

        if old_group != self.hovered_group || old_chord != self.hovered_chord {
            self.redraw(cx);
        }
    }

    fn is_angle_in_range(&self, angle: f64, start: f64, end: f64) -> bool {
        let mut check_angle = angle;
        let mut range_start = start;
        let mut range_end = end;

        while check_angle < 0.0 {
            check_angle += 2.0 * PI;
        }
        while range_start < 0.0 {
            range_start += 2.0 * PI;
        }
        while range_end < 0.0 {
            range_end += 2.0 * PI;
        }

        check_angle >= range_start && check_angle < range_end
    }
}

impl ChordChartRef {
    pub fn set_data(&self, data: ChordData) {
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

    pub fn set_gap_angle(&self, gap: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_gap_angle(gap);
        }
    }

    pub fn set_arc_thickness(&self, thickness: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_arc_thickness(thickness);
        }
    }

    pub fn set_gradient(&self, enabled: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_gradient(enabled);
        }
    }

    pub fn set_directed(&self, enabled: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_directed(enabled);
        }
    }

    pub fn set_arc_gradient(&self, enabled: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_arc_gradient(enabled);
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
