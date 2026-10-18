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

//! A buffer of a new size gets a surface of that size (wayland/reshape.rs,
//! wayland/scene_pixels.rs), and its damage is the whole new rect. The
//! surface kept the first buffer's shape: a smaller buffer was read at the
//! old stride with stale pixels round it, a larger one was never shown.

use crate::reshape::{reshape, Reshape};
use crate::scene_pixels::Pixels;
use crate::window_damage::window_damage;

const QWEN: (u32, u32, u32) = (880, 560, 880 * 4);
const FULL: (u32, u32, u32) = (1920, 1022, 1920 * 4);

#[test]
fn every_other_shape_gets_a_surface_of_its_own() {
    assert_eq!(reshape(None, QWEN), Reshape::Open);
    assert_eq!(reshape(Some(QWEN), QWEN), Reshape::Same);
    assert_eq!(reshape(Some(QWEN), FULL), Reshape::Rehome, "larger");
    assert_eq!(reshape(Some(FULL), QWEN), Reshape::Rehome, "smaller");
    assert_eq!(reshape(Some(QWEN), (880, 560, 896 * 4)), Reshape::Rehome, "stride");
}

#[test]
fn after_a_resize_the_damage_is_the_whole_new_rect() {
    let at = Some((0, 58));
    let (x, y, w, h) = window_damage(at, FULL.0, FULL.1).expect("placed");
    assert_eq!((x, y, w, h), (0, 58, 1920, 1022));
    assert_eq!(y + h, 1080, "rows at the bottom edge left stale");
    let back = window_damage(Some((520, 284)), QWEN.0, QWEN.1).expect("placed");
    assert_eq!(back, (520, 284, 880, 560));
}

#[test]
fn new_pixels_are_page_aligned_and_hold_the_frame() {
    let bytes = (FULL.2 * FULL.1) as usize;
    let mut p = Pixels::with_room(bytes).expect("room");
    let (at, len) = p.span();
    assert_eq!(at % 4096, 0, "the kernel registers only a page-aligned surface");
    assert!(len >= bytes as u64);
    assert_eq!(p.frame(bytes).map(|f| f.len()), Some(bytes));
    assert!(p.frame(len as usize + 1).is_none(), "a frame past the room");
    let mut none = Pixels::empty();
    assert!(none.frame(1).is_none());
}
