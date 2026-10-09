extern crate alloc;
use crate::viewer::gallery::layout::{cell_rect, follow, grid, max_scroll, visible};
use crate::viewer::gallery::state::GalleryState;
use alloc::string::String;
use nonos_app_skeleton::input::{KEY_DOWN, KEY_ENTER, KEY_LEFT, KEY_RIGHT, KEY_UP};
use nonos_app_skeleton::{InputEvent, InputKind};

pub enum GalleryAction {
    None,
    Open(String),
    Repaint,
}

/// The tile under the point, among those on screen.
pub fn hit_tile(
    x: i32,
    y: i32,
    scroll: usize,
    win_w: u32,
    win_h: u32,
    count: usize,
) -> Option<usize> {
    let g = grid(win_w);
    let (start, end) = visible(count, scroll, win_h, &g);
    (start..end).find(|&i| {
        let (cx, cy, w, h) = cell_rect(i, scroll, &g);
        x >= cx && x < cx + w as i32 && y >= cy && y < cy + h as i32
    })
}

pub fn on_event(g: &mut GalleryState, ev: &InputEvent, win_w: u32, win_h: u32) -> GalleryAction {
    let count = g.entries.len();
    match ev.kind {
        InputKind::ButtonDown => match hit_tile(ev.x, ev.y, g.scroll, win_w, win_h, count) {
            Some(i) => {
                g.sel = i;
                GalleryAction::Open(g.entries[i].path.clone())
            }
            None => GalleryAction::None,
        },
        InputKind::Wheel => {
            scroll(g, ev.delta_y, win_h, win_w, count);
            GalleryAction::Repaint
        }
        InputKind::KeyDown => key(g, ev.code, win_w, win_h, count),
        _ => GalleryAction::None,
    }
}

fn scroll(g: &mut GalleryState, delta_y: i32, win_h: u32, win_w: u32, count: usize) {
    let gr = grid(win_w);
    let max = max_scroll(count, win_h, &gr);
    if delta_y > 0 {
        g.scroll = g.scroll.saturating_sub(1);
    } else {
        g.scroll = (g.scroll + 1).min(max);
    }
}

fn key(g: &mut GalleryState, code: u32, win_w: u32, win_h: u32, count: usize) -> GalleryAction {
    if count == 0 {
        return GalleryAction::None;
    }
    let gr = grid(win_w);
    let cols = gr.cols as usize;
    g.sel = match code {
        KEY_LEFT => g.sel.saturating_sub(1),
        KEY_RIGHT => (g.sel + 1).min(count - 1),
        KEY_UP => g.sel.saturating_sub(cols),
        KEY_DOWN => (g.sel + cols).min(count - 1),
        KEY_ENTER => return GalleryAction::Open(g.entries[g.sel].path.clone()),
        _ => return GalleryAction::None,
    };
    // The selection is drawn only on screen; the view follows it there.
    g.scroll = follow(g.scroll, g.sel, count, win_h, &gr);
    GalleryAction::Repaint
}
