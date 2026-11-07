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
//! A window resized from its edges, by the frame's border or by
//! xdg_toplevel.resize from a client that draws its own: held at a press,
//! and at the release the guest is told the new size in a configure, with
//! the corner the window goes to once the buffer drawn for it comes, as a
//! window coming back from full screen is (place.rs origin). One configure
//! for the whole drag, as a native window commits its resize at the release,
//! so the guest draws once. Pure, so the host proofs hold it.

use super::held::Held;
use super::mode::{Configure, Mode, Rect};
use super::state::Fit;

/// The least content a resize leaves a window.
pub const MIN_CONTENT: (u32, u32) = (64, 48);

/// The edges a resize holds.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Sides {
    pub left: bool,
    pub right: bool,
    pub top: bool,
    pub bottom: bool,
}

impl Sides {
    pub fn any(self) -> bool {
        self.left || self.right || self.top || self.bottom
    }

    /// xdg_toplevel.resize_edge: top 1, bottom 2, left 4, right 8, and the
    /// corners as their sums; a value naming two opposite edges, or no edge,
    /// holds none.
    pub fn of_edge(edge: u32) -> Sides {
        let s = Sides {
            top: edge & 1 != 0,
            bottom: edge & 2 != 0,
            left: edge & 4 != 0,
            right: edge & 8 != 0,
        };
        let opposed = (s.top && s.bottom) || (s.left && s.right) || edge > 15;
        if opposed {
            Sides::default()
        } else {
            s
        }
    }
}

/// A resize held: its edges, where the pointer was pressed on the window,
/// the window's rect on screen then, frame and all, and the least and most
/// content the client set (set_min_size, set_max_size), in shown pixels,
/// 0 where it set none.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Resize {
    pub sides: Sides,
    pub press: (i32, i32),
    pub base: Rect,
    pub limits: ((u32, u32), (u32, u32)),
}

/// The configure's rect for `r` with the pointer at `at` on the window, as
/// it was at the press: the window's new corner on screen and its content's
/// size, the frame `frame` wide and high round it. The moved edges follow
/// the pointer, the others stay; the window keeps at least MIN_CONTENT and
/// within the client's own limits, on the screen `screen` and below the
/// menubar's foot `top`. A client drawing a size of its own other than the
/// one told keeps the edges the resize did not move where they were: its
/// buffer is placed from them (held.rs), not at the told corner.
pub fn sized(r: &Resize, at: (i32, i32), frame: (u32, u32), top: u32, screen: (u32, u32)) -> Rect {
    let (bx, by, bw, bh) = (r.base.0 as i64, r.base.1 as i64, r.base.2 as i64, r.base.3 as i64);
    let dx = i64::from(at.0) - i64::from(r.press.0);
    let dy = i64::from(at.1) - i64::from(r.press.1);
    let ((min_w, min_h), (max_w, max_h)) = r.limits;
    let span = |f: u32, min: u32, max: u32, least: u32| {
        let lo = i64::from(f) + i64::from(min.max(least));
        let hi = if max == 0 { i64::MAX } else { i64::from(f) + i64::from(max) };
        (lo, hi.max(lo))
    };
    let span_w = span(frame.0, min_w, max_w, MIN_CONTENT.0);
    let span_h = span(frame.1, min_h, max_h, MIN_CONTENT.1);
    let (sw, sh) = (i64::from(screen.0), i64::from(screen.1));
    let (x, w) = side(r.sides.left, r.sides.right, (bx, bw), dx, (0, sw), span_w);
    let (y, h) = side(r.sides.top, r.sides.bottom, (by, bh), dy, (i64::from(top), sh), span_h);
    let content = |v: i64, f: u32| (v - i64::from(f)).clamp(1, i64::from(u32::MAX)) as u32;
    let at = |v: i64| v.clamp(0, i64::from(u32::MAX)) as u32;
    (at(x), at(y), content(w, frame.0), content(h, frame.1))
}

/// One axis: the start and length of a window from `start` `len` long, its
/// low edge moved by `d` when `low`, its high edge when `high`, within
/// `lo..hi` where it fits, and never under `least` nor over `most` long. A
/// window dragged past `hi` and then resized from its low edge ends at
/// `hi`, not past it.
fn side(
    low: bool,
    high: bool,
    (start, len): (i64, i64),
    d: i64,
    (lo, hi): (i64, i64),
    (least, most): (i64, i64),
) -> (i64, i64) {
    let end = start + len;
    if low {
        let room = end - lo.min(start);
        let n = (len - d).min(most).min(room).max(least);
        let x = end - n;
        return (x, n.min((hi - x).max(least)));
    }
    if high {
        let room = hi - start;
        return (start, (len + d).min(most).min(room).max(least));
    }
    (start, len)
}

impl Fit {
    /// The guest asked to take `rect`, a resize's (sized), holding `held`:
    /// the configure to send under `serial`, or None when the window is not
    /// in its own size or is on its way out of it, where a resize has no
    /// meaning.
    pub fn resize(&mut self, rect: Rect, held: Held, serial: u32) -> Option<Configure> {
        if self.mode != Mode::Normal || self.heading() != Mode::Normal {
            return None;
        }
        let told = Configure { serial, mode: Mode::Normal, rect: Some(rect) };
        self.sent = Some(told);
        self.sent_from = serial;
        self.sent_held = Some(held);
        self.last = Some(told);
        Some(told)
    }
}
