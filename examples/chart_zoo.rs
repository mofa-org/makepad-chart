use makepad_widgets::*;
use makepad_charts::*;
use makepad_charts::chart::bar_chart::BarChart;
use makepad_charts::chart::line_chart::{LineChart, SteppedMode, CubicInterpolationMode};
use makepad_charts::chart::pie_chart::PieChart;
use makepad_charts::chart::scatter_chart::ScatterChart;
use makepad_charts::chart::radar_chart::RadarChart;
use makepad_charts::chart::polar_area_chart::PolarAreaChart;
use makepad_charts::chart::bubble_chart::BubbleChart;
use makepad_charts::chart::horizontal_bar_chart::HorizontalBarChart;
use makepad_charts::chart::combo_chart::{ComboChart, DatasetType};
use makepad_charts::chart::chord_chart::{ChordChart, ChordData};

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
    use makepad_charts::chart::chord_chart::ChordChart;

    FONT_MANROPE = {
        font_family: {
            latin = font("crate://self/resources/Manrope-Regular.ttf", 0.0, 0.0),
        }
    }

    // Clickable chart card
    ChartCard = <RoundedView> {
        width: Fill,
        height: 280,
        margin: 8,
        padding: 12,
        cursor: Hand,

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

    // Detail page chart card (larger)
    DetailChartCard = <RoundedView> {
        width: Fill,
        height: 350,
        margin: 10,
        padding: 15,

        show_bg: true,
        draw_bg: {
            color: #ffffff,
            border_radius: 8.0,
        }

        flow: Overlay,
    }

    // Back button style
    BackButton = <Button> {
        width: Fit,
        height: Fit,
        padding: {left: 16, right: 16, top: 8, bottom: 8},

        draw_bg: {
            instance color: #666666,
            instance color_hover: #4A90D9,
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
        text: "< Back to Charts"
    }

    App = {{App}} {
        ui: <Window> {
            show_bg: true,
            width: Fill,
            height: Fill,

            draw_bg: {
                color: #f0f0f0
            }

            window: {
                // Set reasonable default window size
                inner_size: vec2(1200, 800)
            }

            body = <View> {
                width: Fill,
                height: Fill,
                flow: Overlay,

                // Main Page - Chart Grid
                main_page = <ScrollXYView> {
                    visible: true,
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
                            text: "Click any chart to see variations"
                            draw_text: {
                                color: #666666,
                                text_style: <FONT_MANROPE> { font_size: 14.0 }
                            }
                        }
                    }

                    // Row 1: Bar, Line, Pie, Scatter
                    <View> {
                        width: Fill,
                        height: Fit,
                        flow: Right,
                        spacing: 0,

                        bar_card = <ChartCard> {
                            bar_chart = <BarChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Bar Chart" } } }
                        }

                        line_card = <ChartCard> {
                            line_chart = <LineChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Line Chart" } } }
                        }

                        pie_card = <ChartCard> {
                            pie_chart = <PieChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Pie Chart" } } }
                        }

                        scatter_card = <ChartCard> {
                            scatter_chart = <ScatterChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Scatter Chart" } } }
                        }
                    }

                    // Row 2: Radar, Polar, Bubble, Combo
                    <View> {
                        width: Fill,
                        height: Fit,
                        flow: Right,
                        spacing: 0,

                        radar_card = <ChartCard> {
                            radar_chart = <RadarChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Radar Chart" } } }
                        }

                        polar_card = <ChartCard> {
                            polar_chart = <PolarAreaChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Polar Area" } } }
                        }

                        bubble_card = <ChartCard> {
                            bubble_chart = <BubbleChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Bubble Chart" } } }
                        }

                        combo_card = <ChartCard> {
                            combo_chart = <ComboChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Combo Chart" } } }
                        }
                    }

                    // Row 3: Chord, Horizontal Bar
                    <View> {
                        width: Fill,
                        height: Fit,
                        flow: Right,
                        spacing: 0,

                        chord_card = <ChartCard> {
                            chord_chart = <ChordChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Chord Diagram" } } }
                        }

                        horizontal_bar_card = <ChartCard> {
                            horizontal_bar_chart = <HorizontalBarChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Horizontal Bar" } } }
                        }

                        <View> { width: Fill, height: Fill }
                        <View> { width: Fill, height: Fill }
                    }

                    // Footer
                    <View> {
                        width: Fill,
                        height: Fit,
                        margin: {top: 30},
                        align: {x: 0.5},

                        <Label> {
                            text: "Built with Makepad - GPU Accelerated"
                            draw_text: {
                                color: #999999,
                                text_style: <FONT_MANROPE> { font_size: 12.0 }
                            }
                        }
                    }
                }

                // Detail Page - Line Chart Variations
                line_detail_page = <ScrollXYView> {
                    visible: false,
                    flow: Down,
                    spacing: 0,
                    padding: 20,

                    <View> {
                        width: Fill,
                        height: Fit,
                        flow: Right,
                        spacing: 20,
                        margin: {bottom: 20},
                        align: {y: 0.5},

                        back_button_line = <BackButton> {}
                        <Label> {
                            text: "Line Chart Variations"
                            draw_text: {
                                color: #333333,
                                text_style: <FONT_MANROPE> { font_size: 24.0 }
                            }
                        }
                    }

                    <View> {
                        width: Fill, height: Fit, flow: Right, spacing: 0,

                        <DetailChartCard> {
                            detail_line_basic = <LineChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Basic Line" } } }
                        }
                        <DetailChartCard> {
                            detail_line_cubic = <LineChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Cubic Interpolation" } } }
                        }
                        <DetailChartCard> {
                            detail_line_monotone = <LineChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Monotone Cubic" } } }
                        }
                    }

                    <View> {
                        width: Fill, height: Fit, flow: Right, spacing: 0,

                        <DetailChartCard> {
                            detail_line_stepped_before = <LineChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Stepped (Before)" } } }
                        }
                        <DetailChartCard> {
                            detail_line_stepped_after = <LineChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Stepped (After)" } } }
                        }
                        <DetailChartCard> {
                            detail_line_stepped_middle = <LineChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Stepped (Middle)" } } }
                        }
                    }

                    <View> {
                        width: Fill, height: Fit, flow: Right, spacing: 0,

                        <DetailChartCard> {
                            detail_line_area = <LineChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Area Fill" } } }
                        }
                        <DetailChartCard> {
                            detail_line_multi = <LineChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Multi-Series" } } }
                        }
                        <DetailChartCard> {
                            detail_line_smooth_area = <LineChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Smooth Area" } } }
                        }
                    }

                    <View> {
                        width: Fill, height: Fit, flow: Right, spacing: 0,

                        <DetailChartCard> {
                            detail_line_stock_market = <LineChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Stock Market (Progressive)" } } }
                        }
                        <DetailChartCard> {
                            detail_line_radio_wave_2 = <LineChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Dense Wave" } } }
                        }
                        <DetailChartCard> {
                            detail_line_radio_wave_3 = <LineChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Audio Signal" } } }
                        }
                    }

                    <View> {
                        width: Fill, height: Fit, flow: Right, spacing: 0,

                        <DetailChartCard> {
                            detail_line_gradient = <LineChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Gradient Area Fill" } } }
                        }
                        <DetailChartCard> {
                            detail_line_gradient_smooth = <LineChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Smooth Gradient" } } }
                        }
                        <DetailChartCard> {
                            detail_line_gradient_multi = <LineChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Multi-Series Gradient" } } }
                        }
                    }
                }

                // Detail Page - Bar Chart Variations
                bar_detail_page = <ScrollXYView> {
                    visible: false,
                    flow: Down,
                    spacing: 0,
                    padding: 20,

                    <View> {
                        width: Fill,
                        height: Fit,
                        flow: Right,
                        spacing: 20,
                        margin: {bottom: 20},
                        align: {y: 0.5},

                        back_button_bar = <BackButton> {}
                        <Label> {
                            text: "Bar Chart Variations"
                            draw_text: {
                                color: #333333,
                                text_style: <FONT_MANROPE> { font_size: 24.0 }
                            }
                        }
                    }

                    <View> {
                        width: Fill, height: Fit, flow: Right, spacing: 0,

                        <DetailChartCard> {
                            detail_bar_basic = <BarChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Basic Vertical" } } }
                        }
                        <DetailChartCard> {
                            detail_bar_horizontal = <HorizontalBarChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Horizontal" } } }
                        }
                        <DetailChartCard> {
                            detail_bar_stacked = <BarChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Stacked" } } }
                        }
                    }

                    <View> {
                        width: Fill, height: Fit, flow: Right, spacing: 0,

                        <DetailChartCard> {
                            detail_bar_grouped = <BarChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Grouped" } } }
                        }
                        <DetailChartCard> {
                            detail_bar_floating = <BarChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Floating Bars" } } }
                        }
                        <DetailChartCard> {
                            detail_bar_negative = <BarChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Negative Values" } } }
                        }
                    }

                    <View> {
                        width: Fill, height: Fit, flow: Right, spacing: 0,

                        <DetailChartCard> {
                            detail_bar_gradient = <BarChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Gradient Bars" } } }
                        }
                        <DetailChartCard> {
                            detail_bar_gradient_grouped = <BarChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Gradient Grouped" } } }
                        }
                        <DetailChartCard> {
                            detail_bar_gradient_stacked = <BarChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Gradient Stacked" } } }
                        }
                    }
                }

                // Detail Page - Pie Chart Variations
                pie_detail_page = <ScrollXYView> {
                    visible: false,
                    flow: Down,
                    spacing: 0,
                    padding: 20,

                    <View> {
                        width: Fill,
                        height: Fit,
                        flow: Right,
                        spacing: 20,
                        margin: {bottom: 20},
                        align: {y: 0.5},

                        back_button_pie = <BackButton> {}
                        <Label> {
                            text: "Pie Chart Variations"
                            draw_text: {
                                color: #333333,
                                text_style: <FONT_MANROPE> { font_size: 24.0 }
                            }
                        }
                    }

                    <View> {
                        width: Fill, height: Fit, flow: Right, spacing: 0,

                        <DetailChartCard> {
                            detail_pie_basic = <PieChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Basic Pie" } } }
                        }
                        <DetailChartCard> {
                            detail_pie_doughnut = <PieChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Doughnut (50%)" } } }
                        }
                        <DetailChartCard> {
                            detail_pie_doughnut_small = <PieChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Doughnut (25%)" } } }
                        }
                    }

                    <View> {
                        width: Fill, height: Fit, flow: Right, spacing: 0,

                        <DetailChartCard> {
                            detail_pie_doughnut_large = <PieChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Doughnut (75%)" } } }
                        }
                        <DetailChartCard> {
                            detail_pie_unequal = <PieChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Unequal Distribution" } } }
                        }
                        <DetailChartCard> {
                            detail_pie_many = <PieChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Many Segments (7)" } } }
                        }
                    }

                    <View> {
                        width: Fill, height: Fit, flow: Right, spacing: 0,

                        <DetailChartCard> {
                            detail_pie_gradient_radial = <PieChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Radial Gradient" } } }
                        }
                        <DetailChartCard> {
                            detail_pie_gradient_angular = <PieChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Angular Gradient" } } }
                        }
                        <DetailChartCard> {
                            detail_pie_gradient_doughnut = <PieChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Gradient Doughnut" } } }
                        }
                    }
                }

                // Detail Page - Scatter Chart
                scatter_detail_page = <ScrollXYView> {
                    visible: false,
                    flow: Down,
                    spacing: 0,
                    padding: 20,

                    <View> {
                        width: Fill,
                        height: Fit,
                        flow: Right,
                        spacing: 20,
                        margin: {bottom: 20},
                        align: {y: 0.5},

                        back_button_scatter = <BackButton> {}
                        <Label> {
                            text: "Scatter Chart Variations"
                            draw_text: {
                                color: #333333,
                                text_style: <FONT_MANROPE> { font_size: 24.0 }
                            }
                        }
                    }

                    <View> {
                        width: Fill, height: Fit, flow: Right, spacing: 0,

                        <DetailChartCard> {
                            detail_scatter_basic = <ScatterChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Basic Scatter" } } }
                        }
                        <DetailChartCard> {
                            detail_scatter_multi = <ScatterChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Multi-Dataset" } } }
                        }
                        <DetailChartCard> {
                            detail_bubble = <BubbleChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Bubble (x,y,r)" } } }
                        }
                    }

                    <View> {
                        width: Fill, height: Fit, flow: Right, spacing: 0,

                        <DetailChartCard> {
                            detail_scatter_gradient = <ScatterChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Gradient Points" } } }
                        }
                        <DetailChartCard> {
                            detail_scatter_gradient_multi = <ScatterChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Gradient Multi-Dataset" } } }
                        }
                        <View> { width: Fill, height: Fill }
                    }

                    <View> {
                        width: Fill, height: Fit, flow: Right, spacing: 0,

                        <DetailChartCard> {
                            height: 500,
                            // Overlay scatter and bubble charts
                            detail_bubble_diagonal = <BubbleChart> { width: Fill, height: Fill }
                            detail_scatter_dense = <ScatterChart> {
                                width: Fill, height: Fill,
                                abs_pos: vec2(0.0, 0.0)
                            }
                            <ChartTitle> { label = { label = { text: "Dense Diagonal Scatter + Bubbles" } } }
                        }
                    }
                }

                // Detail Page - Radar Chart
                radar_detail_page = <ScrollXYView> {
                    visible: false,
                    flow: Down,
                    spacing: 0,
                    padding: 20,

                    <View> {
                        width: Fill,
                        height: Fit,
                        flow: Right,
                        spacing: 20,
                        margin: {bottom: 20},
                        align: {y: 0.5},

                        back_button_radar = <BackButton> {}
                        <Label> {
                            text: "Radar Chart Variations"
                            draw_text: {
                                color: #333333,
                                text_style: <FONT_MANROPE> { font_size: 24.0 }
                            }
                        }
                    }

                    <View> {
                        width: Fill, height: Fit, flow: Right, spacing: 0,

                        <DetailChartCard> {
                            detail_radar_basic = <RadarChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Basic (No Fill)" } } }
                        }
                        <DetailChartCard> {
                            detail_radar_filled = <RadarChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Filled" } } }
                        }
                        <DetailChartCard> {
                            detail_radar_multi_2 = <RadarChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "2 Datasets" } } }
                        }
                    }

                    <View> {
                        width: Fill, height: Fit, flow: Right, spacing: 0,

                        <DetailChartCard> {
                            detail_radar_multi_3 = <RadarChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "3 Datasets" } } }
                        }
                        <DetailChartCard> {
                            detail_radar_5axis = <RadarChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "5 Axes" } } }
                        }
                        <DetailChartCard> {
                            detail_radar_8axis = <RadarChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "8 Axes" } } }
                        }
                    }

                    <View> {
                        width: Fill, height: Fit, flow: Right, spacing: 0,

                        <DetailChartCard> {
                            detail_radar_gradient = <RadarChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Gradient Fill" } } }
                        }
                        <DetailChartCard> {
                            detail_radar_gradient_multi = <RadarChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Gradient Multi-Dataset" } } }
                        }
                        <View> { width: Fill, height: Fill }
                    }
                }

                // Detail Page - Polar Area
                polar_detail_page = <ScrollXYView> {
                    visible: false,
                    flow: Down,
                    spacing: 0,
                    padding: 20,

                    <View> {
                        width: Fill,
                        height: Fit,
                        flow: Right,
                        spacing: 20,
                        margin: {bottom: 20},
                        align: {y: 0.5},

                        back_button_polar = <BackButton> {}
                        <Label> {
                            text: "Polar Area Variations"
                            draw_text: {
                                color: #333333,
                                text_style: <FONT_MANROPE> { font_size: 24.0 }
                            }
                        }
                    }

                    <View> {
                        width: Fill, height: Fit, flow: Right, spacing: 0,

                        <DetailChartCard> {
                            detail_polar_area_basic = <PolarAreaChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Basic" } } }
                        }
                        <DetailChartCard> {
                            detail_polar_area_4 = <PolarAreaChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "4 Segments" } } }
                        }
                        <DetailChartCard> {
                            detail_polar_area_8 = <PolarAreaChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "8 Segments" } } }
                        }
                    }

                    <View> {
                        width: Fill, height: Fit, flow: Right, spacing: 0,

                        <DetailChartCard> {
                            detail_polar_gradient = <PolarAreaChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Gradient Segments" } } }
                        }
                        <DetailChartCard> {
                            detail_polar_gradient_4 = <PolarAreaChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Gradient 4 Segments" } } }
                        }
                        <View> { width: Fill, height: Fill }
                    }
                }

                // Detail Page - Bubble Chart
                bubble_detail_page = <ScrollXYView> {
                    visible: false,
                    flow: Down,
                    spacing: 0,
                    padding: 20,

                    <View> {
                        width: Fill,
                        height: Fit,
                        flow: Right,
                        spacing: 20,
                        margin: {bottom: 20},
                        align: {y: 0.5},

                        back_button_bubble = <BackButton> {}
                        <Label> {
                            text: "Bubble Chart Variations"
                            draw_text: {
                                color: #333333,
                                text_style: <FONT_MANROPE> { font_size: 24.0 }
                            }
                        }
                    }

                    <View> {
                        width: Fill, height: Fit, flow: Right, spacing: 0,

                        <DetailChartCard> {
                            detail_bubble_basic = <BubbleChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Basic Bubble" } } }
                        }
                        <DetailChartCard> {
                            detail_bubble_multi = <BubbleChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Multi-Dataset" } } }
                        }
                        <DetailChartCard> {
                            detail_bubble_large = <BubbleChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Large Bubbles" } } }
                        }
                    }

                    <View> {
                        width: Fill, height: Fit, flow: Right, spacing: 0,

                        <DetailChartCard> {
                            detail_bubble_gradient = <BubbleChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Gradient Bubbles" } } }
                        }
                        <DetailChartCard> {
                            detail_bubble_gradient_multi = <BubbleChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Gradient Multi-Dataset" } } }
                        }
                        <View> { width: Fill, height: Fill }
                    }
                }

                // Detail Page - Combo Chart
                combo_detail_page = <ScrollXYView> {
                    visible: false,
                    flow: Down,
                    spacing: 0,
                    padding: 20,

                    <View> {
                        width: Fill,
                        height: Fit,
                        flow: Right,
                        spacing: 20,
                        margin: {bottom: 20},
                        align: {y: 0.5},

                        back_button_combo = <BackButton> {}
                        <Label> {
                            text: "Combo Chart Variations"
                            draw_text: {
                                color: #333333,
                                text_style: <FONT_MANROPE> { font_size: 24.0 }
                            }
                        }
                    }

                    <View> {
                        width: Fill, height: Fit, flow: Right, spacing: 0,

                        <DetailChartCard> {
                            detail_combo_bar_line = <ComboChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Bar + Line" } } }
                        }
                        <DetailChartCard> {
                            detail_combo_line_bar = <ComboChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Line + Bar" } } }
                        }
                        <DetailChartCard> {
                            detail_combo_stacked = <ComboChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Stacked Bar + Line" } } }
                        }
                    }

                    <View> {
                        width: Fill, height: Fit, flow: Right, spacing: 0,

                        <DetailChartCard> {
                            detail_combo_gradient = <ComboChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Gradient Bars + Line" } } }
                        }
                        <View> { width: Fill, height: Fill }
                        <View> { width: Fill, height: Fill }
                    }
                }

                // Detail Page - Chord Diagram
                chord_detail_page = <ScrollXYView> {
                    visible: false,
                    flow: Down,
                    spacing: 0,
                    padding: 20,

                    <View> {
                        width: Fill,
                        height: Fit,
                        flow: Right,
                        spacing: 20,
                        margin: {bottom: 20},
                        align: {y: 0.5},

                        back_button_chord = <BackButton> {}
                        <Label> {
                            text: "Chord Diagram Variations"
                            draw_text: {
                                color: #333333,
                                text_style: <FONT_MANROPE> { font_size: 24.0 }
                            }
                        }
                    }

                    <View> {
                        width: Fill, height: Fit, flow: Right, spacing: 0,

                        <DetailChartCard> {
                            detail_chord_basic = <ChordChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Basic Chord" } } }
                        }
                        <DetailChartCard> {
                            detail_chord_gradient = <ChordChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Gradient Ribbons" } } }
                        }
                        <DetailChartCard> {
                            detail_chord_directed = <ChordChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Directed (Arrows)" } } }
                        }
                    }

                    <View> {
                        width: Fill, height: Fit, flow: Right, spacing: 0,

                        <DetailChartCard> {
                            detail_chord_arc_gradient = <ChordChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Arc Gradient" } } }
                        }
                        <DetailChartCard> {
                            detail_chord_directed_gradient = <ChordChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Directed + Gradient" } } }
                        }
                        <DetailChartCard> {
                            detail_chord_thin_arcs = <ChordChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Thin Arcs" } } }
                        }
                    }

                    <View> {
                        width: Fill, height: Fit, flow: Right, spacing: 0,

                        <DetailChartCard> {
                            detail_chord_wide_gap = <ChordChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Wide Gap" } } }
                        }
                        <DetailChartCard> {
                            detail_chord_bounce = <ChordChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Bounce Animation" } } }
                        }
                        <DetailChartCard> {
                            detail_chord_elastic = <ChordChart> { width: Fill, height: Fill }
                            <ChartTitle> { label = { label = { text: "Elastic Animation" } } }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum CurrentPage {
    #[default]
    Main,
    LineDetail,
    BarDetail,
    PieDetail,
    ScatterDetail,
    RadarDetail,
    PolarDetail,
    BubbleDetail,
    ComboDetail,
    ChordDetail,
}

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,

    #[rust]
    current_page: CurrentPage,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        makepad_charts::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        // Resize window to fit screen nicely based on measured screen size
        let screen = cx.display_context.screen_size;

        if screen.x > 0.0 && screen.y > 0.0 {
            // Calculate 85% of available screen space, with reasonable minimums and maximums
            let target_width = (screen.x * 0.85).max(1200.0).min(1920.0);
            let target_height = (screen.y * 0.85).max(800.0).min(1200.0);

            // Resize the window to fit the screen
            self.ui.window(id!(ui)).resize(cx, dvec2(target_width, target_height));
        }

        self.setup_main_charts(cx);
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // Handle chart card hover animations on main page
        // Only trigger animation if not already animating
        if self.ui.view(id!(bar_card)).finger_hover_in(actions).is_some() {
            if let Some(mut chart) = self.ui.widget(id!(bar_chart)).borrow_mut::<BarChart>() {
                if !chart.is_animating() {
                    chart.replay_animation(cx);
                }
            }
        }
        if self.ui.view(id!(line_card)).finger_hover_in(actions).is_some() {
            if let Some(mut chart) = self.ui.widget(id!(line_chart)).borrow_mut::<LineChart>() {
                if !chart.is_animating() {
                    chart.replay_animation(cx);
                }
            }
        }
        if self.ui.view(id!(pie_card)).finger_hover_in(actions).is_some() {
            if let Some(mut chart) = self.ui.widget(id!(pie_chart)).borrow_mut::<PieChart>() {
                if !chart.is_animating() {
                    chart.replay_animation(cx);
                }
            }
        }
        if self.ui.view(id!(scatter_card)).finger_hover_in(actions).is_some() {
            if let Some(mut chart) = self.ui.widget(id!(scatter_chart)).borrow_mut::<ScatterChart>() {
                if !chart.is_animating() {
                    chart.replay_animation(cx);
                }
            }
        }
        if self.ui.view(id!(radar_card)).finger_hover_in(actions).is_some() {
            if let Some(mut chart) = self.ui.widget(id!(radar_chart)).borrow_mut::<RadarChart>() {
                if !chart.is_animating() {
                    chart.replay_animation(cx);
                }
            }
        }
        if self.ui.view(id!(polar_card)).finger_hover_in(actions).is_some() {
            if let Some(mut chart) = self.ui.widget(id!(polar_chart)).borrow_mut::<PolarAreaChart>() {
                if !chart.is_animating() {
                    chart.replay_animation(cx);
                }
            }
        }
        if self.ui.view(id!(bubble_card)).finger_hover_in(actions).is_some() {
            if let Some(mut chart) = self.ui.widget(id!(bubble_chart)).borrow_mut::<BubbleChart>() {
                if !chart.is_animating() {
                    chart.replay_animation(cx);
                }
            }
        }
        if self.ui.view(id!(combo_card)).finger_hover_in(actions).is_some() {
            if let Some(mut chart) = self.ui.widget(id!(combo_chart)).borrow_mut::<ComboChart>() {
                if !chart.is_animating() {
                    chart.replay_animation(cx);
                }
            }
        }
        if self.ui.view(id!(chord_card)).finger_hover_in(actions).is_some() {
            if let Some(mut chart) = self.ui.widget(id!(chord_chart)).borrow_mut::<ChordChart>() {
                if !chart.is_animating() {
                    chart.replay_animation(cx);
                }
            }
        }
        // Handle chart card clicks
        if self.ui.view(id!(line_card)).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::LineDetail);
        }
        if self.ui.view(id!(bar_card)).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::BarDetail);
        }
        if self.ui.view(id!(pie_card)).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::PieDetail);
        }
        if self.ui.view(id!(scatter_card)).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::ScatterDetail);
        }
        if self.ui.view(id!(radar_card)).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::RadarDetail);
        }
        if self.ui.view(id!(polar_card)).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::PolarDetail);
        }
        if self.ui.view(id!(bubble_card)).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::BubbleDetail);
        }
        if self.ui.view(id!(combo_card)).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::ComboDetail);
        }
        if self.ui.view(id!(chord_card)).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::ChordDetail);
        }

        // Handle back buttons
        if self.ui.button(id!(back_button_line)).clicked(actions) {
            self.navigate_to(cx, CurrentPage::Main);
        }
        if self.ui.button(id!(back_button_bar)).clicked(actions) {
            self.navigate_to(cx, CurrentPage::Main);
        }
        if self.ui.button(id!(back_button_pie)).clicked(actions) {
            self.navigate_to(cx, CurrentPage::Main);
        }
        if self.ui.button(id!(back_button_scatter)).clicked(actions) {
            self.navigate_to(cx, CurrentPage::Main);
        }
        if self.ui.button(id!(back_button_radar)).clicked(actions) {
            self.navigate_to(cx, CurrentPage::Main);
        }
        if self.ui.button(id!(back_button_polar)).clicked(actions) {
            self.navigate_to(cx, CurrentPage::Main);
        }
        if self.ui.button(id!(back_button_bubble)).clicked(actions) {
            self.navigate_to(cx, CurrentPage::Main);
        }
        if self.ui.button(id!(back_button_combo)).clicked(actions) {
            self.navigate_to(cx, CurrentPage::Main);
        }
        if self.ui.button(id!(back_button_chord)).clicked(actions) {
            self.navigate_to(cx, CurrentPage::Main);
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
    fn navigate_to(&mut self, cx: &mut Cx, page: CurrentPage) {
        self.current_page = page;

        // Hide all pages
        self.ui.view(id!(main_page)).set_visible(cx, false);
        self.ui.view(id!(line_detail_page)).set_visible(cx, false);
        self.ui.view(id!(bar_detail_page)).set_visible(cx, false);
        self.ui.view(id!(pie_detail_page)).set_visible(cx, false);
        self.ui.view(id!(scatter_detail_page)).set_visible(cx, false);
        self.ui.view(id!(radar_detail_page)).set_visible(cx, false);
        self.ui.view(id!(polar_detail_page)).set_visible(cx, false);
        self.ui.view(id!(bubble_detail_page)).set_visible(cx, false);
        self.ui.view(id!(combo_detail_page)).set_visible(cx, false);
        self.ui.view(id!(chord_detail_page)).set_visible(cx, false);

        // Show selected page and setup its charts
        match page {
            CurrentPage::Main => {
                self.ui.view(id!(main_page)).set_visible(cx, true);
            }
            CurrentPage::LineDetail => {
                self.ui.view(id!(line_detail_page)).set_visible(cx, true);
                self.setup_line_detail_charts(cx);
            }
            CurrentPage::BarDetail => {
                self.ui.view(id!(bar_detail_page)).set_visible(cx, true);
                self.setup_bar_detail_charts(cx);
            }
            CurrentPage::PieDetail => {
                self.ui.view(id!(pie_detail_page)).set_visible(cx, true);
                self.setup_pie_detail_charts(cx);
            }
            CurrentPage::ScatterDetail => {
                self.ui.view(id!(scatter_detail_page)).set_visible(cx, true);
                self.setup_scatter_detail_charts(cx);
            }
            CurrentPage::RadarDetail => {
                self.ui.view(id!(radar_detail_page)).set_visible(cx, true);
                self.setup_radar_detail_charts(cx);
            }
            CurrentPage::PolarDetail => {
                self.ui.view(id!(polar_detail_page)).set_visible(cx, true);
                self.setup_polar_detail_charts(cx);
            }
            CurrentPage::BubbleDetail => {
                self.ui.view(id!(bubble_detail_page)).set_visible(cx, true);
                self.setup_bubble_detail_charts(cx);
            }
            CurrentPage::ComboDetail => {
                self.ui.view(id!(combo_detail_page)).set_visible(cx, true);
                self.setup_combo_detail_charts(cx);
            }
            CurrentPage::ChordDetail => {
                self.ui.view(id!(chord_detail_page)).set_visible(cx, true);
                self.setup_chord_detail_charts(cx);
            }
        }

        // Force redraw to trigger layout recalculation
        self.ui.redraw(cx);
        // Schedule next frame to ensure charts recalculate their sizes
        cx.new_next_frame();
    }

    fn setup_main_charts(&mut self, cx: &mut Cx) {
        // Basic sample data for main page preview
        let sample_data = ChartData::new()
            .with_labels(vec!["Jan", "Feb", "Mar", "Apr", "May", "Jun"])
            .add_dataset(Dataset::new("Data").with_data(vec![65.0, 59.0, 80.0, 81.0, 56.0, 72.0]));

        // Bar Chart
        if let Some(mut chart) = self.ui.widget(id!(bar_chart)).borrow_mut::<BarChart>() {
            chart.set_data(sample_data.clone());
            chart.set_options(ChartOptions::new().with_begin_at_zero(true));
        }

        // Line Chart
        if let Some(mut chart) = self.ui.widget(id!(line_chart)).borrow_mut::<LineChart>() {
            chart.set_data(sample_data.clone());
            chart.set_options(ChartOptions::new().with_begin_at_zero(true));
        }

        // Pie Chart
        let pie_data = ChartData::new()
            .with_labels(vec!["A", "B", "C", "D", "E"])
            .add_dataset(Dataset::new("Data").with_data(vec![30.0, 25.0, 20.0, 15.0, 10.0]));
        if let Some(mut chart) = self.ui.widget(id!(pie_chart)).borrow_mut::<PieChart>() {
            chart.set_data(pie_data);
        }

        // Scatter Chart
        let scatter_data = ChartData::new()
            .add_dataset(Dataset::new("Points").with_xy_data(vec![
                (10.0, 20.0), (20.0, 30.0), (30.0, 25.0), (40.0, 50.0), (50.0, 40.0)
            ]));
        if let Some(mut chart) = self.ui.widget(id!(scatter_chart)).borrow_mut::<ScatterChart>() {
            chart.set_data(scatter_data);
        }

        // Radar Chart
        let radar_data = ChartData::new()
            .with_labels(vec!["A", "B", "C", "D", "E", "F"])
            .add_dataset(Dataset::new("Data").with_data(vec![80.0, 60.0, 90.0, 70.0, 85.0, 75.0]));
        if let Some(mut chart) = self.ui.widget(id!(radar_chart)).borrow_mut::<RadarChart>() {
            chart.set_data(radar_data);
            chart.set_fill(true);
        }

        // Polar Area Chart
        let polar_data = ChartData::new()
            .with_labels(vec!["Q1", "Q2", "Q3", "Q4"])
            .add_dataset(Dataset::new("Data").with_data(vec![120.0, 190.0, 150.0, 220.0]));
        if let Some(mut chart) = self.ui.widget(id!(polar_chart)).borrow_mut::<PolarAreaChart>() {
            chart.set_data(polar_data);
        }

        // Bubble Chart
        let bubble_data = ChartData::new()
            .add_dataset(Dataset::new("Bubbles").with_bubble_data(vec![
                (10.0, 20.0, 15.0), (20.0, 35.0, 20.0), (35.0, 25.0, 12.0), (45.0, 40.0, 18.0)
            ]));
        if let Some(mut chart) = self.ui.widget(id!(bubble_chart)).borrow_mut::<BubbleChart>() {
            chart.set_data(bubble_data);
        }

        // Combo Chart
        let combo_data = ChartData::new()
            .with_labels(vec!["Jan", "Feb", "Mar", "Apr", "May", "Jun"])
            .add_dataset(Dataset::new("Bars").with_data(vec![65.0, 59.0, 80.0, 81.0, 56.0, 72.0]))
            .add_dataset(Dataset::new("Line").with_data(vec![60.0, 62.0, 70.0, 75.0, 65.0, 70.0]));
        if let Some(mut chart) = self.ui.widget(id!(combo_chart)).borrow_mut::<ComboChart>() {
            chart.set_data(combo_data);
            chart.set_dataset_types(vec![DatasetType::Bar, DatasetType::Line]);
            chart.set_options(ChartOptions::new().with_begin_at_zero(true));
        }

        // Chord Diagram - relationship between 5 groups
        let chord_data = ChordData::new()
            .with_labels(vec!["Sales", "Marketing", "Engineering", "Support", "Finance"])
            .with_matrix(vec![
                vec![0.0, 80.0, 30.0, 50.0, 20.0],    // Sales flows to others
                vec![40.0, 0.0, 60.0, 30.0, 10.0],    // Marketing flows to others
                vec![20.0, 50.0, 0.0, 70.0, 40.0],    // Engineering flows to others
                vec![60.0, 20.0, 40.0, 0.0, 30.0],    // Support flows to others
                vec![30.0, 15.0, 25.0, 20.0, 0.0],    // Finance flows to others
            ]);
        if let Some(mut chart) = self.ui.widget(id!(chord_chart)).borrow_mut::<ChordChart>() {
            chart.set_data(chord_data);
            chart.set_options(ChartOptions::new().with_animation_duration(1000.0));
        }

        // Horizontal Bar Chart
        let horizontal_data = ChartData::new()
            .with_labels(vec!["Jan", "Feb", "Mar", "Apr", "May", "Jun"])
            .add_dataset(Dataset::new("Sales").with_data(vec![65.0, 59.0, 80.0, 81.0, 56.0, 72.0]));
        if let Some(mut chart) = self.ui.widget(id!(horizontal_bar_chart)).borrow_mut::<HorizontalBarChart>() {
            chart.set_data(horizontal_data);
            chart.set_options(ChartOptions::new().with_begin_at_zero(true));
        }

        self.ui.redraw(cx);
    }

    fn setup_line_detail_charts(&mut self, cx: &mut Cx) {
        let data = ChartData::new()
            .with_labels(vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11"])
            .add_dataset(Dataset::new("Data")
                .with_data(vec![0.0, 20.0, 20.0, 60.0, 60.0, 120.0, 80.0, 120.0, 140.0, 100.0, 70.0, 50.0]));

        // Basic Line
        if let Some(mut chart) = self.ui.widget(id!(detail_line_basic)).borrow_mut::<LineChart>() {
            chart.set_data(data.clone());
            chart.set_tension(0.0);
            chart.set_options(ChartOptions::new().with_begin_at_zero(true));
            chart.replay_animation(cx); // Trigger animation for proper layout
        }

        // Cubic Interpolation (with EaseOutBounce animation - bouncy effect)
        if let Some(mut chart) = self.ui.widget(id!(detail_line_cubic)).borrow_mut::<LineChart>() {
            chart.set_data(data.clone());
            chart.set_tension(0.4);
            chart.set_cubic_interpolation_mode(CubicInterpolationMode::Default);
            chart.set_options(ChartOptions::new()
                .with_begin_at_zero(true)
                .with_animation_duration(1500.0)
                .with_animation_easing(EasingType::EaseOutBounce));
        }

        // Monotone Cubic
        if let Some(mut chart) = self.ui.widget(id!(detail_line_monotone)).borrow_mut::<LineChart>() {
            chart.set_data(data.clone());
            chart.set_tension(0.4);
            chart.set_cubic_interpolation_mode(CubicInterpolationMode::Monotone);
            chart.set_options(ChartOptions::new().with_begin_at_zero(true));
            chart.replay_animation(cx); // Trigger animation for proper layout
        }

        // Stepped modes
        let stepped_data = ChartData::new()
            .with_labels(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"])
            .add_dataset(Dataset::new("Data").with_data(vec![10.0, 15.0, 12.0, 20.0, 18.0, 25.0, 22.0]));

        if let Some(mut chart) = self.ui.widget(id!(detail_line_stepped_before)).borrow_mut::<LineChart>() {
            chart.set_data(stepped_data.clone());
            chart.set_stepped(SteppedMode::Before);
            chart.set_options(ChartOptions::new().with_begin_at_zero(true));
            chart.replay_animation(cx); // Trigger animation for proper layout
        }

        if let Some(mut chart) = self.ui.widget(id!(detail_line_stepped_after)).borrow_mut::<LineChart>() {
            chart.set_data(stepped_data.clone());
            chart.set_stepped(SteppedMode::After);
            chart.set_options(ChartOptions::new().with_begin_at_zero(true));
            chart.replay_animation(cx); // Trigger animation for proper layout
        }

        if let Some(mut chart) = self.ui.widget(id!(detail_line_stepped_middle)).borrow_mut::<LineChart>() {
            chart.set_data(stepped_data);
            chart.set_stepped(SteppedMode::Middle);
            chart.set_options(ChartOptions::new().with_begin_at_zero(true));
            chart.replay_animation(cx); // Trigger animation for proper layout
        }

        // Area Fill
        if let Some(mut chart) = self.ui.widget(id!(detail_line_area)).borrow_mut::<LineChart>() {
            chart.set_data(data.clone());
            chart.set_fill(true);
            chart.set_options(ChartOptions::new().with_begin_at_zero(true));
            chart.replay_animation(cx); // Trigger animation for proper layout
        }

        // Multi-series
        let multi_data = ChartData::new()
            .with_labels(vec!["Jan", "Feb", "Mar", "Apr", "May", "Jun"])
            .add_dataset(Dataset::new("Series A").with_data(vec![65.0, 59.0, 80.0, 81.0, 56.0, 72.0]))
            .add_dataset(Dataset::new("Series B").with_data(vec![40.0, 48.0, 60.0, 55.0, 45.0, 58.0]))
            .add_dataset(Dataset::new("Series C").with_data(vec![30.0, 35.0, 42.0, 38.0, 32.0, 40.0]));

        if let Some(mut chart) = self.ui.widget(id!(detail_line_multi)).borrow_mut::<LineChart>() {
            chart.set_data(multi_data);
            chart.set_options(ChartOptions::new().with_begin_at_zero(true));
            chart.replay_animation(cx); // Trigger animation for proper layout
        }

        // Smooth Area
        if let Some(mut chart) = self.ui.widget(id!(detail_line_smooth_area)).borrow_mut::<LineChart>() {
            chart.set_data(data);
            chart.set_tension(0.4);
            chart.set_cubic_interpolation_mode(CubicInterpolationMode::Monotone);
            chart.set_fill(true);
            chart.set_options(ChartOptions::new().with_begin_at_zero(true));
            chart.replay_animation(cx); // Trigger animation for proper layout
        }

        // Stock Market - Progressive animation with easing
        // Generate realistic stock price movements with trend + volatility
        let num_points = 1000;

        // Stock 1: MKPD - Bull then correction then recovery
        let mut stock_data: Vec<f64> = Vec::with_capacity(num_points);
        let mut price = 100.0;
        for i in 0..num_points {
            let seed = i as f64 * 0.618033988749;
            let t = i as f64 / num_points as f64;
            let trend = if t < 0.3 { 0.08 } else if t < 0.45 { -0.15 } else if t < 0.7 { 0.12 } else if t < 0.85 { -0.05 } else { 0.1 };
            let volatility = (seed * 17.0).sin() * 0.8 + (seed * 31.0).cos() * 0.5;
            let spike = if (seed * 47.0).sin() > 0.95 { (seed * 23.0).sin() * 3.0 } else { 0.0 };
            price += trend + volatility + spike;
            price = price.max(50.0);
            stock_data.push(price);
        }

        // Stock 2: INDEX - Steady growth with mild corrections
        let mut stock_data3: Vec<f64> = Vec::with_capacity(num_points);
        let mut price3 = 90.0;
        for i in 0..num_points {
            let seed = (i + 600) as f64 * 0.618033988749;
            let t = i as f64 / num_points as f64;
            let trend = if t < 0.35 { 0.05 } else if t < 0.5 { -0.06 } else if t < 0.75 { 0.07 } else { 0.04 };
            let volatility = (seed * 23.0).sin() * 0.5 + (seed * 41.0).cos() * 0.3;
            let spike = if (seed * 59.0).sin() > 0.96 { (seed * 31.0).sin() * 2.0 } else { 0.0 };
            price3 += trend + volatility + spike;
            price3 = price3.max(70.0);
            stock_data3.push(price3);
        }

        let wave_labels: Vec<&str> = (0..num_points).map(|_| "").collect();

        let stock_market_data = ChartData::new()
            .with_labels(wave_labels.clone())
            .add_dataset(Dataset::new("MKPD").with_data(stock_data))
            .add_dataset(Dataset::new("INDEX").with_data(stock_data3).with_color(vec4(0.4, 0.8, 0.5, 1.0)));

        if let Some(mut chart) = self.ui.widget(id!(detail_line_stock_market)).borrow_mut::<LineChart>() {
            chart.set_data(stock_market_data);
            chart.set_tension(0.1);
            chart.set_show_points(false);
            chart.set_progressive_animation(true);
            chart.set_options(ChartOptions::new()
                .with_animation_duration(3000.0)
                .with_animation_easing(EasingType::EaseOutCubic));
            chart.replay_animation(cx);
        }

        // Dense Wave - faster frequency
        let dense_wave_data: Vec<f64> = (0..num_points)
            .map(|i| {
                let x = i as f64 / num_points as f64 * 8.0 * std::f64::consts::PI;
                50.0 + 30.0 * x.sin() + 15.0 * (x * 2.0).sin()
            })
            .collect();

        let dense_data = ChartData::new()
            .with_labels(wave_labels.clone())
            .add_dataset(Dataset::new("Dense").with_data(dense_wave_data));

        if let Some(mut chart) = self.ui.widget(id!(detail_line_radio_wave_2)).borrow_mut::<LineChart>() {
            chart.set_data(dense_data);
            chart.set_tension(0.2);
            chart.set_show_points(false);
            chart.set_progressive_animation(true);
            chart.set_options(ChartOptions::new()
                .with_begin_at_zero(true)
                .with_animation_duration(2500.0));
            chart.replay_animation(cx);
        }

        // Audio Signal - complex waveform
        let audio_data: Vec<f64> = (0..num_points)
            .map(|i| {
                let x = i as f64 / num_points as f64 * 6.0 * std::f64::consts::PI;
                let envelope = (-(i as f64 - 50.0).powi(2) / 800.0).exp();
                50.0 + 40.0 * x.sin() * envelope + 10.0 * (x * 3.0).cos()
            })
            .collect();

        let audio_chart_data = ChartData::new()
            .with_labels(wave_labels)
            .add_dataset(Dataset::new("Audio").with_data(audio_data));

        if let Some(mut chart) = self.ui.widget(id!(detail_line_radio_wave_3)).borrow_mut::<LineChart>() {
            chart.set_data(audio_chart_data);
            chart.set_tension(0.3);
            chart.set_show_points(false);
            chart.set_progressive_animation(true);
            chart.set_options(ChartOptions::new()
                .with_begin_at_zero(true)
                .with_animation_duration(3000.0));
            chart.replay_animation(cx);
        }

        // Gradient line charts
        let gradient_data = ChartData::new()
            .with_labels(vec!["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug"])
            .add_dataset(Dataset::new("Data").with_data(vec![65.0, 59.0, 80.0, 81.0, 56.0, 72.0, 85.0, 78.0]));

        if let Some(mut chart) = self.ui.widget(id!(detail_line_gradient)).borrow_mut::<LineChart>() {
            chart.set_data(gradient_data.clone());
            chart.set_fill(true);
            chart.set_gradient(true);
            chart.set_options(ChartOptions::new().with_begin_at_zero(true));
            chart.replay_animation(cx);
        }

        if let Some(mut chart) = self.ui.widget(id!(detail_line_gradient_smooth)).borrow_mut::<LineChart>() {
            chart.set_data(gradient_data.clone());
            chart.set_fill(true);
            chart.set_gradient(true);
            chart.set_tension(0.4);
            chart.set_cubic_interpolation_mode(CubicInterpolationMode::Monotone);
            chart.set_options(ChartOptions::new().with_begin_at_zero(true));
            chart.replay_animation(cx);
        }

        let gradient_multi_data = ChartData::new()
            .with_labels(vec!["Jan", "Feb", "Mar", "Apr", "May", "Jun"])
            .add_dataset(Dataset::new("Series A").with_data(vec![65.0, 59.0, 80.0, 81.0, 56.0, 72.0]))
            .add_dataset(Dataset::new("Series B").with_data(vec![40.0, 48.0, 60.0, 55.0, 45.0, 58.0]));

        if let Some(mut chart) = self.ui.widget(id!(detail_line_gradient_multi)).borrow_mut::<LineChart>() {
            chart.set_data(gradient_multi_data);
            chart.set_fill(true);
            chart.set_gradient(true);
            chart.set_tension(0.3);
            chart.set_options(ChartOptions::new().with_begin_at_zero(true));
            chart.replay_animation(cx);
        }

        self.ui.redraw(cx);
    }

    fn setup_bar_detail_charts(&mut self, cx: &mut Cx) {
        let data = ChartData::new()
            .with_labels(vec!["Jan", "Feb", "Mar", "Apr", "May", "Jun"])
            .add_dataset(Dataset::new("Data").with_data(vec![65.0, 59.0, 80.0, 81.0, 56.0, 72.0]));

        // Basic (with EaseOutBounce animation - fun bouncy effect)
        if let Some(mut chart) = self.ui.widget(id!(detail_bar_basic)).borrow_mut::<BarChart>() {
            chart.set_data(data.clone());
            chart.set_options(ChartOptions::new()
                .with_begin_at_zero(true)
                .with_animation_duration(1000.0)
                .with_animation_easing(EasingType::EaseOutBounce));
        }

        // Horizontal
        if let Some(mut chart) = self.ui.widget(id!(detail_bar_horizontal)).borrow_mut::<HorizontalBarChart>() {
            chart.set_data(data.clone());
            chart.set_options(ChartOptions::new().with_begin_at_zero(true));
        }

        // Stacked
        let stacked_data = ChartData::new()
            .with_labels(vec!["Q1", "Q2", "Q3", "Q4"])
            .add_dataset(Dataset::new("A").with_data(vec![50.0, 60.0, 70.0, 80.0]))
            .add_dataset(Dataset::new("B").with_data(vec![30.0, 40.0, 35.0, 45.0]))
            .add_dataset(Dataset::new("C").with_data(vec![20.0, 25.0, 30.0, 35.0]));
        if let Some(mut chart) = self.ui.widget(id!(detail_bar_stacked)).borrow_mut::<BarChart>() {
            chart.set_data(stacked_data);
            chart.set_stacked(true);
            chart.set_options(ChartOptions::new().with_begin_at_zero(true));
        }

        // Grouped
        let grouped_data = ChartData::new()
            .with_labels(vec!["Q1", "Q2", "Q3", "Q4"])
            .add_dataset(Dataset::new("2023").with_data(vec![120.0, 150.0, 180.0, 200.0]))
            .add_dataset(Dataset::new("2024").with_data(vec![140.0, 170.0, 210.0, 240.0]));
        if let Some(mut chart) = self.ui.widget(id!(detail_bar_grouped)).borrow_mut::<BarChart>() {
            chart.set_data(grouped_data);
            chart.set_options(ChartOptions::new().with_begin_at_zero(true));
        }

        // Floating
        let floating_data = ChartData::new()
            .with_labels(vec!["Jan", "Feb", "Mar", "Apr", "May", "Jun"])
            .add_dataset(Dataset::new("Range").with_floating_data(vec![
                (-5.0, 8.0), (-3.0, 12.0), (2.0, 18.0), (8.0, 24.0), (14.0, 28.0), (18.0, 32.0)
            ]));
        if let Some(mut chart) = self.ui.widget(id!(detail_bar_floating)).borrow_mut::<BarChart>() {
            chart.set_data(floating_data);
            chart.set_options(ChartOptions::new().with_begin_at_zero(false));
        }

        // Negative
        let negative_data = ChartData::new()
            .with_labels(vec!["Jan", "Feb", "Mar", "Apr", "May", "Jun"])
            .add_dataset(Dataset::new("P/L").with_data(vec![20.0, -15.0, 35.0, -25.0, 40.0, -10.0]));
        if let Some(mut chart) = self.ui.widget(id!(detail_bar_negative)).borrow_mut::<BarChart>() {
            chart.set_data(negative_data);
            chart.set_options(ChartOptions::new().with_begin_at_zero(false));
        }

        // Gradient bars
        if let Some(mut chart) = self.ui.widget(id!(detail_bar_gradient)).borrow_mut::<BarChart>() {
            chart.set_data(data.clone());
            chart.set_gradient(true);
            chart.set_options(ChartOptions::new().with_begin_at_zero(true));
            chart.replay_animation(cx);
        }

        // Gradient grouped
        let gradient_grouped = ChartData::new()
            .with_labels(vec!["Q1", "Q2", "Q3", "Q4"])
            .add_dataset(Dataset::new("2023").with_data(vec![120.0, 150.0, 180.0, 200.0]))
            .add_dataset(Dataset::new("2024").with_data(vec![140.0, 170.0, 210.0, 240.0]));
        if let Some(mut chart) = self.ui.widget(id!(detail_bar_gradient_grouped)).borrow_mut::<BarChart>() {
            chart.set_data(gradient_grouped);
            chart.set_gradient(true);
            chart.set_options(ChartOptions::new().with_begin_at_zero(true));
            chart.replay_animation(cx);
        }

        // Gradient stacked
        let gradient_stacked = ChartData::new()
            .with_labels(vec!["Q1", "Q2", "Q3", "Q4"])
            .add_dataset(Dataset::new("A").with_data(vec![50.0, 60.0, 70.0, 80.0]))
            .add_dataset(Dataset::new("B").with_data(vec![30.0, 40.0, 35.0, 45.0]))
            .add_dataset(Dataset::new("C").with_data(vec![20.0, 25.0, 30.0, 35.0]));
        if let Some(mut chart) = self.ui.widget(id!(detail_bar_gradient_stacked)).borrow_mut::<BarChart>() {
            chart.set_data(gradient_stacked);
            chart.set_stacked(true);
            chart.set_gradient(true);
            chart.set_options(ChartOptions::new().with_begin_at_zero(true));
            chart.replay_animation(cx);
        }

        self.ui.redraw(cx);
    }

    fn setup_pie_detail_charts(&mut self, cx: &mut Cx) {
        // Standard 5-segment pie data (like Chart.js samples)
        let data = ChartData::new()
            .with_labels(vec!["Red", "Orange", "Yellow", "Green", "Blue"])
            .add_dataset(Dataset::new("Dataset 1").with_data(vec![300.0, 50.0, 100.0, 80.0, 120.0]));

        // Basic Pie (with EaseOutBounce animation - bouncy effect)
        if let Some(mut chart) = self.ui.widget(id!(detail_pie_basic)).borrow_mut::<PieChart>() {
            chart.set_data(data.clone());
            chart.set_options(ChartOptions::new()
                .with_animation_duration(1500.0)
                .with_animation_easing(EasingType::EaseOutBounce));
        }

        // Doughnut (50% cutout - standard)
        if let Some(mut chart) = self.ui.widget(id!(detail_pie_doughnut)).borrow_mut::<PieChart>() {
            chart.set_data(data.clone());
            chart.set_doughnut(true);
            chart.set_inner_radius_ratio(0.5);
            chart.replay_animation(cx);
        }

        // Doughnut (25% cutout - small hole)
        if let Some(mut chart) = self.ui.widget(id!(detail_pie_doughnut_small)).borrow_mut::<PieChart>() {
            chart.set_data(data.clone());
            chart.set_doughnut(true);
            chart.set_inner_radius_ratio(0.25);
            chart.replay_animation(cx);
        }

        // Doughnut (75% cutout - large hole)
        if let Some(mut chart) = self.ui.widget(id!(detail_pie_doughnut_large)).borrow_mut::<PieChart>() {
            chart.set_data(data.clone());
            chart.set_doughnut(true);
            chart.set_inner_radius_ratio(0.75);
            chart.replay_animation(cx);
        }

        // Unequal distribution (one dominant segment)
        let unequal_data = ChartData::new()
            .with_labels(vec!["Dominant", "Small A", "Small B", "Small C", "Small D"])
            .add_dataset(Dataset::new("Data").with_data(vec![60.0, 10.0, 10.0, 10.0, 10.0]));
        if let Some(mut chart) = self.ui.widget(id!(detail_pie_unequal)).borrow_mut::<PieChart>() {
            chart.set_data(unequal_data);
            chart.replay_animation(cx);
        }

        // Many segments (7 segments)
        let many_data = ChartData::new()
            .with_labels(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"])
            .add_dataset(Dataset::new("Data").with_data(vec![65.0, 59.0, 80.0, 81.0, 56.0, 55.0, 40.0]));
        if let Some(mut chart) = self.ui.widget(id!(detail_pie_many)).borrow_mut::<PieChart>() {
            chart.set_data(many_data);
            chart.replay_animation(cx);
        }

        // Radial gradient pie
        if let Some(mut chart) = self.ui.widget(id!(detail_pie_gradient_radial)).borrow_mut::<PieChart>() {
            chart.set_data(data.clone());
            chart.set_radial_gradient(true);
            chart.replay_animation(cx);
        }

        // Angular gradient pie
        if let Some(mut chart) = self.ui.widget(id!(detail_pie_gradient_angular)).borrow_mut::<PieChart>() {
            chart.set_data(data.clone());
            chart.set_angular_gradient(true);
            chart.replay_animation(cx);
        }

        // Gradient doughnut
        if let Some(mut chart) = self.ui.widget(id!(detail_pie_gradient_doughnut)).borrow_mut::<PieChart>() {
            chart.set_data(data);
            chart.set_doughnut(true);
            chart.set_inner_radius_ratio(0.5);
            chart.set_radial_gradient(true);
            chart.replay_animation(cx);
        }

        self.ui.redraw(cx);
    }

    fn setup_scatter_detail_charts(&mut self, cx: &mut Cx) {
        // Basic scatter (with EaseOutBounce animation - bouncy effect)
        let scatter_data = ChartData::new()
            .add_dataset(Dataset::new("Points").with_xy_data(vec![
                (10.0, 20.0), (20.0, 35.0), (30.0, 25.0), (40.0, 50.0), (50.0, 40.0),
                (15.0, 30.0), (25.0, 45.0), (35.0, 35.0), (45.0, 55.0), (55.0, 45.0)
            ]));
        if let Some(mut chart) = self.ui.widget(id!(detail_scatter_basic)).borrow_mut::<ScatterChart>() {
            chart.set_data(scatter_data.clone());
            chart.set_options(ChartOptions::new()
                .with_animation_duration(1500.0)
                .with_animation_easing(EasingType::EaseOutBounce));
        }

        // Multi-dataset
        let multi_scatter = ChartData::new()
            .add_dataset(Dataset::new("Series A").with_xy_data(vec![
                (10.0, 20.0), (20.0, 30.0), (30.0, 40.0), (40.0, 50.0)
            ]))
            .add_dataset(Dataset::new("Series B").with_xy_data(vec![
                (15.0, 45.0), (25.0, 35.0), (35.0, 25.0), (45.0, 15.0)
            ]));
        if let Some(mut chart) = self.ui.widget(id!(detail_scatter_multi)).borrow_mut::<ScatterChart>() {
            chart.set_data(multi_scatter.clone());
            chart.replay_animation(cx);
        }

        // Bubble
        let bubble_data = ChartData::new()
            .add_dataset(Dataset::new("Bubbles").with_bubble_data(vec![
                (10.0, 20.0, 15.0), (25.0, 40.0, 25.0), (40.0, 30.0, 20.0), (55.0, 50.0, 30.0)
            ]));
        if let Some(mut chart) = self.ui.widget(id!(detail_bubble)).borrow_mut::<BubbleChart>() {
            chart.set_data(bubble_data);
            chart.replay_animation(cx);
        }

        // Gradient scatter
        if let Some(mut chart) = self.ui.widget(id!(detail_scatter_gradient)).borrow_mut::<ScatterChart>() {
            chart.set_data(scatter_data.clone());
            chart.set_gradient(true);
            chart.replay_animation(cx);
        }

        // Gradient multi-dataset scatter
        if let Some(mut chart) = self.ui.widget(id!(detail_scatter_gradient_multi)).borrow_mut::<ScatterChart>() {
            chart.set_data(multi_scatter.clone());
            chart.set_gradient(true);
            chart.replay_animation(cx);
        }

        // Dense diagonal scatter - 3 datasets spreading from (0,0) at 45 degrees
        // Blue: on diagonal (width = height)
        // Orange: above diagonal (width < height)
        // Pink: below diagonal (width > height)

        let num_scatter_points = 500;

        // Generate points along diagonal with increasing spread
        let mut on_diagonal: Vec<(f64, f64)> = Vec::new();
        let mut above_diagonal: Vec<(f64, f64)> = Vec::new();
        let mut below_diagonal: Vec<(f64, f64)> = Vec::new();

        for i in 0..num_scatter_points {
            // Base position along diagonal
            let t = (i as f64 / num_scatter_points as f64) * 5.0;
            let spread = t * 0.4; // Spread increases with distance

            // Random offset using simple pseudo-random
            let seed = i as f64 * 0.618033988749;
            let r1 = (seed * 17.0).sin() * 0.5 + 0.5;
            let r2 = (seed * 31.0).cos() * 0.5 + 0.5;
            let r3 = (seed * 47.0).sin() * 0.5 + 0.5;

            let offset_x = (r1 - 0.5) * spread * 2.0;
            let offset_y = (r2 - 0.5) * spread * 2.0;

            let x = (t + offset_x).min(4.8);
            let y = (t + offset_y).min(4.8);

            if x > 0.0 && y > 0.0 && x < 5.0 && y < 5.0 {
                // Classify based on position relative to diagonal
                let diff = y - x;
                if diff.abs() < 0.1 && r3 < 0.15 {
                    on_diagonal.push((x, y));
                } else if diff > 0.0 {
                    above_diagonal.push((x, y));
                } else {
                    below_diagonal.push((x, y));
                }
            }
        }

        let dense_scatter_data = ChartData::new()
            .add_dataset(Dataset::new("width = height")
                .with_xy_data(on_diagonal.clone())
                .with_color(vec4(0.35, 0.65, 0.85, 1.0))) // Blue
            .add_dataset(Dataset::new("width < height")
                .with_xy_data(above_diagonal.clone())
                .with_color(vec4(0.95, 0.7, 0.4, 1.0))) // Orange
            .add_dataset(Dataset::new("width > height")
                .with_xy_data(below_diagonal.clone())
                .with_color(vec4(0.9, 0.5, 0.6, 1.0))); // Pink

        if let Some(mut chart) = self.ui.widget(id!(detail_scatter_dense)).borrow_mut::<ScatterChart>() {
            chart.set_data(dense_scatter_data);
            chart.set_point_radius(3.0);
            chart.set_options(ChartOptions::new()
                .with_begin_at_zero(true)
                .with_animation_duration(2000.0));
            chart.replay_animation(cx);
        }

        // Diagonal bubbles - more random positions and varied sizes along diagonal
        // Colors range from red to purple
        let num_bubbles = 25;
        let mut red_bubbles: Vec<(f64, f64, f64)> = Vec::new();
        let mut magenta_bubbles: Vec<(f64, f64, f64)> = Vec::new();
        let mut pink_bubbles: Vec<(f64, f64, f64)> = Vec::new();
        let mut violet_bubbles: Vec<(f64, f64, f64)> = Vec::new();
        let mut purple_bubbles: Vec<(f64, f64, f64)> = Vec::new();

        for i in 0..num_bubbles {
            let seed = i as f64 * 0.618033988749;
            let t = i as f64 / num_bubbles as f64 * 4.0;

            // Add randomness to position (slight offset from diagonal)
            let offset_x = (seed * 17.0).sin() * 0.15;
            let offset_y = (seed * 23.0).cos() * 0.15;
            let x = t + offset_x;
            let y = t + offset_y;

            // Varied radius: generally larger at origin but with randomness
            let base_radius = 45.0 - t * 8.0;
            let random_factor = 0.5 + (seed * 31.0).sin().abs() * 1.0; // 0.5 to 1.5
            let radius = (base_radius * random_factor).max(8.0);

            if radius > 5.0 && x > 0.0 && y > 0.0 {
                // Distribute to different color groups based on position
                let color_idx = i % 5;
                match color_idx {
                    0 => red_bubbles.push((x, y, radius)),
                    1 => magenta_bubbles.push((x, y, radius)),
                    2 => pink_bubbles.push((x, y, radius)),
                    3 => violet_bubbles.push((x, y, radius)),
                    _ => purple_bubbles.push((x, y, radius)),
                }
            }
        }

        let bubble_diagonal_data = ChartData::new()
            .add_dataset(Dataset::new("Red")
                .with_bubble_data(red_bubbles)
                .with_color(vec4(0.95, 0.25, 0.25, 0.6))) // Red
            .add_dataset(Dataset::new("Magenta")
                .with_bubble_data(magenta_bubbles)
                .with_color(vec4(0.95, 0.25, 0.45, 0.6))) // Magenta-red
            .add_dataset(Dataset::new("Pink")
                .with_bubble_data(pink_bubbles)
                .with_color(vec4(0.9, 0.3, 0.65, 0.6))) // Pink
            .add_dataset(Dataset::new("Violet")
                .with_bubble_data(violet_bubbles)
                .with_color(vec4(0.75, 0.3, 0.8, 0.6))) // Violet
            .add_dataset(Dataset::new("Purple")
                .with_bubble_data(purple_bubbles)
                .with_color(vec4(0.6, 0.25, 0.9, 0.6))); // Purple

        if let Some(mut chart) = self.ui.widget(id!(detail_bubble_diagonal)).borrow_mut::<BubbleChart>() {
            chart.set_data(bubble_diagonal_data);
            chart.set_gradient(true);
            chart.set_options(ChartOptions::new()
                .with_begin_at_zero(true)
                .with_animation_duration(1500.0));
            chart.replay_animation(cx);
        }

        self.ui.redraw(cx);
    }

    fn setup_radar_detail_charts(&mut self, cx: &mut Cx) {
        // 6-axis data (like Chart.js default)
        let data_6 = ChartData::new()
            .with_labels(vec!["Eating", "Drinking", "Sleeping", "Designing", "Coding", "Cycling"])
            .add_dataset(Dataset::new("Dataset 1").with_data(vec![65.0, 59.0, 90.0, 81.0, 56.0, 55.0]));

        // Basic (no fill, with EaseOutBounce animation - bouncy effect)
        if let Some(mut chart) = self.ui.widget(id!(detail_radar_basic)).borrow_mut::<RadarChart>() {
            chart.set_data(data_6.clone());
            chart.set_fill(false);
            chart.set_options(ChartOptions::new()
                .with_animation_duration(1500.0)
                .with_animation_easing(EasingType::EaseOutBounce));
        }

        // Filled
        if let Some(mut chart) = self.ui.widget(id!(detail_radar_filled)).borrow_mut::<RadarChart>() {
            chart.set_data(data_6.clone());
            chart.set_fill(true);
            chart.replay_animation(cx);
        }

        // 2 Datasets (like Chart.js sample)
        let multi_2 = ChartData::new()
            .with_labels(vec!["Eating", "Drinking", "Sleeping", "Designing", "Coding", "Cycling"])
            .add_dataset(Dataset::new("Dataset 1").with_data(vec![65.0, 59.0, 90.0, 81.0, 56.0, 55.0]))
            .add_dataset(Dataset::new("Dataset 2").with_data(vec![28.0, 48.0, 40.0, 19.0, 96.0, 27.0]));
        if let Some(mut chart) = self.ui.widget(id!(detail_radar_multi_2)).borrow_mut::<RadarChart>() {
            chart.set_data(multi_2);
            chart.set_fill(true);
            chart.replay_animation(cx);
        }

        // 3 Datasets
        let multi_3 = ChartData::new()
            .with_labels(vec!["Eating", "Drinking", "Sleeping", "Designing", "Coding", "Cycling"])
            .add_dataset(Dataset::new("Person A").with_data(vec![65.0, 59.0, 90.0, 81.0, 56.0, 55.0]))
            .add_dataset(Dataset::new("Person B").with_data(vec![28.0, 48.0, 40.0, 19.0, 96.0, 27.0]))
            .add_dataset(Dataset::new("Person C").with_data(vec![50.0, 70.0, 60.0, 45.0, 75.0, 80.0]));
        if let Some(mut chart) = self.ui.widget(id!(detail_radar_multi_3)).borrow_mut::<RadarChart>() {
            chart.set_data(multi_3);
            chart.set_fill(true);
            chart.replay_animation(cx);
        }

        // 5 Axes
        let data_5 = ChartData::new()
            .with_labels(vec!["Speed", "Strength", "Defense", "Magic", "Luck"])
            .add_dataset(Dataset::new("Character").with_data(vec![80.0, 60.0, 90.0, 70.0, 85.0]));
        if let Some(mut chart) = self.ui.widget(id!(detail_radar_5axis)).borrow_mut::<RadarChart>() {
            chart.set_data(data_5);
            chart.set_fill(true);
            chart.replay_animation(cx);
        }

        // 8 Axes
        let data_8 = ChartData::new()
            .with_labels(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun", "Holiday"])
            .add_dataset(Dataset::new("Activity").with_data(vec![65.0, 59.0, 80.0, 81.0, 56.0, 55.0, 40.0, 90.0]));
        if let Some(mut chart) = self.ui.widget(id!(detail_radar_8axis)).borrow_mut::<RadarChart>() {
            chart.set_data(data_8);
            chart.set_fill(true);
            chart.replay_animation(cx);
        }

        // Gradient radar
        if let Some(mut chart) = self.ui.widget(id!(detail_radar_gradient)).borrow_mut::<RadarChart>() {
            chart.set_data(data_6.clone());
            chart.set_fill(true);
            chart.set_gradient(true);
            chart.replay_animation(cx);
        }

        // Gradient multi-dataset radar
        let gradient_multi_radar = ChartData::new()
            .with_labels(vec!["Eating", "Drinking", "Sleeping", "Designing", "Coding", "Cycling"])
            .add_dataset(Dataset::new("Dataset 1").with_data(vec![65.0, 59.0, 90.0, 81.0, 56.0, 55.0]))
            .add_dataset(Dataset::new("Dataset 2").with_data(vec![28.0, 48.0, 40.0, 19.0, 96.0, 27.0]));
        if let Some(mut chart) = self.ui.widget(id!(detail_radar_gradient_multi)).borrow_mut::<RadarChart>() {
            chart.set_data(gradient_multi_radar);
            chart.set_fill(true);
            chart.set_gradient(true);
            chart.replay_animation(cx);
        }

        self.ui.redraw(cx);
    }

    fn setup_polar_detail_charts(&mut self, cx: &mut Cx) {
        // Basic 6 segments (with EaseOutBounce animation - bouncy effect)
        let data_6 = ChartData::new()
            .with_labels(vec!["A", "B", "C", "D", "E", "F"])
            .add_dataset(Dataset::new("Data").with_data(vec![120.0, 190.0, 150.0, 220.0, 180.0, 160.0]));
        if let Some(mut chart) = self.ui.widget(id!(detail_polar_area_basic)).borrow_mut::<PolarAreaChart>() {
            chart.set_data(data_6.clone());
            chart.set_options(ChartOptions::new()
                .with_animation_duration(1500.0)
                .with_animation_easing(EasingType::EaseOutBounce));
        }

        // 4 segments
        let data_4 = ChartData::new()
            .with_labels(vec!["Q1", "Q2", "Q3", "Q4"])
            .add_dataset(Dataset::new("Data").with_data(vec![120.0, 190.0, 150.0, 220.0]));
        if let Some(mut chart) = self.ui.widget(id!(detail_polar_area_4)).borrow_mut::<PolarAreaChart>() {
            chart.set_data(data_4.clone());
            chart.replay_animation(cx);
        }

        // 8 segments
        let data_8 = ChartData::new()
            .with_labels(vec!["N", "NE", "E", "SE", "S", "SW", "W", "NW"])
            .add_dataset(Dataset::new("Wind").with_data(vec![80.0, 60.0, 100.0, 70.0, 90.0, 50.0, 110.0, 75.0]));
        if let Some(mut chart) = self.ui.widget(id!(detail_polar_area_8)).borrow_mut::<PolarAreaChart>() {
            chart.set_data(data_8);
            chart.replay_animation(cx);
        }

        // Gradient polar area
        if let Some(mut chart) = self.ui.widget(id!(detail_polar_gradient)).borrow_mut::<PolarAreaChart>() {
            chart.set_data(data_6.clone());
            chart.set_gradient(true);
            chart.replay_animation(cx);
        }

        // Gradient 4-segment polar area
        if let Some(mut chart) = self.ui.widget(id!(detail_polar_gradient_4)).borrow_mut::<PolarAreaChart>() {
            chart.set_data(data_4.clone());
            chart.set_gradient(true);
            chart.replay_animation(cx);
        }

        self.ui.redraw(cx);
    }

    fn setup_bubble_detail_charts(&mut self, cx: &mut Cx) {
        // Basic (with EaseOutBounce animation - bouncy bubble growth)
        let data = ChartData::new()
            .add_dataset(Dataset::new("Bubbles").with_bubble_data(vec![
                (10.0, 20.0, 15.0), (25.0, 40.0, 20.0), (40.0, 30.0, 18.0),
                (55.0, 50.0, 25.0), (70.0, 35.0, 12.0)
            ]));
        if let Some(mut chart) = self.ui.widget(id!(detail_bubble_basic)).borrow_mut::<BubbleChart>() {
            chart.set_data(data.clone());
            chart.set_options(ChartOptions::new()
                .with_animation_duration(1500.0)
                .with_animation_easing(EasingType::EaseOutBounce));
        }

        // Multi-dataset
        let multi_data = ChartData::new()
            .add_dataset(Dataset::new("Group A").with_bubble_data(vec![
                (10.0, 20.0, 15.0), (30.0, 40.0, 20.0), (50.0, 30.0, 12.0)
            ]))
            .add_dataset(Dataset::new("Group B").with_bubble_data(vec![
                (20.0, 35.0, 18.0), (40.0, 25.0, 22.0), (60.0, 45.0, 16.0)
            ]));
        if let Some(mut chart) = self.ui.widget(id!(detail_bubble_multi)).borrow_mut::<BubbleChart>() {
            chart.set_data(multi_data.clone());
            chart.replay_animation(cx);
        }

        // Large bubbles
        let large_data = ChartData::new()
            .add_dataset(Dataset::new("Large").with_bubble_data(vec![
                (20.0, 30.0, 35.0), (50.0, 50.0, 40.0), (80.0, 35.0, 30.0)
            ]));
        if let Some(mut chart) = self.ui.widget(id!(detail_bubble_large)).borrow_mut::<BubbleChart>() {
            chart.set_data(large_data);
            chart.replay_animation(cx);
        }

        // Gradient bubbles
        if let Some(mut chart) = self.ui.widget(id!(detail_bubble_gradient)).borrow_mut::<BubbleChart>() {
            chart.set_data(data.clone());
            chart.set_gradient(true);
            chart.replay_animation(cx);
        }

        // Gradient multi-dataset bubbles
        if let Some(mut chart) = self.ui.widget(id!(detail_bubble_gradient_multi)).borrow_mut::<BubbleChart>() {
            chart.set_data(multi_data.clone());
            chart.set_gradient(true);
            chart.replay_animation(cx);
        }

        self.ui.redraw(cx);
    }

    fn setup_combo_detail_charts(&mut self, cx: &mut Cx) {
        let data = ChartData::new()
            .with_labels(vec!["Jan", "Feb", "Mar", "Apr", "May", "Jun"])
            .add_dataset(Dataset::new("Sales").with_data(vec![65.0, 59.0, 80.0, 81.0, 56.0, 72.0]))
            .add_dataset(Dataset::new("Trend").with_data(vec![60.0, 62.0, 70.0, 75.0, 65.0, 70.0]));

        // Bar + Line (with EaseOutBounce animation - bouncy effect)
        if let Some(mut chart) = self.ui.widget(id!(detail_combo_bar_line)).borrow_mut::<ComboChart>() {
            chart.set_data(data.clone());
            chart.set_dataset_types(vec![DatasetType::Bar, DatasetType::Line]);
            chart.set_options(ChartOptions::new()
                .with_begin_at_zero(true)
                .with_animation_duration(1500.0)
                .with_animation_easing(EasingType::EaseOutBounce));
        }

        // Line + Bar
        if let Some(mut chart) = self.ui.widget(id!(detail_combo_line_bar)).borrow_mut::<ComboChart>() {
            chart.set_data(data.clone());
            chart.set_dataset_types(vec![DatasetType::Line, DatasetType::Bar]);
            chart.set_options(ChartOptions::new().with_begin_at_zero(true));
            chart.replay_animation(cx);
        }

        // Stacked with line
        let stacked_combo = ChartData::new()
            .with_labels(vec!["Jan", "Feb", "Mar", "Apr", "May", "Jun"])
            .add_dataset(Dataset::new("A").with_data(vec![40.0, 35.0, 50.0, 45.0, 35.0, 45.0]))
            .add_dataset(Dataset::new("B").with_data(vec![25.0, 24.0, 30.0, 36.0, 21.0, 27.0]))
            .add_dataset(Dataset::new("Avg").with_data(vec![60.0, 62.0, 70.0, 75.0, 65.0, 70.0]));
        if let Some(mut chart) = self.ui.widget(id!(detail_combo_stacked)).borrow_mut::<ComboChart>() {
            chart.set_data(stacked_combo);
            chart.set_dataset_types(vec![DatasetType::Bar, DatasetType::Bar, DatasetType::Line]);
            chart.set_options(ChartOptions::new().with_begin_at_zero(true));
            chart.replay_animation(cx);
        }

        // Gradient combo
        if let Some(mut chart) = self.ui.widget(id!(detail_combo_gradient)).borrow_mut::<ComboChart>() {
            chart.set_data(data.clone());
            chart.set_dataset_types(vec![DatasetType::Bar, DatasetType::Line]);
            chart.set_gradient(true);
            chart.set_options(ChartOptions::new().with_begin_at_zero(true));
            chart.replay_animation(cx);
        }

        self.ui.redraw(cx);
    }

    fn setup_chord_detail_charts(&mut self, cx: &mut Cx) {
        // Chord data - relationship matrix between 5 departments
        let chord_data = ChordData::new()
            .with_labels(vec!["Sales", "Marketing", "Engineering", "Support", "Finance"])
            .with_matrix(vec![
                vec![0.0, 80.0, 30.0, 50.0, 20.0],
                vec![40.0, 0.0, 60.0, 30.0, 10.0],
                vec![20.0, 50.0, 0.0, 70.0, 40.0],
                vec![60.0, 20.0, 40.0, 0.0, 30.0],
                vec![30.0, 15.0, 25.0, 20.0, 0.0],
            ]);

        // Basic Chord
        if let Some(mut chart) = self.ui.widget(id!(detail_chord_basic)).borrow_mut::<ChordChart>() {
            chart.set_data(chord_data.clone());
            chart.set_options(ChartOptions::new().with_animation_duration(1000.0));
            chart.replay_animation(cx);
        }

        // Gradient Ribbons
        if let Some(mut chart) = self.ui.widget(id!(detail_chord_gradient)).borrow_mut::<ChordChart>() {
            chart.set_data(chord_data.clone());
            chart.set_gradient(true);
            chart.set_options(ChartOptions::new().with_animation_duration(1000.0));
            chart.replay_animation(cx);
        }

        // Directed (Arrows)
        if let Some(mut chart) = self.ui.widget(id!(detail_chord_directed)).borrow_mut::<ChordChart>() {
            chart.set_data(chord_data.clone());
            chart.set_directed(true);
            chart.set_options(ChartOptions::new().with_animation_duration(1000.0));
            chart.replay_animation(cx);
        }

        // Arc Gradient
        if let Some(mut chart) = self.ui.widget(id!(detail_chord_arc_gradient)).borrow_mut::<ChordChart>() {
            chart.set_data(chord_data.clone());
            chart.set_arc_gradient(true);
            chart.set_options(ChartOptions::new().with_animation_duration(1000.0));
            chart.replay_animation(cx);
        }

        // Directed + Gradient (Full effects)
        if let Some(mut chart) = self.ui.widget(id!(detail_chord_directed_gradient)).borrow_mut::<ChordChart>() {
            chart.set_data(chord_data.clone());
            chart.set_directed(true);
            chart.set_gradient(true);
            chart.set_arc_gradient(true);
            chart.set_options(ChartOptions::new().with_animation_duration(1200.0));
            chart.replay_animation(cx);
        }

        // Thin Arcs
        if let Some(mut chart) = self.ui.widget(id!(detail_chord_thin_arcs)).borrow_mut::<ChordChart>() {
            chart.set_data(chord_data.clone());
            chart.set_arc_thickness(0.03);
            chart.set_gradient(true);
            chart.set_options(ChartOptions::new().with_animation_duration(1000.0));
            chart.replay_animation(cx);
        }

        // Wide Gap
        if let Some(mut chart) = self.ui.widget(id!(detail_chord_wide_gap)).borrow_mut::<ChordChart>() {
            chart.set_data(chord_data.clone());
            chart.set_gap_angle(0.12);
            chart.set_options(ChartOptions::new().with_animation_duration(1000.0));
            chart.replay_animation(cx);
        }

        // Bounce Animation
        if let Some(mut chart) = self.ui.widget(id!(detail_chord_bounce)).borrow_mut::<ChordChart>() {
            chart.set_data(chord_data.clone());
            chart.set_gradient(true);
            chart.set_options(ChartOptions::new()
                .with_animation_duration(1500.0)
                .with_animation_easing(EasingType::EaseOutBounce));
            chart.replay_animation(cx);
        }

        // Elastic Animation
        if let Some(mut chart) = self.ui.widget(id!(detail_chord_elastic)).borrow_mut::<ChordChart>() {
            chart.set_data(chord_data.clone());
            chart.set_directed(true);
            chart.set_gradient(true);
            chart.set_options(ChartOptions::new()
                .with_animation_duration(2000.0)
                .with_animation_easing(EasingType::EaseOutElastic));
            chart.replay_animation(cx);
        }

        self.ui.redraw(cx);
    }
}

app_main!(App);

fn main() {
    app_main();
}
