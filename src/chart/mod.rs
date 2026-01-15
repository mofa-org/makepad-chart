pub mod bar_chart;
pub mod line_chart;
pub mod pie_chart;
pub mod scatter_chart;
pub mod radar_chart;
pub mod polar_area_chart;
pub mod bubble_chart;
pub mod horizontal_bar_chart;
pub mod combo_chart;
pub mod chord_chart;

pub use bar_chart::*;
pub use line_chart::*;
pub use pie_chart::*;
pub use scatter_chart::*;
pub use radar_chart::*;
pub use polar_area_chart::*;
pub use bubble_chart::*;
pub use horizontal_bar_chart::*;
pub use combo_chart::*;
pub use chord_chart::*;

use makepad_widgets::*;

pub fn live_design(cx: &mut Cx) {
    bar_chart::live_design(cx);
    line_chart::live_design(cx);
    pie_chart::live_design(cx);
    scatter_chart::live_design(cx);
    radar_chart::live_design(cx);
    polar_area_chart::live_design(cx);
    bubble_chart::live_design(cx);
    horizontal_bar_chart::live_design(cx);
    combo_chart::live_design(cx);
    chord_chart::live_design(cx);
}
