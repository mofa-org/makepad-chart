pub mod bar;
pub mod line;
pub mod point;
pub mod arc;
pub mod triangle;
pub mod grid;

pub use bar::*;
pub use line::*;
pub use point::*;
pub use arc::*;
pub use triangle::*;
pub use grid::*;

use makepad_widgets::*;

pub fn live_design(cx: &mut Cx) {
    bar::live_design(cx);
    line::live_design(cx);
    point::live_design(cx);
    arc::live_design(cx);
    triangle::live_design(cx);
    grid::live_design(cx);
}
