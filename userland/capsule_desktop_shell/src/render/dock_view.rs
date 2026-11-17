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
//! The dock's app tiles as shared `IconButton`s, lit by the shell's
//! `Pointer`. One row of boxes serves the paint, the light and the click, so
//! a tile lights and launches exactly where it is drawn.

use alloc::vec::Vec;

use nonos_toolkit::icons::IconId;
use nonos_toolkit::widgets::rect::contains;
use nonos_toolkit::widgets::{IconButton, Kind, Pointer, Rect, State, Surface};

/// What the dock knows about one app when it draws its tile.
#[derive(Clone, Copy)]
pub struct Tile {
    pub icon: IconId,
    pub open: bool,
    pub active: bool,
    pub pulsing: bool,
    pub off: bool,
}

/// `n` boxes of `w` by `h` from `left`, `stride` apart, all at `top`.
pub fn row(left: u32, top: u32, w: u32, h: u32, stride: u32, n: usize) -> Vec<Rect> {
    (0..n as u32).map(|i| Rect { x: left + i * stride, y: top, w, h }).collect()
}

/// The tile whose box holds the point.
pub fn hit(boxes: &[Rect], x: i32, y: i32) -> Option<usize> {
    boxes.iter().position(|b| contains(*b, x, y))
}

/// The tile the pointer is over, and the one it holds pressed.
pub fn lit(boxes: &[Rect], pointer: &Pointer) -> (Option<usize>, Option<usize>) {
    let Some(i) = pointer.position().and_then(|(x, y)| hit(boxes, x, y)) else {
        return (None, None);
    };
    let s = pointer.state(|x, y| hit(boxes, x, y) == Some(i));
    (s.hovered.then_some(i), s.pressed.then_some(i))
}

/// The focused app's tile is `Primary`, an open or opening one `Secondary`,
/// the rest `Plain` on the dock. An opening app's tile reads as held, and an
/// app turned off reads as unavailable.
pub fn button(t: &Tile, side: u32, hovered: bool, pressed: bool) -> IconButton<'static> {
    let kind = match (t.active, t.open || t.pulsing) {
        (true, _) => Kind::Primary,
        (false, true) => Kind::Secondary,
        (false, false) => Kind::Plain,
    };
    let state =
        State { hovered, pressed: pressed || t.pulsing, disabled: t.off, ..State::default() };
    IconButton::new(t.icon).icon_size(side).kind(kind).state(state)
}

/// Every tile in its box, with the one under the pointer lit.
pub fn draw(fb: &mut impl Surface, tiles: &[Tile], boxes: &[Rect], side: u32, p: &Pointer) {
    let (hovered, pressed) = lit(boxes, p);
    for (i, (t, at)) in tiles.iter().zip(boxes).enumerate() {
        button(t, side, hovered == Some(i), pressed == Some(i)).draw(fb, *at);
    }
}
