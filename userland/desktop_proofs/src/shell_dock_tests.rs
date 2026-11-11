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
//! The dock's tiles light and press only in the box a click on them names,
//! and draw over the dock's panel by blending, never punching through it.

use nonos_toolkit::icons::IconId;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{Pointer, Rect};

use crate::render::layout::*;
use crate::shell_dock_view::{draw, hit, lit, row, Tile};
use crate::shell_layer::layer;

const LAYER: Rect = Rect { x: 0, y: 0, w: 1920, h: 260 };
const T: Tile =
    Tile { icon: IconId::Terminal, open: false, active: false, pulsing: false, off: false };
const TILES: [Tile; 4] =
    [T, Tile { open: true, ..T }, Tile { active: true, ..T }, Tile { off: true, ..T }];

fn boxes() -> Vec<Rect> {
    let d = bottom_dock_rect(LAYER.w, LAYER.h);
    let (top, h) = (d.y + dock_box_inset(), d.h - 2 * dock_box_inset());
    row(d.x + dock_pad(), top, taskbar_entry_w(), h, dock_stride(), TILES.len())
}

fn painted(under: u32, p: &Pointer) -> Vec<u32> {
    let (w, h) = (LAYER.w, LAYER.h);
    let mut words = vec![under; (w * h) as usize];
    draw(&mut unsafe { layer(words.as_mut_ptr() as u64, w * 4, w, h) }, &TILES, &boxes(), 21, p);
    words
}

fn moved_only_in(under: u32, p: &Pointer, tile: usize) -> bool {
    let (plain, lit, b) = (painted(under, &Pointer::new()), painted(under, p), boxes());
    let mut moved = (0..plain.len()).filter(|&i| plain[i] != lit[i]).peekable();
    let at = |i: usize| ((i as u32 % LAYER.w) as i32, (i as u32 / LAYER.w) as i32);
    moved.peek().is_some() && moved.all(|i| hit(&b, at(i).0, at(i).1) == Some(tile))
}

pub(crate) fn dock_tiles_light_where_they_are_clicked(tag: &str) {
    let (b, under) = (boxes(), roles().raised.argb());
    let mid = |r: Rect| ((r.x + r.w / 2) as i32, (r.y + r.h / 2) as i32);
    for (tile, other) in [(0, 2), (1, 0), (2, 1)] {
        let ((x, y), (ox, oy), mut p) = (mid(b[tile]), mid(b[other]), Pointer::new());
        p.moved(x, y);
        assert_eq!(lit(&b, &p), (Some(tile), None), "{tag}: hover {tile}");
        assert!(moved_only_in(under, &p, tile), "{tag}: hover {tile} off its box");
        p.pressed(x, y);
        assert_eq!(lit(&b, &p), (Some(tile), Some(tile)), "{tag}: press {tile}");
        assert!(moved_only_in(under, &p, tile), "{tag}: press {tile} off its box");
        p.moved(ox, oy);
        assert_eq!(lit(&b, &p), (Some(other), None), "{tag}: dragged off {tile}");
        p.moved(x, b[tile].y as i32 - 1);
        assert_eq!(lit(&b, &p), (None, None), "{tag}: left the dock");
        assert_eq!(painted(under, &p), painted(under, &Pointer::new()), "{tag}: left lit");
    }
    let mut p = Pointer::new();
    p.moved(mid(b[0]).0, mid(b[0]).1);
    let panel = (0xC0 << 24) | (under & 0x00FF_FFFF);
    let thinnest = painted(panel, &p).iter().map(|w| w >> 24).min();
    assert_eq!(thinnest, Some(0xC0), "{tag}: a tile punched through the dock");
}
