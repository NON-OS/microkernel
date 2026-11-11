// NONOS Operating System (AGPL-3.0-or-later)
//! The shell keeps logical metrics and draws them at the display scale: at
//! 1x to 2x every chrome length is its 1x length through `scale().px`, every
//! text size through `scale().font`, and a window grown by the same scale
//! holds the chrome and a code pane with room for text. The page is laid out
//! once and carried onto the screen at the scale, and back for a hit test.

use nonos_toolkit::design::scale::{scale, set_scale};

use super::manifest::{HEIGHT, WIDTH};
use super::shell::pane_rect;
use super::square_tests::lock;
use crate::layout::{
    activity_w, body_px, chrome_px, footer_h, gutter_px, gutter_w, line_height, pad_top, ribbon_h,
    row_h, sidebar_w, tabbar_h, text_left, titlebar_h, wrap_cols, MAX_SCALE, MIN_SCALE,
};
use crate::sheet_scale::{on_page, on_screen};

/// Every chrome length, then every text size, the shell lays out with.
fn sizes() -> (Vec<u32>, Vec<f32>) {
    let mut lengths = vec![titlebar_h(), tabbar_h(), ribbon_h(), footer_h(), activity_w()];
    lengths.extend([sidebar_w(), gutter_w(), pad_top(), row_h()]);
    let mut fonts = vec![chrome_px()];
    for z in MIN_SCALE..=MAX_SCALE {
        lengths.push(line_height(z));
        fonts.extend([body_px(z), gutter_px(z)]);
    }
    (lengths, fonts)
}

#[test]
fn every_shell_size_grows_with_the_scale_and_the_window_still_fits() {
    let _g = lock();
    set_scale(4);
    let (lengths, fonts) = sizes();
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let s = scale();
        let px = lengths.iter().map(|&n| s.px(n)).collect();
        let want = (px, fonts.iter().map(|&f| s.font(f)).collect());
        assert_eq!(sizes(), want, "q={q}: a size is not its 1x size at the scale");
        let (w, h) = (s.px(WIDTH), s.px(HEIGHT));
        let (x, y, pw, ph) = pane_rect(w, h, true, true);
        assert_eq!((x, y), (activity_w() + sidebar_w(), titlebar_h() + tabbar_h() + ribbon_h()));
        assert_eq!((x + pw, y + ph + footer_h()), (w, h), "q={q}: the pane leaves the window");
        assert!(text_left(x) > x + gutter_w(), "q={q}: text starts inside the gutter");
        assert!(wrap_cols(pw, s.px(9)) >= 80, "q={q}: the pane has no room for text");
        assert!(pad_top() + line_height(MAX_SCALE) * 20 <= ph, "q={q}: rows overrun the pane");
    }
    set_scale(4);
}

#[test]
fn a_page_length_is_scaled_once_onto_the_screen_and_back() {
    let _g = lock();
    for q in [4, 5, 6, 8] {
        set_scale(q);
        for v in [0.0, 1.0, 56.0, 648.0, 760.0, 980.0] {
            assert_eq!(on_screen(v), scale().font(v), "q={q}: {v} is not drawn at the scale");
            assert!((on_page(on_screen(v)) - v).abs() < 1e-3, "q={q}: {v} does not come back");
        }
    }
    set_scale(4);
    assert_eq!((on_screen(760.0), on_page(760.0)), (760.0, 760.0), "1x is the page itself");
}
