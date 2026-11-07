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

//! Where a popup goes: placed against an anchor rectangle on its parent, as
//! a Wayland xdg_positioner places it, and kept on the output by the
//! adjustments its client allows (flip, then slide, then resize, per axis).

use super::rect::Rect;

/// xdg_positioner.anchor and .gravity: 0 none, 1 top, 2 bottom, 3 left,
/// 4 right, 5 top left, 6 bottom left, 7 top right, 8 bottom right.
pub const EDGE_MAX: u32 = 8;
/// xdg_positioner.constraint_adjustment bits.
pub const SLIDE_X: u32 = 1;
pub const SLIDE_Y: u32 = 2;
pub const FLIP_X: u32 = 4;
pub const FLIP_Y: u32 = 8;
pub const RESIZE_X: u32 = 16;
pub const RESIZE_Y: u32 = 32;
pub const ADJUST_ALL: u32 = 63;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Positioner {
    /// The anchor rectangle, relative to the parent window's origin.
    pub anchor_x: i32,
    pub anchor_y: i32,
    pub anchor_w: u32,
    pub anchor_h: u32,
    pub width: u32,
    pub height: u32,
    pub anchor: u32,
    pub gravity: u32,
    pub adjust: u32,
    pub offset_x: i32,
    pub offset_y: i32,
}

/// The popup's place on the output, and relative to its parent's origin.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Placed {
    pub rect: Rect,
    pub rel_x: i32,
    pub rel_y: i32,
}

/// Which side of a point, along one axis.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Side {
    Before,
    Middle,
    After,
}

impl Side {
    fn flipped(self) -> Side {
        match self {
            Side::Before => Side::After,
            Side::After => Side::Before,
            Side::Middle => Side::Middle,
        }
    }
}

fn horizontal(edge: u32) -> Side {
    match edge {
        3 | 5 | 6 => Side::Before,
        4 | 7 | 8 => Side::After,
        _ => Side::Middle,
    }
}

fn vertical(edge: u32) -> Side {
    match edge {
        1 | 5 | 7 => Side::Before,
        2 | 6 | 8 => Side::After,
        _ => Side::Middle,
    }
}

/// One axis of the placement: the anchor span, the sides, the offset, the
/// popup's length, and the output's span.
#[derive(Clone, Copy)]
struct Axis {
    start: i64,
    span: i64,
    anchor: Side,
    gravity: Side,
    offset: i64,
    len: i64,
    lo: i64,
    hi: i64,
}

impl Axis {
    fn at(&self) -> i64 {
        let point = match self.anchor {
            Side::Before => self.start,
            Side::Middle => self.start + self.span / 2,
            Side::After => self.start + self.span,
        };
        let from = match self.gravity {
            Side::Before => point - self.len,
            Side::Middle => point - self.len / 2,
            Side::After => point,
        };
        from + self.offset
    }

    fn off(&self, at: i64, len: i64) -> bool {
        at < self.lo || at + len > self.hi
    }

    fn flipped(&self) -> Axis {
        Axis {
            anchor: self.anchor.flipped(),
            gravity: self.gravity.flipped(),
            offset: -self.offset,
            ..*self
        }
    }

    // The start and length on this axis after the adjustments allowed: a
    // flip is kept only when it is no longer off the output; a slide moves
    // the popup in from the edge it crossed; a resize cuts it to the output.
    // What is still off after those is slid, and cut, onto the output: a
    // popup nobody could see, or one drawn at a negative place, helps no one.
    fn settle(&self, flip: bool, slide: bool, resize: bool) -> (i64, i64) {
        let (mut at, mut len) = (self.at(), self.len);
        if flip && self.off(at, len) {
            let other = self.flipped().at();
            if !self.off(other, len) {
                at = other;
            }
        }
        if slide && self.off(at, len) {
            at = at.min(self.hi - len).max(self.lo);
        }
        if resize && self.off(at, len) {
            let (a, b) = (at.max(self.lo), (at + len).min(self.hi));
            if b > a {
                (at, len) = (a, b - a);
            }
        }
        len = len.min(self.hi - self.lo).max(1);
        at = at.min(self.hi - len).max(self.lo);
        (at, len)
    }
}

/// Place `p` against `parent` on an output of `bounds`.
pub fn place(p: &Positioner, parent: Rect, bounds: Rect) -> Placed {
    let span = |lo: u32, len: u32| (i64::from(lo), i64::from(lo) + i64::from(len));
    let (x_lo, x_hi) = span(bounds.x, bounds.width.max(1));
    let (y_lo, y_hi) = span(bounds.y, bounds.height.max(1));
    let x = Axis {
        start: i64::from(parent.x) + i64::from(p.anchor_x),
        span: i64::from(p.anchor_w),
        anchor: horizontal(p.anchor),
        gravity: horizontal(p.gravity),
        offset: i64::from(p.offset_x),
        len: i64::from(p.width.max(1)),
        lo: x_lo,
        hi: x_hi,
    };
    let y = Axis {
        start: i64::from(parent.y) + i64::from(p.anchor_y),
        span: i64::from(p.anchor_h),
        anchor: vertical(p.anchor),
        gravity: vertical(p.gravity),
        offset: i64::from(p.offset_y),
        len: i64::from(p.height.max(1)),
        lo: y_lo,
        hi: y_hi,
    };
    let a = p.adjust;
    let (px, w) = x.settle(a & FLIP_X != 0, a & SLIDE_X != 0, a & RESIZE_X != 0);
    let (py, h) = y.settle(a & FLIP_Y != 0, a & SLIDE_Y != 0, a & RESIZE_Y != 0);
    let narrow = |v: i64| u32::try_from(v.max(0)).unwrap_or(u32::MAX);
    let rel = |v: i64| i32::try_from(v).unwrap_or(if v < 0 { i32::MIN } else { i32::MAX });
    Placed {
        rect: Rect { x: narrow(px), y: narrow(py), width: narrow(w), height: narrow(h) },
        rel_x: rel(px - i64::from(parent.x)),
        rel_y: rel(py - i64::from(parent.y)),
    }
}
