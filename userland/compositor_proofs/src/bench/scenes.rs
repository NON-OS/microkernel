// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The scripted scenes: a wallpaper, one window and a translucent dock, the
//! arrow drawn by the compositor, and one thing changing each frame. The
//! click scene has a second window under the first, raised in turn.

use crate::damage::Rect;
use crate::scene::Layer;
use crate::scene_raise::raise_by_pid;
use crate::state::cursor_pick::reach;

use super::desk::Desk;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    /// The window dragged across the screen, the pointer with it.
    Drag,
    /// The pointer moved over the desktop.
    Cursor,
    /// A caret blinking: one glyph cell of the window redrawn.
    Caret,
    /// The window's content scrolled: the whole window redrawn.
    Scroll,
    /// The window resized from its corner, 8 pixels a frame out and back,
    /// drawn whole at each new size.
    Resize,
    /// The pointer moved over a 120x40 button on the window, the button lit.
    Hover,
    /// A click on the window under the first: raised, its button redrawn.
    Click,
}

impl Kind {
    pub const ALL: [Kind; 7] = [
        Kind::Drag,
        Kind::Cursor,
        Kind::Caret,
        Kind::Scroll,
        Kind::Resize,
        Kind::Hover,
        Kind::Click,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Kind::Drag => "drag",
            Kind::Cursor => "cursor",
            Kind::Caret => "caret",
            Kind::Scroll => "scroll",
            Kind::Resize => "resize",
            Kind::Hover => "hover",
            Kind::Click => "click",
        }
    }
}

const WALL: (u32, u64) = (1, 1);
const WINDOW: (u32, u64) = (2, 2);
const DOCK: (u32, u64) = (1, 3);
const UNDER: (u32, u64) = (4, 4);
/// A step of the resize scene, in pixels.
const STEP: u32 = 8;

fn layer(who: (u32, u64), r: Rect, z: u32) -> Layer {
    Layer {
        owner_pid: who.0,
        surface_handle: who.1,
        x: r.x,
        y: r.y,
        width: r.width,
        height: r.height,
        z,
        stack: 0,
        in_use: true,
        window_id: 0,
        keyed: false,
        miss_count: 0,
        cursor: None,
        content: Default::default(),
        opaque: false,
    }
}

/// Pixels a glyph cell is: 8 by 16 at 1080p, twice that at 2160p.
pub fn cell(height: u32) -> (u32, u32) {
    let k = (height / 1080).max(1);
    (8 * k, 16 * k)
}

pub fn window_rect(width: u32, height: u32) -> Rect {
    Rect { x: width / 8, y: height / 8, width: width / 2, height: height / 2 }
}

/// The click scene's second window, half under the first.
pub fn under_rect(width: u32, height: u32) -> Rect {
    let w = window_rect(width, height);
    Rect { x: w.x + w.width / 2, y: w.y + w.height / 3, ..w }
}

/// A 120x40 button in the middle of `win`, as many cells as at 1080p.
pub fn button_rect(win: Rect, height: u32) -> Rect {
    let (cw, ch) = cell(height);
    let (bw, bh) = (15 * cw, 5 * ch / 2);
    Rect {
        x: win.x + (win.width - bw) / 2,
        y: win.y + (win.height - bh) / 2,
        width: bw,
        height: bh,
    }
}

fn window_px(x: u32, y: u32) -> u32 {
    0xFFE0_E0E0 ^ ((x ^ y) & 0x0F)
}

pub fn caret_rect(width: u32, height: u32) -> Rect {
    let w = window_rect(width, height);
    let (cw, ch) = cell(height);
    Rect { x: w.x + 10 * cw, y: w.y + 4 * ch, width: cw, height: ch }
}

/// The desk at rest, composed once.
pub fn desk(width: u32, height: u32) -> Desk {
    let mut d = Desk::new(width, height);
    let screen = Rect { x: 0, y: 0, width, height };
    d.kernel.register(WALL.1, width, height, |x, y| 0xFF00_0000 | (x & 0xFF) << 16 | (y & 0xFF));
    d.submit(layer(WALL, screen, 0));
    let w = window_rect(width, height);
    d.kernel.register(WINDOW.1, w.width, w.height, window_px);
    d.submit(layer(WINDOW, w, 2));
    let dock =
        Rect { x: width / 4, y: height - height / 16, width: width / 2, height: height / 16 };
    d.kernel.register(DOCK.1, dock.width, dock.height, |x, _| 0x8020_3040 | (x & 0x3F));
    d.submit(layer(DOCK, dock, 3));
    d.cursor = Some((width / 2, height / 2));
    d
}

fn move_cursor(d: &mut Desk, to: (u32, u32)) {
    if let Some((x, y)) = d.cursor {
        d.damage.accumulate(reach(x, y, d.width, d.height));
    }
    d.cursor = Some(to);
    d.damage.accumulate(reach(to.0, to.1, d.width, d.height));
}

/// The click scene's desk: the second window under the first.
pub fn desk_for(kind: Kind, width: u32, height: u32) -> Desk {
    let mut d = desk(width, height);
    if kind == Kind::Click {
        let u = under_rect(width, height);
        d.kernel.register(UNDER.1, u.width, u.height, |x, y| 0xFF30_6090 ^ ((x + y) & 0x1F));
        d.submit(layer(UNDER, u, 2));
        let _ = raise_by_pid(&mut d.scene, WINDOW.0);
    }
    d
}

/// A button on `win` drawn lit or not, and damaged.
fn light(d: &mut Desk, handle: u64, win: Rect, lit: bool) {
    let b = button_rect(win, d.height);
    if let Some(px) = d.kernel.pixels(handle) {
        let v = if lit { 0xFF22_C3F0 } else { 0xFFE0_E0E0 };
        for y in b.y - win.y..b.y - win.y + b.height {
            let row = (y * win.width + b.x - win.x) as usize;
            px[row..row + b.width as usize].fill(v);
        }
    }
    d.damage.accumulate(b);
}

/// The window drawn again at a new size, as a client's new buffer is: the
/// old one let go, the new one attached at the first frame that shows it.
fn resize(d: &mut Desk, i: u32) {
    let base = window_rect(d.width, d.height);
    let k = i % 64;
    let grown = STEP * k.min(64 - k);
    let at = Rect { width: base.width + grown, height: base.height + grown, ..base };
    d.attach.forget(WINDOW.1, &mut d.kernel);
    d.kernel.register(WINDOW.1, at.width, at.height, window_px);
    d.submit(layer(WINDOW, at, 2));
    move_cursor(d, (at.x + at.width, at.y + at.height));
}

/// What the clients and the pointer do before frame `i`.
pub fn step(d: &mut Desk, kind: Kind, i: u32) {
    let (w, h) = (d.width, d.height);
    match kind {
        Kind::Drag => {
            let base = window_rect(w, h);
            let (dx, dy) = ((w / 240).max(1), (h / 270).max(1));
            let room = (w - base.width - base.x, h - base.height - base.y);
            let at = Rect { x: base.x + (i * dx) % room.0, y: base.y + (i * dy) % room.1, ..base };
            d.submit(layer(WINDOW, at, 2));
            move_cursor(d, (at.x + 40, at.y + 10));
        }
        Kind::Cursor => {
            let (dx, dy) = ((w / 240).max(1), (h / 270).max(1));
            move_cursor(d, ((i * dx) % w, (h / 4 + i * dy) % h));
        }
        Kind::Caret => {
            let win = window_rect(w, h);
            let c = caret_rect(w, h);
            let lit = i.is_multiple_of(2);
            if let Some(px) = d.kernel.pixels(WINDOW.1) {
                for y in c.y - win.y..c.y - win.y + c.height {
                    let row = (y * win.width + c.x - win.x) as usize;
                    let v = if lit { 0xFF00_0000 } else { 0xFFE0_E0E0 };
                    px[row..row + c.width as usize].fill(v);
                }
            }
            d.damage.accumulate(c);
        }
        Kind::Scroll => {
            let win = window_rect(w, h);
            let (_, ch) = cell(h);
            if let Some(px) = d.kernel.pixels(WINDOW.1) {
                px.rotate_left((ch * win.width) as usize);
            }
            d.damage.accumulate(win);
        }
        Kind::Resize => resize(d, i),
        Kind::Hover => {
            let win = window_rect(w, h);
            let b = button_rect(win, h);
            move_cursor(d, (b.x + b.width / 2 + i % 2, b.y + b.height / 2));
            light(d, WINDOW.1, win, i.is_multiple_of(2));
        }
        Kind::Click => {
            let (top, handle, win) = match i % 2 {
                0 => (UNDER.0, UNDER.1, under_rect(w, h)),
                _ => (WINDOW.0, WINDOW.1, window_rect(w, h)),
            };
            let b = button_rect(win, h);
            move_cursor(d, (b.x + b.width / 2, b.y + b.height / 2));
            if let Some(r) = raise_by_pid(&mut d.scene, top) {
                d.damage.accumulate(r);
            }
            light(d, handle, win, true);
        }
    }
}
