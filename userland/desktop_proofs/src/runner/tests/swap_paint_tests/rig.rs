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

//! The two paths over one model app. The app's content is a grid of
//! colours a frame changes inside its damage; the frame is one colour that
//! only a whole paint changes. Each path keeps a screen: what the
//! compositor composed, which takes only damaged pixels from what the
//! layer shows.

use super::super::super::chrome::quarters;
use super::super::super::finish_band::finish_band;
use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::decorations::{content_rect_at, Rect};

pub const W: u32 = 160;
pub const H: u32 = 120;
const LEN: usize = (W * H) as usize;

pub fn content() -> Rect {
    content_rect_at(W, H, false, quarters())
}

pub struct App {
    pub cells: Vec<u32>,
    pub frame: u32,
}

impl App {
    pub fn new() -> App {
        let c = content();
        App { cells: vec![0xFF10_2030; (c.w * c.h) as usize], frame: 0xFF2A_2E36 }
    }

    /// The content area drawn whole, as an app's `paint` draws it.
    pub fn paint_content(&self, px: &mut [u32]) {
        let c = content();
        for y in 0..c.h {
            let row = ((c.y + y) * W + c.x) as usize;
            px[row..row + c.w as usize]
                .copy_from_slice(&self.cells[(y * c.w) as usize..][..c.w as usize]);
        }
    }

    /// The cells in `r` (content coordinates, cut to the content) alone, as
    /// an app that named `r` as its damage paints: every pixel inside it
    /// and nothing else (`App::take_damage`).
    pub fn paint_rect(&self, px: &mut [u32], r: Rect) {
        let c = content();
        for y in r.y..r.y + r.h {
            let row = ((c.y + y) * W + c.x + r.x) as usize;
            let cells = ((y * c.w) + r.x) as usize;
            px[row..row + r.w as usize].copy_from_slice(&self.cells[cells..cells + r.w as usize]);
        }
    }

    pub fn draw_whole(&self, px: &mut [u32]) {
        px.fill(self.frame);
        self.paint_content(px);
        let c = content();
        finish_band(
            &mut PaintBuffer { pixels: px, stride_words: W, width: W, height: H },
            false,
            (c.x, c.y, c.x + c.w, c.y + c.h),
        );
    }

    /// Content cells in `r` (content coordinates) take colour `v`.
    pub fn change(&mut self, r: Rect, v: u32) {
        let c = content();
        for y in r.y..r.y + r.h {
            for x in r.x..r.x + r.w {
                self.cells[(y * c.w + x) as usize] = v;
            }
        }
    }
}

/// `r` cut to the content area, as `paint_partial` cuts it.
pub fn clip(r: Rect) -> Rect {
    let c = content();
    let (x, y) = (r.x.min(c.w), r.y.min(c.h));
    Rect { x, y, w: r.w.min(c.w - x), h: r.h.min(c.h - y) }
}

fn compose(screen: &mut [u32], from: &[u32], d: Rect) {
    for y in d.y..d.y + d.h {
        let at = (y * W + d.x) as usize;
        screen[at..at + d.w as usize].copy_from_slice(&from[at..at + d.w as usize]);
    }
}

/// The single surface: a partial repaint drawn off it, its content rows
/// written back, the damage committed.
pub struct Old {
    pub surface: Vec<u32>,
    pub screen: Vec<u32>,
}

impl Old {
    pub fn new() -> Old {
        Old { surface: vec![0; LEN], screen: vec![0; LEN] }
    }

    pub fn whole(&mut self, app: &App) {
        app.draw_whole(&mut self.surface);
        self.screen.copy_from_slice(&self.surface);
    }

    pub fn partial(&mut self, app: &App, r: Rect) {
        let (c, r) = (content(), clip(r));
        let mut back = self.surface.clone();
        app.paint_content(&mut back);
        if r.w != 0 && r.h != 0 {
            let fb = &mut PaintBuffer { pixels: &mut back, stride_words: W, width: W, height: H };
            finish_band(fb, false, (c.x, c.y + r.y, c.x + c.w, c.y + r.y + r.h));
        }
        let rows = (c.y * W) as usize..((c.y + c.h) * W) as usize;
        self.surface[rows.clone()].copy_from_slice(&back[rows]);
        if r.w != 0 && r.h != 0 {
            compose(
                &mut self.screen,
                &self.surface,
                Rect { x: c.x + r.x, y: c.y + r.y, w: r.w, h: r.h },
            );
        }
    }
}
