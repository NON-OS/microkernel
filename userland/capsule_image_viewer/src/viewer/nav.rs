use nonos_toolkit::widgets::Rect;

pub const BTN_W: u32 = 40;
pub const BTN_H: u32 = 64;
pub const MARGIN: u32 = 8;
pub const SWIPE_MIN: i32 = 80;

pub fn button_rects(vw: u32, vh: u32) -> (Rect, Rect) {
    let y = vh.saturating_sub(BTN_H) / 2;
    let left = Rect { x: MARGIN, y, w: BTN_W, h: BTN_H };
    let right = Rect { x: vw.saturating_sub(MARGIN + BTN_W), y, w: BTN_W, h: BTN_H };
    (left, right)
}

pub fn swipe_delta(dx: i32, dy: i32, pannable: bool) -> Option<i32> {
    if pannable || dx.abs() <= SWIPE_MIN || dx.abs() <= dy.abs() {
        return None;
    }
    if dx < 0 {
        Some(1)
    } else {
        Some(-1)
    }
}
