extern crate alloc;
use crate::viewer::caption::{interval_label, size_line, zoom_percent, KEYMAP};
use crate::viewer::state::ViewerState;
use crate::viewer::viewport::FitMode;
use alloc::format;
use alloc::string::String;
use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::theme::roles;

const PANEL_W: u32 = 260;
const INFO_H: u32 = 60;
/// A line's inset from a panel's left and top edges, the step from one info
/// line or one help row to the next, the gap between the two panels, the help
/// panel's padding and how far in from the right edge the slideshow line
/// starts, in logical pixels, as the panel sizes above are.
const TEXT_X: u32 = 8;
const TEXT_Y: u32 = 6;
const INFO_STEP: u32 = 18;
const HELP_STEP: u32 = 16;
const PANEL_GAP: u32 = 4;
const HELP_PAD: u32 = 8;
const SLIDESHOW_IN: u32 = 60;
/// The overlays' glyph size, as a multiple of the 8 pixel bitmap face at 1x.
const GLYPHS: u32 = 1;

fn basename(path: &str) -> &str {
    match path.rfind('/') {
        Some(i) => &path[i + 1..],
        None => path,
    }
}

fn ext_upper(path: &str) -> String {
    match path.rfind('.') {
        Some(i) => path[i + 1..].to_ascii_uppercase(),
        None => String::new(),
    }
}

fn mode_name(mode: FitMode) -> &'static str {
    match mode {
        FitMode::Fit => "FIT",
        FitMode::Actual => "ACTUAL",
        FitMode::Fill => "FILL",
    }
}

pub fn draw_info(fb: &mut PaintBuffer, st: &ViewerState) {
    if !st.info_visible {
        return;
    }
    let (theme, s) = (roles(), scale());
    let (ink, g) = (theme.text.argb(), s.px(GLYPHS));
    fb.blend_rect(0, 0, s.px(PANEL_W), s.px(INFO_H), theme.scrim.argb());
    let path = st.dir.get(st.idx).map(|p| p.as_str());
    let (x, y0) = (s.px(TEXT_X), |n: u32| s.px(TEXT_Y + n * INFO_STEP));
    fb.text_scaled(x, y0(0), path.map(basename).unwrap_or("(no image)").as_bytes(), ink, g);
    let dims = st.img.as_ref().map(|i| (i.w, i.h));
    let fmt = path.map(ext_upper).unwrap_or_default();
    fb.text_scaled(x, y0(1), size_line(dims, &fmt, st.file_size).as_bytes(), ink, g);
    let (iw, ih) = dims.unwrap_or((0, 0));
    let pct = zoom_percent(st.fit_mode, iw, ih, st.view_w, st.view_h, st.view.zoom);
    let line = format!("{}/{}  {}%  {}", st.idx + 1, st.dir.len(), pct, mode_name(st.fit_mode));
    fb.text_scaled(x, y0(2), line.as_bytes(), ink, g);
}

/// Below the info panel when both are open, so neither covers the other.
pub fn draw_help(fb: &mut PaintBuffer, st: &ViewerState) {
    if !st.help_visible {
        return;
    }
    let s = scale();
    let top = if st.info_visible { s.px(INFO_H + PANEL_GAP) } else { 0 };
    let h = s.px(HELP_PAD + KEYMAP.len() as u32 * HELP_STEP);
    let theme = roles();
    let (ink, x, g) = (theme.text.argb(), s.px(TEXT_X), s.px(GLYPHS));
    fb.blend_rect(0, top, s.px(PANEL_W), h, theme.scrim.argb());
    for (i, (keys, action)) in KEYMAP.iter().enumerate() {
        let y = top + s.px(TEXT_Y + i as u32 * HELP_STEP);
        fb.text_scaled(x, y, format!("{}  {}", keys, action).as_bytes(), ink, g);
    }
}

pub fn draw_slideshow(fb: &mut PaintBuffer, st: &ViewerState) {
    if !st.slideshow_on {
        return;
    }
    let line = format!("> {}", interval_label(st.interval_ms));
    let (s, ink) = (scale(), roles().text.argb());
    let x = fb.width.saturating_sub(s.px(SLIDESHOW_IN));
    fb.text_scaled(x, s.px(TEXT_Y), line.as_bytes(), ink, s.px(GLYPHS));
}
