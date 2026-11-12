use crate::viewer::nav::arrow_box;
use crate::viewer::overlay;
use crate::viewer::scale::{draw_bilinear, draw_nn, Dst};
use crate::viewer::state::ViewerState;
use crate::viewer::target::{controls, Target};
use crate::viewer::viewport::{clamp_pan_mode, place_mode, FitMode};
use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::icons::IconId;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{draw, Button, Kind};

/// The status line's inset from the left edge and how far above the bottom
/// edge it starts, in logical pixels, and its glyph size as a multiple of the
/// 8 pixel bitmap face at 1x.
const STATUS_X: u32 = 8;
const STATUS_RISE: u32 = 20;
const STATUS_GLYPHS: u32 = 1;

pub fn paint(st: &mut ViewerState, fb: &mut PaintBuffer) {
    st.view_w = fb.width;
    st.view_h = fb.height;
    fb.fill_rect(0, 0, fb.width, fb.height, roles().bg.argb());
    match st.img.as_ref() {
        Some(img) => {
            clamp_pan_mode(&mut st.view, st.fit_mode, img.w, img.h, fb.width, fb.height);
            let p = place_mode(st.fit_mode, img.w, img.h, fb.width, fb.height, &st.view);
            let (w, h, stride) = (fb.width, fb.height, fb.stride_words);
            let mut dst = Dst { px: fb.pixels, stride, w, h };
            if st.fit_mode == FitMode::Actual && st.view.zoom == 1.0 {
                draw_nn(&mut dst, &img.px, img.w, img.h, p.dx, p.dy, p.dw, p.dh);
            } else {
                draw_bilinear(&mut dst, &img.px, img.w, img.h, p.dx, p.dy, p.dw, p.dh);
            }
        }
        None => {}
    }
    if !st.status.is_empty() {
        let (s, ink) = (scale(), roles().text.argb());
        let y = fb.height.saturating_sub(s.px(STATUS_RISE));
        fb.text_scaled(s.px(STATUS_X), y, st.status.as_bytes(), ink, s.px(STATUS_GLYPHS));
    }
    overlay::draw_info(fb, st);
    overlay::draw_help(fb, st);
    overlay::draw_slideshow(fb, st);
    paint_nav(fb, st);
}

/// Draws prev and next as `Secondary` button bodies, whose opaque fill reads
/// over any picture, each with its chevron icon in the label's ink, centred
/// where `arrow_box` puts it.
fn paint_nav(fb: &mut PaintBuffer, st: &ViewerState) {
    let (shown, ink) = (st.shown(), roles().text);
    for (t, at) in controls(&shown) {
        Button::new("").kind(Kind::Secondary).state(st.hover.look(&shown, t)).draw(fb, at);
        let id = if t == Target::Prev { IconId::ChevronLeft } else { IconId::ChevronRight };
        let (x, y, side) = arrow_box(at);
        draw::icon(fb, id, x, y, side, ink);
    }
}
