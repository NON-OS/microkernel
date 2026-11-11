// NONOS Operating System (AGPL-3.0-or-later)
//! The text grid and the splash at the display scale: at 1x, 1.25x, 1.5x and
//! 2x every zoom's point size, row height and insets are the 1x ones scaled
//! once, the grid fits a window grown by the same scale with the rail open or
//! shut, and the banner and swatches step and size by the scale.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::scale::{scale, set_scale};
use nonos_toolkit::paint::measure_ttf_mono;

use crate::constants::{body_pad_top, text_left};
use crate::fetch_banner::{draw_banner, BANNER};
use crate::fetch_palette::draw_palette;
use crate::geometry::{geometry, grid_size};
use crate::metrics::Metrics;
use crate::strip_boxes::serial;
use crate::theme_profiles::DARK;

const QS: [u32; 4] = [4, 5, 6, 8];
const WIN: (u32, u32) = (960, 540);

fn with_fb<R>(w: u32, h: u32, f: impl FnOnce(&mut PaintBuffer) -> R) -> (R, Vec<u32>) {
    let mut px = vec![0u32; (w * h) as usize];
    let r = f(&mut PaintBuffer { pixels: &mut px, stride_words: w, width: w, height: h });
    (r, px)
}

#[test]
fn every_zoom_is_the_1x_grid_scaled_once() {
    let _g = serial();
    set_scale(4);
    let one: Vec<_> = (1..=6).map(|z| with_fb(8, 8, |fb| Metrics::new(fb, z)).0).collect();
    let (inset, pad) = (text_left(), body_pad_top());
    for q in QS {
        set_scale(q);
        let s = scale();
        assert_eq!((text_left(), body_pad_top()), (s.px(inset), s.px(pad)), "q={q}");
        for (z, m1) in (1..=6).zip(&one) {
            let m = with_fb(8, 8, |fb| Metrics::new(fb, z)).0;
            assert_eq!((m.px, m.lh), (s.font(m1.px), s.px(m1.lh)), "q={q} zoom={z}");
            assert_eq!(m.adv, measure_ttf_mono("M", m.px).max(1) as u32, "q={q} zoom={z}");
        }
    }
    set_scale(4);
}

#[test]
fn the_grid_fits_the_window_grown_by_the_scale() {
    let _g = serial();
    for q in QS {
        set_scale(q);
        let (w, h) = (scale().px(WIN.0), scale().px(WIN.1));
        for (z, open) in (1..=6).flat_map(|z| [(z, false), (z, true)]) {
            let ((l, m, c), _) = with_fb(w, h, |fb| geometry(fb, z, open));
            let ((cols, rows, _), _) = with_fb(w, h, |fb| grid_size(fb, z, open));
            let rail = if open { scale().px(320) } else { 0 };
            assert_eq!((l.left_rail.w, l.body.x, l.body.w), (rail, rail, w - rail), "q={q}");
            assert_eq!(c.text_left, text_left());
            assert!(2 * c.text_left + cols as u32 * m.adv <= l.body.w, "q={q} zoom={z}");
            assert!(rows as u32 * m.lh <= l.body.h + l.input.h, "q={q} zoom={z}");
            assert!(l.footer.y + l.footer.h <= h && l.input.y + l.input.h <= l.footer.y);
        }
    }
    set_scale(4);
}

#[test]
fn the_banner_and_swatches_step_by_the_scale() {
    let _g = serial();
    for q in QS {
        set_scale(q);
        let s = scale();
        let (end, px) = with_fb(1600, 400, |fb| draw_banner(fb, 0, 10, 1600, &DARK));
        assert_eq!(end, 10 + BANNER.len() as i32 * s.px(15) as i32, "q={q}");
        assert!(px.iter().any(|&p| p != 0));
        assert_eq!(with_fb(1600, 400, |fb| draw_banner(fb, 0, 10, 40, &DARK)).0, 10);
        let (_, px) = with_fb(200, 40, |fb| draw_palette(fb, 0, 0, &[1, 2, 3]));
        let (pitch, sw, sh) = (s.px(18), s.px(14), s.px(10));
        let drawn = |x: u32, y: u32| x / pitch < 3 && x % pitch < sw && y < sh;
        for (n, &p) in px.iter().enumerate() {
            let (x, y) = (n as u32 % 200, n as u32 / 200);
            assert_eq!(p != 0, drawn(x, y), "q={q} at {x}, {y}");
        }
    }
    set_scale(4);
}
