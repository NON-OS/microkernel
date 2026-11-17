extern crate alloc;
use crate::viewer::gallery::layout::{cell_rect, grid, header_h, thumb_h, thumb_w, Grid};
use crate::viewer::gallery::state::GalleryState;
use crate::viewer::says::{gallery_next, gallery_note};
use crate::viewer::scale::{draw_nn, Dst};
use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::theme::derive::mix;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{draw, Rect};

/// The bitmap face's advance, for centring a line by its length.
const GLYPH_W: u32 = 8;
/// Where the header's title starts and how thick the selected tile's bars
/// are, in logical pixels, and the gallery's glyph size as a multiple of the
/// bitmap face at 1x.
const HEADER_X: u32 = 10;
const HEADER_Y: u32 = 8;
const SELECTED_BAR: u32 = 2;
const GLYPHS: u32 = 1;

/// Paints the gallery, and washes the tile `lit` as hovered when there is one.
pub fn paint_gallery(g: &mut GalleryState, fb: &mut PaintBuffer, lit: Option<usize>) {
    let (w, h, r, s) = (fb.width, fb.height, roles(), scale());
    let (text, glyphs) = (r.text.argb(), s.px(GLYPHS));
    fb.fill_rect(0, 0, w, h, r.bg.argb());
    let gr = grid(w);
    let n = g.entries.len();
    if let Some(note) = gallery_note(g.scanned, g.scan_error, n) {
        let x = (w / 2).saturating_sub(note.len() as u32 * GLYPH_W * glyphs / 2);
        fb.text_scaled(x, h / 2, note.as_bytes(), text, glyphs);
        if let Some(next) = gallery_next(g.scanned, g.scan_error, n) {
            let x = (w / 2).saturating_sub(next.len() as u32 * GLYPH_W * glyphs / 2);
            let y = h / 2 + 2 * GLYPH_W * glyphs;
            fb.text_scaled(x, y, next.as_bytes(), r.muted.argb(), glyphs);
        }
        return;
    }
    paint_boxes(fb, g, &gr, n);
    blit_thumbs(fb, g, &gr, n);
    if let Some(i) = lit {
        let (x, y, _, _) = cell_rect(i, g.scroll, &gr);
        let at = Rect { x: x.max(0) as u32, y: y.max(0) as u32, w: thumb_w(), h: thumb_h() };
        draw::blend(fb, at, r.hover_wash);
    }
    let (x, y) = (s.px(HEADER_X), s.px(HEADER_Y));
    fb.text_scaled(x, y, b"Gallery", text, glyphs);
    if let Some(trouble) = g.decoder_trouble {
        let x = x + 9 * GLYPH_W * glyphs;
        fb.text_scaled(x, y, trouble.as_bytes(), r.danger.argb(), glyphs);
    }
}

fn paint_boxes(fb: &mut PaintBuffer, g: &GalleryState, gr: &Grid, n: usize) {
    let (h, r, s) = (fb.height, roles(), scale());
    let (tw, th, bar) = (thumb_w(), thumb_h(), s.px(SELECTED_BAR));
    for i in 0..n {
        let (x, y, _cw, ch) = cell_rect(i, g.scroll, gr);
        if y + ch as i32 <= header_h() as i32 || y >= h as i32 {
            continue;
        }
        let (ux, uy) = (x.max(0) as u32, y.max(0) as u32);
        let e = &g.entries[i];
        let base = if e.failed {
            mix(r.surface.argb(), r.danger.argb(), 40)
        } else if e.thumb.is_none() {
            r.surface.argb()
        } else {
            r.bg.argb()
        };
        fb.fill_rect(ux, uy, tw, th, base);
        if i == g.sel {
            fb.fill_rect(ux, uy, tw, bar, r.accent.argb());
            fb.fill_rect(ux, uy + th - bar, tw, bar, r.accent.argb());
        }
        fb.text_scaled(ux, uy + th, label(&e.path), r.text.argb(), s.px(GLYPHS));
    }
}

fn blit_thumbs(fb: &mut PaintBuffer, g: &GalleryState, gr: &Grid, n: usize) {
    let (w, h, stride) = (fb.width, fb.height, fb.stride_words);
    let mut dst = Dst { px: fb.pixels, stride, w, h };
    for i in 0..n {
        let (x, y, _cw, ch) = cell_rect(i, g.scroll, gr);
        if y + ch as i32 <= header_h() as i32 || y >= h as i32 {
            continue;
        }
        if let Some(thumb) = g.entries[i].thumb.as_ref() {
            let (tw, th) = (g.entries[i].tw, g.entries[i].th);
            let ox = x + ((thumb_w() - tw) / 2) as i32;
            let oy = y + ((thumb_h() - th) / 2) as i32;
            draw_nn(&mut dst, thumb, tw, th, ox, oy, tw, th);
        }
    }
}

fn label(path: &str) -> &[u8] {
    let b = path.as_bytes();
    let start = match b.iter().rposition(|&c| c == b'/') {
        Some(i) => i + 1,
        None => 0,
    };
    let end = (start + 18).min(b.len());
    &b[start..end]
}
