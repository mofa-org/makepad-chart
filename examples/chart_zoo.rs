use makepad_widgets::*;
use makepad_charts::*;
use makepad_charts::chart::bar_chart::BarChart;
use makepad_charts::chart::line_chart::{LineChart, SteppedMode};
use makepad_charts::chart::pie_chart::PieChart;
use makepad_charts::chart::scatter_chart::ScatterChart;
use makepad_charts::chart::radar_chart::RadarChart;
use makepad_charts::chart::polar_area_chart::PolarAreaChart;
use makepad_charts::chart::bubble_chart::BubbleChart;
use makepad_charts::chart::horizontal_bar_chart::HorizontalBarChart;
use makepad_charts::chart::combo_chart::{ComboChart, DatasetType};
use makepad_charts::component::legend::{ChartLegend, LegendItemData};

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use makepad_charts::chart::bar_chart::BarChart;
    use makepad_charts::chart::line_chart::LineChart;
    use makepad_charts::chart::pie_chart::PieChart;
    use makepad_charts::chart::scatter_chart::ScatterChart;
    use makepad_charts::chart::radar_chart::RadarChart;
    use makepad_charts::chart::polar_area_chart::PolarAreaChart;
    use makepad_charts::chart::bubble_chart::BubbleChart;
    use makepad_charts::chart::horizontal_bar_chart::HorizontalBarChart;
    use makepad_charts::chart::combo_chart::ComboChart;
    use makepad_charts::component::legend::ChartLegend;

    FONT_MANROPE = {
        font_family: {
            latin = font("crate://self/resources/Manrope-Regular.ttf", 0.0, 0.0),
        }
    }

    ChartCard = <RoundedView> {
        width: Fill,
        height: 280,
        margin: 8,
        padding: 12,

        show_bg: true,
        draw_bg: {
            color: #ffffff,
            border_radius: 8.0,
        }

        flow: Overlay,
    }

    ChartTitle = <View> {
        width: Fill,
        height: Fill,
        align: {x: 0.5, y: 0.0},
        padding: {top: 0},

        label = <RoundedView> {
            width: Fit,
            height: Fit,
            padding: {left: 12, right: 12, top: 6, bottom: 6},
            show_bg: true,
            draw_bg: {
                color: #ffffffdd,
                border_radius: 4.0,
            }

            label = <Label> {
                width: Fit,
                height: Fit,
                draw_text: {
                    color: #333333,
                    text_style: <FONT_MANROPE> { font_size: 13.0 }
                }
            }
        }
    }

    App = {{App}} {
        ui: <Window> {
            show_bg: true,
            width: Fill,
            height: Fill,

            draw_bg: {
                color: #f0f0f0
            }

            body = <ScrollXYView> {
                flow: Down,
                spacing: 0,
                padding: 20,

                // Header
                <View> {
                    width: Fill,
                    height: Fit,
                    margin: {bottom: 20},
                    flow: Down,
                    align: {x: 0.5},
                    spacing: 10,

                    <Label> {
                        text: "Makepad Charts Zoo"
                        draw_text: {
                            color: #333333,
                            text_style: <FONT_MANROPE> { font_size: 28.0 }
                        }
                    }
                    <Label> {
                        text: "GPU-accelerated charting library - All Chart.js types"
                        draw_text: {
                            color: #666666,
                            text_style: <FONT_MANROPE> { font_size: 14.0 }
                        }
                    }

                    replay_button = <Button> {
                        width: Fit,
                        height: Fit,
                        padding: {left: 20, right: 20, top: 10, bottom: 10},
                        margin: {top: 10},

                        draw_bg: {
                            instance color: #4A90D9,
                            instance color_hover: #5A9FE8,
                            instance border_radius: 6.0,

                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                let color = mix(self.color, self.color_hover, self.hover);
                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                                sdf.fill(color);
                                return sdf.result;
                            }
                        }
                        draw_text: {
                            color: #ffffff,
                            text_style: <FONT_MANROPE> { font_size: 14.0 }
                        }
                        text: "Replay Animations"
                    }
                }

                // Row 1: Bar, Line, Pie, Doughnut
                <View> {
                    width: Fill,
                    height: Fit,
                    flow: Right,
                    spacing: 0,

                    <ChartCard> {
                        bar_chart = <BarChart> { width: Fill, height: Fill }
                        <ChartTitle> { label = { label = { text: "Bar Chart" } } }
                    }

                    <ChartCard> {
                        line_chart = <LineChart> { width: Fill, height: Fill }
                        <ChartTitle> { label = { label = { text: "Line Chart" } } }
                    }

                    <ChartCard> {
                        pie_chart = <PieChart> { width: Fill, height: Fill }
                        <ChartTitle> { label = { label = { text: "Pie Chart" } } }
                    }

                    <ChartCard> {
                        doughnut_chart = <PieChart> { width: Fill, height: Fill }
                        <ChartTitle> { label = { label = { text: "Doughnut Chart" } } }
                    }
                }

                // Row 2: Scatter, Radar, Polar Area, Bubble
                <View> {
                    width: Fill,
                    height: Fit,
                    flow: Right,
                    spacing: 0,

                    <ChartCard> {
                        scatter_chart = <ScatterChart> { width: Fill, height: Fill }
                        <ChartTitle> { label = { label = { text: "Scatter Chart" } } }
                    }

                    <ChartCard> {
                        radar_chart = <RadarChart> { width: Fill, height: Fill }
                        <ChartTitle> { label = { label = { text: "Radar Chart" } } }
                    }

                    <ChartCard> {
                        polar_chart = <PolarAreaChart> { width: Fill, height: Fill }
                        <ChartTitle> { label = { label = { text: "Polar Area Chart" } } }
                    }

                    <ChartCard> {
                        bubble_chart = <BubbleChart> { width: Fill, height: Fill }
                        <ChartTitle> { label = { label = { text: "Bubble Chart" } } }
                    }
                }

                // Row 3: Multi-Line, Multi-Bar, Horizontal Bar, Area
                <View> {
                    width: Fill,
                    height: Fit,
                    flow: Right,
                    spacing: 0,

                    <ChartCard> {
                        multi_line_chart = <LineChart> { width: Fill, height: Fill }
                        <ChartTitle> { label = { label = { text: "Multi-Line Chart" } } }
                    }

                    <ChartCard> {
                        multi_bar_chart = <BarChart> { width: Fill, height: Fill }
                        multi_bar_legend = <ChartLegend> {}
                        <ChartTitle> { label = { label = { text: "Multi-Series Bar" } } }
                    }

                    <ChartCard> {
                        horizontal_bar_chart = <HorizontalBarChart> { width: Fill, height: Fill }
                        <ChartTitle> { label = { label = { text: "Horizontal Bar" } } }
                    }

                    <ChartCard> {
                        area_chart = <LineChart> { width: Fill, height: Fill }
                        <ChartTitle> { label = { label = { text: "Area Chart" } } }
                    }
                }

                // Row 4: Stacked Bar, Combo, Stepped Line, Multi-Radar
                <View> {
                    width: Fill,
                    height: Fit,
                    flow: Right,
                    spacing: 0,

                    <ChartCard> {
                        stacked_bar_chart = <BarChart> { width: Fill, height: Fill }
                        <ChartTitle> { label = { label = { text: "Stacked Bar" } } }
                    }

                    <ChartCard> {
                        combo_chart = <ComboChart> { width: Fill, height: Fill }
                        <ChartTitle> { label = { label = { text: "Combo Bar/Line" } } }
                    }

                    <ChartCard> {
                        stepped_line_chart = <LineChart> { width: Fill, height: Fill }
                        <ChartTitle> { label = { label = { text: "Stepped Line" } } }
                    }

                    <ChartCard> {
                        floating_bar_chart = <BarChart> { width: Fill, height: Fill }
                        <ChartTitle> { label = { label = { text: "Floating Bars" } } }
                    }
                }

                // Footer
                <View> {
                    width: Fill,
                    height: Fit,
                    margin: {top: 30},
                    align: {x: 0.5},

                    <Label> {
                        text: "Built with Makepad • GPU Accelerated • Cross Platform"
                        draw_text: {
                            color: #999999,
                            text_style: <FONT_MANROPE> { font_size: 12.0 }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        makepad_charts::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        self.setup_charts(cx);
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(id!(replay_button)).clicked(actions) {
            self.replay_all_animations(cx);
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

impl App {
    fn setup_charts(&mut self, cx: &mut Cx) {
        // Bar Chart - Monthly Sales
        let bar_data = ChartData::new()
            .with_labels(vec!["Jan", "Feb", "Mar", "Apr", "May", "Jun"])
            .add_dataset(
                Dataset::new("Sales 2024")
                    .with_data(vec![65.0, 59.0, 80.0, 81.0, 56.0, 72.0])
            );

        if let Some(mut bar) = self.ui.widget(id!(bar_chart)).borrow_mut::<BarChart>() {
            bar.set_data(bar_data);
            bar.set_options(ChartOptions::new()
                .with_begin_at_zero(true)
                .with_animation_duration(400.0));
            bar.set_delay_animation(true);
            bar.set_delay_timing(80.0, 40.0);  // 80ms per bar, 40ms per dataset
        }

        // Line Chart - Temperature
        let line_data = ChartData::new()
            .with_labels(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"])
            .add_dataset(
                Dataset::new("Temperature °C")
                    .with_data(vec![22.0, 19.0, 24.0, 25.0, 23.0, 26.0, 28.0])
            );

        if let Some(mut line) = self.ui.widget(id!(line_chart)).borrow_mut::<LineChart>() {
            line.set_data(line_data);
            line.set_options(ChartOptions::new().with_begin_at_zero(false));
        }

        // Pie Chart - Market Share
        let pie_data = ChartData::new()
            .with_labels(vec!["Chrome", "Safari", "Firefox", "Edge", "Other"])
            .add_dataset(
                Dataset::new("Browser Share")
                    .with_data(vec![65.0, 18.0, 8.0, 5.0, 4.0])
            );

        if let Some(mut pie) = self.ui.widget(id!(pie_chart)).borrow_mut::<PieChart>() {
            pie.set_data(pie_data);
            pie.set_radial_gradient(true);  // Enable radial gradient
        }

        // Doughnut Chart - Expenses
        let doughnut_data = ChartData::new()
            .with_labels(vec!["Housing", "Food", "Transport", "Entertainment", "Utilities"])
            .add_dataset(
                Dataset::new("Monthly Expenses")
                    .with_data(vec![1200.0, 600.0, 400.0, 300.0, 200.0])
            );

        if let Some(mut pie) = self.ui.widget(id!(doughnut_chart)).borrow_mut::<PieChart>() {
            pie.set_data(doughnut_data);
            pie.set_doughnut(true);
            pie.set_radial_gradient(true);  // Enable radial gradient
        }

        // Scatter Chart - Height vs Weight
        let scatter_data = ChartData::new()
            .add_dataset(
                Dataset::new("Athletes")
                    .with_xy_data(vec![
                        (165.0, 60.0), (170.0, 68.0), (175.0, 72.0), (180.0, 78.0),
                        (168.0, 65.0), (172.0, 70.0), (178.0, 75.0), (182.0, 82.0),
                        (160.0, 55.0), (185.0, 88.0), (177.0, 73.0), (169.0, 64.0),
                    ])
            );

        if let Some(mut scatter) = self.ui.widget(id!(scatter_chart)).borrow_mut::<ScatterChart>() {
            scatter.set_data(scatter_data);
            scatter.set_options(ChartOptions::new().with_begin_at_zero(false));
        }

        // Radar Chart - Skills Assessment
        let radar_data = ChartData::new()
            .with_labels(vec!["Coding", "Design", "Communication", "Teamwork", "Problem Solving", "Leadership"])
            .add_dataset(
                Dataset::new("Developer A")
                    .with_data(vec![90.0, 60.0, 75.0, 80.0, 85.0, 70.0])
            );

        if let Some(mut radar) = self.ui.widget(id!(radar_chart)).borrow_mut::<RadarChart>() {
            radar.set_data(radar_data);
            radar.set_gradient(true);  // Enable radial gradient fill
        }

        // Polar Area Chart - Revenue by Quarter
        let polar_data = ChartData::new()
            .with_labels(vec!["Q1", "Q2", "Q3", "Q4"])
            .add_dataset(
                Dataset::new("Revenue")
                    .with_data(vec![120.0, 190.0, 150.0, 220.0])
            );

        if let Some(mut polar) = self.ui.widget(id!(polar_chart)).borrow_mut::<PolarAreaChart>() {
            polar.set_data(polar_data);
        }

        // Bubble Chart - Population vs GDP
        let bubble_data = ChartData::new()
            .add_dataset(
                Dataset::new("Countries")
                    .with_bubble_data(vec![
                        (10.0, 20.0, 15.0),  // x=GDP, y=Population, r=Area
                        (15.0, 30.0, 20.0),
                        (25.0, 25.0, 10.0),
                        (30.0, 45.0, 25.0),
                        (40.0, 35.0, 18.0),
                        (20.0, 50.0, 30.0),
                        (35.0, 20.0, 12.0),
                    ])
            );

        if let Some(mut bubble) = self.ui.widget(id!(bubble_chart)).borrow_mut::<BubbleChart>() {
            bubble.set_data(bubble_data);
            bubble.set_options(ChartOptions::new().with_begin_at_zero(false));
        }

        // Multi-Line Chart - Stock Prices
        let multi_line_data = ChartData::new()
            .with_labels(vec!["Jan", "Feb", "Mar", "Apr", "May", "Jun"])
            .add_dataset(
                Dataset::new("AAPL")
                    .with_data(vec![175.0, 182.0, 178.0, 185.0, 190.0, 195.0])
            )
            .add_dataset(
                Dataset::new("GOOGL")
                    .with_data(vec![140.0, 145.0, 142.0, 150.0, 155.0, 160.0])
            )
            .add_dataset(
                Dataset::new("MSFT")
                    .with_data(vec![380.0, 395.0, 388.0, 405.0, 420.0, 430.0])
            );

        if let Some(mut line) = self.ui.widget(id!(multi_line_chart)).borrow_mut::<LineChart>() {
            line.set_data(multi_line_data);
            line.set_options(ChartOptions::new().with_begin_at_zero(false));
        }

        // Multi-Series Bar Chart - Quarterly Revenue
        let multi_bar_data = ChartData::new()
            .with_labels(vec!["Q1", "Q2", "Q3", "Q4"])
            .add_dataset(
                Dataset::new("2023")
                    .with_data(vec![120.0, 150.0, 180.0, 200.0])
            )
            .add_dataset(
                Dataset::new("2024")
                    .with_data(vec![140.0, 170.0, 210.0, 240.0])
            );

        if let Some(mut bar) = self.ui.widget(id!(multi_bar_chart)).borrow_mut::<BarChart>() {
            bar.set_data(multi_bar_data);
            bar.set_options(ChartOptions::new()
                .with_begin_at_zero(true)
                .with_animation_duration(400.0));
            bar.set_delay_animation(true);
            bar.set_delay_timing(80.0, 40.0);
        }

        // Set up legend for multi-series bar chart
        if let Some(mut legend) = self.ui.widget(id!(multi_bar_legend)).borrow_mut::<ChartLegend>() {
            legend.set_items(vec![
                LegendItemData { label: "2023".to_string(), color: get_color(0), hidden: false },
                LegendItemData { label: "2024".to_string(), color: get_color(1), hidden: false },
            ]);
        }

        // Horizontal Bar Chart - with positive and negative values
        let horizontal_bar_data = ChartData::new()
            .with_labels(vec!["January", "February", "March", "April", "May", "June", "July"])
            .add_dataset(
                Dataset::new("Dataset 1")
                    .with_data(vec![65.0, 59.0, -30.0, 81.0, -100.0, -40.0, 10.0])
            )
            .add_dataset(
                Dataset::new("Dataset 2")
                    .with_data(vec![-28.0, 70.0, -15.0, 72.0, -20.0, 50.0, 80.0])
            );

        if let Some(mut hbar) = self.ui.widget(id!(horizontal_bar_chart)).borrow_mut::<HorizontalBarChart>() {
            hbar.set_data(horizontal_bar_data);
            hbar.set_options(ChartOptions::new().with_begin_at_zero(false));
        }

        // Area Chart (Line with fill)
        let area_data = ChartData::new()
            .with_labels(vec!["Jan", "Feb", "Mar", "Apr", "May", "Jun"])
            .add_dataset(
                Dataset::new("Revenue")
                    .with_data(vec![30.0, 45.0, 55.0, 50.0, 65.0, 80.0])
            );

        if let Some(mut line) = self.ui.widget(id!(area_chart)).borrow_mut::<LineChart>() {
            line.set_data(area_data);
            line.set_fill(true);
            line.set_options(ChartOptions::new().with_begin_at_zero(true));
        }

        // Floating Bar Chart - Temperature Range
        let floating_bar_data = ChartData::new()
            .with_labels(vec!["Jan", "Feb", "Mar", "Apr", "May", "Jun"])
            .add_dataset(
                Dataset::new("Temperature Range")
                    .with_floating_data(vec![
                        (-5.0, 8.0),   // Jan: min=-5, max=8
                        (-3.0, 12.0),  // Feb: min=-3, max=12
                        (2.0, 18.0),   // Mar: min=2, max=18
                        (8.0, 24.0),   // Apr: min=8, max=24
                        (14.0, 28.0),  // May: min=14, max=28
                        (18.0, 32.0),  // Jun: min=18, max=32
                    ])
            );

        if let Some(mut bar) = self.ui.widget(id!(floating_bar_chart)).borrow_mut::<BarChart>() {
            bar.set_data(floating_bar_data);
            bar.set_options(ChartOptions::new().with_begin_at_zero(false));
        }

        // Stacked Bar Chart
        let stacked_bar_data = ChartData::new()
            .with_labels(vec!["Q1", "Q2", "Q3", "Q4"])
            .add_dataset(
                Dataset::new("Product A")
                    .with_data(vec![50.0, 60.0, 70.0, 80.0])
            )
            .add_dataset(
                Dataset::new("Product B")
                    .with_data(vec![30.0, 40.0, 35.0, 45.0])
            )
            .add_dataset(
                Dataset::new("Product C")
                    .with_data(vec![20.0, 25.0, 30.0, 35.0])
            );

        if let Some(mut bar) = self.ui.widget(id!(stacked_bar_chart)).borrow_mut::<BarChart>() {
            bar.set_data(stacked_bar_data);
            bar.set_stacked(true);
            bar.set_options(ChartOptions::new().with_begin_at_zero(true));
        }

        // Combo Bar/Line Chart
        let combo_data = ChartData::new()
            .with_labels(vec!["Jan", "Feb", "Mar", "Apr", "May", "Jun"])
            .add_dataset(
                Dataset::new("Sales")
                    .with_data(vec![65.0, 59.0, 80.0, 81.0, 56.0, 72.0])
            )
            .add_dataset(
                Dataset::new("Trend")
                    .with_data(vec![60.0, 65.0, 70.0, 75.0, 68.0, 74.0])
            );

        if let Some(mut combo) = self.ui.widget(id!(combo_chart)).borrow_mut::<ComboChart>() {
            combo.set_data(combo_data);
            combo.set_dataset_types(vec![DatasetType::Bar, DatasetType::Line]);
            combo.set_options(ChartOptions::new().with_begin_at_zero(true));
        }

        // Stepped Line Chart
        let stepped_data = ChartData::new()
            .with_labels(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"])
            .add_dataset(
                Dataset::new("Price")
                    .with_data(vec![10.0, 10.0, 15.0, 15.0, 20.0, 25.0, 25.0])
            );

        if let Some(mut line) = self.ui.widget(id!(stepped_line_chart)).borrow_mut::<LineChart>() {
            line.set_data(stepped_data);
            line.set_stepped(SteppedMode::After);
            line.set_options(ChartOptions::new().with_begin_at_zero(true));
        }

        self.ui.redraw(cx);
    }

    fn replay_all_animations(&mut self, cx: &mut Cx) {
        // Replay bar chart animations
        if let Some(mut bar) = self.ui.widget(id!(bar_chart)).borrow_mut::<BarChart>() {
            bar.replay_animation(cx);
        }
        if let Some(mut bar) = self.ui.widget(id!(multi_bar_chart)).borrow_mut::<BarChart>() {
            bar.replay_animation(cx);
        }
        if let Some(mut bar) = self.ui.widget(id!(stacked_bar_chart)).borrow_mut::<BarChart>() {
            bar.replay_animation(cx);
        }
        if let Some(mut bar) = self.ui.widget(id!(floating_bar_chart)).borrow_mut::<BarChart>() {
            bar.replay_animation(cx);
        }
    }
}

app_main!(App);

fn main() {
    app_main();
}
