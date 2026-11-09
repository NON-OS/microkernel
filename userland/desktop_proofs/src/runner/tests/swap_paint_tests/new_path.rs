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

//! The swapchain path, as `paint_frame` and `paint_partial` drive it, over
//! a modelled compositor: a present composes its damage from the buffer,
//! and the buffer it replaced is held `hold` frames before its release
//! count moves. Drawing into a buffer the compositor shows or holds fails
//! the test on the spot.

use super::super::super::finish_band::finish_band;
use super::rig::{clip, content, App, H, W};
use nonos_app_skeleton::runner::changed_band::{present_rows, Shown};
use nonos_app_skeleton::runner::swap_paint::whole_needed;
use nonos_app_skeleton::swap::{Chain, Damage, Part, Stale};
use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::decorations::Rect;

pub struct New {
    pub chain: Chain,
    pub bufs: [Vec<u32>; 2],
    counts: [u32; 2],
    pub shown: usize,
    /// The compositor's held buffers and the frames each has left.
    held: Vec<(usize, u32)>,
    pub hold: u32,
    pub screen: Vec<u32>,
    pub refuse_next: bool,
    /// False after a frame was not shown: the next paints the whole.
    pub primed: bool,
    /// The pixels every present had composed.
    pub composed: u64,
}

impl New {
    /// Buffer 0 submitted blank, as `open_window` leaves it.
    pub fn blank() -> New {
        let mut chain = Chain::new(2).expect("two buffers");
        chain.adopt(0).expect("nothing shown yet");
        let len = (W * H) as usize;
        let (bufs, screen) = ([vec![0; len], vec![0; len]], vec![0; len]);
        New {
            chain,
            bufs,
            counts: [0; 2],
            shown: 0,
            held: Vec::new(),
            hold: 0,
            screen,
            refuse_next: false,
            primed: false,
            composed: 0,
        }
    }

    /// One compositor frame: held buffers count down; one at zero is
    /// released, its count moved on.
    pub fn tick(&mut self) {
        let mut kept = Vec::new();
        for (j, left) in self.held.drain(..) {
            match left {
                0 => self.counts[j] += 1,
                _ => kept.push((j, left - 1)),
            }
        }
        self.held = kept;
    }

    fn acquire(&mut self) -> Option<(usize, Stale)> {
        let retiring: Vec<usize> = self.chain.retiring().map(|(j, _)| j).collect();
        for j in retiring {
            self.chain.fence(j, self.counts[j]);
        }
        let (i, stale) = self.chain.acquire()?;
        assert_ne!(i, self.shown, "drew into the buffer the compositor shows");
        assert!(self.held.iter().all(|&(j, _)| j != i), "drew into a buffer the compositor holds");
        Some((i, stale))
    }

    fn present(&mut self, i: usize, d: Damage) -> bool {
        self.present_parts(i, d, &[(d.x, d.y, d.width, d.height)])
    }

    /// Buffer `i` shown, the chain moved by `d`, and only `parts`, boxes
    /// inside `d`, composed from it, as OP_PRESENT_LAYER_PARTS has the
    /// compositor repaint them.
    fn present_parts(&mut self, i: usize, d: Damage, parts: &[Part]) -> bool {
        if std::mem::take(&mut self.refuse_next) {
            self.chain.cancel(i).expect("drawing");
            return false;
        }
        let counts = self.counts;
        self.chain.present(i, d, |j| counts[j]).expect("drawing");
        self.held.push((self.shown, self.hold));
        self.shown = i;
        for &(x, y, w, h) in parts {
            assert!(x >= d.x && y >= d.y && x + w <= d.x + d.width && y + h <= d.y + d.height);
            self.composed += u64::from(w) * u64::from(h);
            for row in y..y + h {
                let at = (row * W + x) as usize;
                self.screen[at..at + w as usize]
                    .copy_from_slice(&self.bufs[i][at..at + w as usize]);
            }
        }
        true
    }

    pub fn whole(&mut self, app: &App) -> bool {
        let Some((i, _)) = self.acquire() else { return false };
        app.draw_whole(&mut self.bufs[i]);
        self.primed = self.present(i, Damage { x: 0, y: 0, width: W, height: H });
        self.primed
    }

    /// A whole frame, as `paint_changed` paints it for the app's ask and
    /// for a frame painted again after one not shown: drawn whole,
    /// presented as the rows it changed from the buffer shown, one band or
    /// the runs inside it, and given back unpresented when it changed none.
    pub fn changed(&mut self, app: &App) -> bool {
        let Some((i, _)) = self.acquire() else { return false };
        app.draw_whole(&mut self.bufs[i]);
        let (next, shown) = (&self.bufs[i], &self.bufs[self.shown]);
        match present_rows(next, shown, (W as usize, H as usize), (W, H)) {
            Shown::Nothing => self.chain.cancel(i).is_ok(),
            Shown::Band(d) => self.present(i, d),
            Shown::Parts(d, parts, n) => self.present_parts(i, d, &parts[..n]),
        }
    }

    /// A frame of an app whose damage is `r`: drawn whole when the buffer is
    /// out of date beyond `r`, as `paint_partial` judges it; else the app
    /// paints `r` alone (`alone`) or its whole content, and the corners are
    /// finished where it drew.
    pub fn partial(&mut self, app: &App, r: Rect, alone: bool) -> bool {
        let (c, r) = (content(), clip(r));
        let d = Damage { x: c.x + r.x, y: c.y + r.y, width: r.w, height: r.h };
        let Some((i, stale)) = self.acquire() else { return false };
        let px = &mut self.bufs[i];
        if whole_needed(stale, (d.x, d.y, d.width, d.height)) {
            app.draw_whole(px);
        } else {
            let drawn = match alone {
                true => {
                    app.paint_rect(px, r);
                    (d.x, d.y, d.x + d.width, d.y + d.height)
                }
                false => {
                    app.paint_content(px);
                    (c.x, c.y, c.x + c.w, c.y + c.h)
                }
            };
            let fb = &mut PaintBuffer { pixels: px, stride_words: W, width: W, height: H };
            finish_band(fb, false, drawn);
        }
        if r.w == 0 || r.h == 0 {
            return self.chain.cancel(i).is_ok();
        }
        self.present(i, d)
    }
}
