pub mod album;
pub mod ascii_art;
pub mod ascii_art_s1;
pub mod ascii_art_s2;
pub mod ascii_art_s3;
pub mod ascii_art_s4;
#[cfg(debug_assertions)]
pub mod debug_album;
pub mod main_screen;
pub mod naming;

use ratatui::layout::Rect;

/// Shrink a `Rect` by the given horizontal and vertical margins.
pub fn inner_area(area: Rect, h_margin: u16, v_margin: u16) -> Rect {
    Rect {
        x: area.x + h_margin,
        y: area.y + v_margin,
        width: area.width.saturating_sub(h_margin * 2),
        height: area.height.saturating_sub(v_margin * 2),
    }
}
