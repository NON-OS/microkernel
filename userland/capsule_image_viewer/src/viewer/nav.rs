use nonos_toolkit::design::scale::scale;
use nonos_toolkit::widgets::Rect;

/// A prev or next button's size, its inset from the window's side and how far
/// a drag must travel to swipe, in logical pixels.
const BTN_W: u32 = 40;
const BTN_H: u32 = 64;
const MARGIN: u32 = 8;
const SWIPE_MIN: u32 = 80;

pub fn button_rects(vw: u32, vh: u32) -> (Rect, Rect) {
    let s = scale();
    let (w, h, inset) = (s.px(BTN_W), s.px(BTN_H), s.px(MARGIN));
    let y = vh.saturating_sub(h) / 2;
    let left = Rect { x: inset, y, w, h };
    let right = Rect { x: vw.saturating_sub(inset + w), y, w, h };
    (left, right)
}

pub fn swipe_delta(dx: i32, dy: i32, pannable: bool) -> Option<i32> {
    let swipe_min = scale().px(SWIPE_MIN) as i32;
    if pannable || dx.abs() <= swipe_min || dx.abs() <= dy.abs() {
        return None;
    }
    if dx < 0 {
        Some(1)
    } else {
        Some(-1)
    }
}
