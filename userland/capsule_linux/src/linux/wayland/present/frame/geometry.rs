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

//! Where a guest's content sits inside its frame, and how large the frame
//! is. Pure, so the host proofs hold it.

use nonos_toolkit::decorations::{content_rect_at, edges_at, Edges};

/// How far the frame's edges reach in over a client's content, in logical
/// pixels (Around::edges).
pub const OVER_CONTENT: u32 = 4;

/// OVER_CONTENT in drawing pixels at `quarters` quarter pixels a logical one.
fn over_content(quarters: u32) -> u32 {
    (OVER_CONTENT.saturating_mul(quarters) / 4).max(1)
}

/// A size the toolkit's frame is measured at: its margins do not depend on
/// the window's size, only on the display's scale.
const PROBE: u32 = 1024;

/// A window `w` by `h` with the content at (`cx`, `cy`) inside it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Around {
    pub w: u32,
    pub h: u32,
    pub cx: u32,
    pub cy: u32,
    /// Drawn as a maximised window's frame: no margin round it.
    pub max: bool,
}

impl Around {
    /// No frame: the window is the content.
    pub const fn bare(w: u32, h: u32) -> Around {
        Around { w, h, cx: 0, cy: 0, max: false }
    }

    pub fn framed(&self) -> bool {
        self.cx != 0 || self.cy != 0
    }

    /// The rows a buffer showing this window is laid in, width, height and
    /// stride in bytes: the frame's own when framed, the content's `shape`
    /// when bare. A window's buffers are drawn in only at the layout they
    /// were registered at; drawn at another, the compositor reads each row
    /// at the old stride and the picture shears. A frame that changed
    /// shape (a maximise acked, full screen) takes new buffers.
    pub fn laid(&self, shape: (u32, u32, u32)) -> (u32, u32, u32) {
        match self.framed() {
            true => (self.w, self.h, self.w.saturating_mul(4)),
            false => shape,
        }
    }

    /// The frame's edges a press at (`x`, `y`) on the window grabs, when it
    /// grabs any. Off the content they are the toolkit's bands. Over the
    /// content's outer pixels an edge counts only within OVER_CONTENT of the
    /// content's own edge, as just inside a native window's border, so a
    /// press there resizes and the client keeps the rest, its scroll bars
    /// included. None on a bare or maximised window, or off it.
    pub fn edges(&self, x: i32, y: i32, (cw, ch): (u32, u32), quarters: u32) -> Option<Edges> {
        if !self.framed() || x < 0 || y < 0 {
            return None;
        }
        let e = edges_at(self.w, self.h, self.max, x as u32, y as u32, quarters);
        let e = match self.on_content(x, y, cw, ch) {
            Some((ix, iy)) => {
                let (ix, iy, band) = (ix as u32, iy as u32, over_content(quarters));
                Edges {
                    left: e.left && ix < band,
                    right: e.right && ix.saturating_add(band) >= cw,
                    top: e.top && iy < band,
                    bottom: e.bottom && iy.saturating_add(band) >= ch,
                }
            }
            None => e,
        };
        Some(e).filter(|e| e.any())
    }

    /// The edges a press at (`x`, `y`) grabs on a window that draws its own
    /// frame: the band OVER_CONTENT in from each side, as just inside a
    /// native window's border. Its client's own resize margin was its
    /// shadow, which the window is cut to its geometry without, so no press
    /// reached it. None past the band, off the window, or on a framed one.
    pub fn inner_edges(&self, x: i32, y: i32, quarters: u32) -> Option<Edges> {
        if self.framed() || x < 0 || y < 0 {
            return None;
        }
        let (x, y, band) = (x as u32, y as u32, over_content(quarters));
        if x >= self.w || y >= self.h {
            return None;
        }
        let e = Edges {
            left: x < band,
            right: x.saturating_add(band) >= self.w,
            top: y < band,
            bottom: y.saturating_add(band) >= self.h,
        };
        Some(e).filter(|e| e.any())
    }

    /// Whether (`x`, `y`) on the window is on the content, and where on it.
    pub fn on_content(&self, x: i32, y: i32, cw: u32, ch: u32) -> Option<(i32, i32)> {
        let (x, y) = (x - self.cx as i32, y - self.cy as i32);
        (x >= 0 && y >= 0 && (x as u32) < cw && (y as u32) < ch).then_some((x, y))
    }
}

/// The frame on screen now, of `next` (the frame of the mode a configure
/// acked asks for) and `was` (the frame of the mode the window is in), round
/// content of `shape`, with the shown buffers laid at `laid`: `next` once a
/// buffer drawn for it is shown, `was` until then. Between a maximise acked
/// and its buffer the old picture stays on screen in the old frame, and
/// what the pointer hits, a redraw of the title bar and a drag's rect are
/// of that frame, not of one not drawn yet. Neither laid so: `next`.
pub fn on_screen(
    next: Around,
    was: Around,
    shape: (u32, u32, u32),
    laid: (u32, u32, u32),
) -> Around {
    if next.laid(shape) != laid && was.laid(shape) == laid {
        return was;
    }
    next
}

/// The frame around content `w` by `h` at `quarters` of display scale: none
/// when `fills` (full screen, or a guest that draws its own), and without
/// the toolkit's margin round it when `max`.
pub fn around_at(w: u32, h: u32, quarters: u32, fills: bool, max: bool) -> Around {
    if fills {
        return Around::bare(w, h);
    }
    let c = content_rect_at(PROBE, PROBE, max, quarters);
    Around { w: w + (PROBE - c.w), h: h + (PROBE - c.h), cx: c.x, cy: c.y, max }
}
