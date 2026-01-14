// Component module - shared chart components

mod axis;
mod grid;
pub mod legend;
mod tooltip;
mod title;

pub use axis::*;
pub use grid::*;
pub use legend::*;
pub use tooltip::*;
pub use title::*;

use makepad_widgets::*;

pub fn live_design(cx: &mut Cx) {
    legend::live_design(cx);
}
