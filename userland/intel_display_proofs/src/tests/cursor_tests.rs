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

//! The cursor plane: position encoding, arming by CUR_BASE, hiding.

use crate::engine::cursor::{position, set, MODE_64_ARGB};
use crate::engine::error::Error;
use crate::engine::pipe::Pipe;
use crate::engine::plane_regs as r;
use crate::model::Gen9;

#[test]
fn positions_are_sign_and_magnitude() {
    assert_eq!(position(10, 20), (20 << 16) | 10);
    assert_eq!(position(-3, 5), (5 << 16) | (1 << 15) | 3);
    assert_eq!(position(7, -8), (1 << 31) | (8 << 16) | 7);
    assert_eq!(position(i32::MIN, i32::MAX), (0xFFF << 16) | (1 << 15) | 0xFFF, "clamped");
}

#[test]
fn a_move_takes_effect_only_with_the_base_write_and_the_blank() {
    let mut m = Gen9::new(1);
    set(&mut m, Pipe::B, Some(0x7000), 100, 50).unwrap();
    assert_eq!(m.live_reg(r::cur_ctl(Pipe::B)), 0, "armed, not latched");
    m.vblank();
    assert_eq!(m.live_reg(r::cur_ctl(Pipe::B)) & 0x3F, MODE_64_ARGB);
    assert_eq!(m.live_reg(r::cur_pos(Pipe::B)), position(100, 50));
    assert_eq!(m.live_reg(r::cur_base(Pipe::B)), 0x7000);
    m.writes.clear();
    set(&mut m, Pipe::B, Some(0x7000), 101, 50).unwrap();
    assert_eq!(m.writes.last().unwrap().0, r::cur_base(Pipe::B), "the base arms it");
}

#[test]
fn hiding_keeps_the_other_ctl_bits_and_still_arms() {
    let mut m = Gen9::new(1);
    m.set(r::cur_ctl(Pipe::A), (1 << 24) | MODE_64_ARGB);
    m.set(r::cur_base(Pipe::A), 0x9000);
    set(&mut m, Pipe::A, None, 0, 0).unwrap();
    m.vblank();
    assert_eq!(m.live_reg(r::cur_ctl(Pipe::A)), 1 << 24);
    assert_eq!(m.live_reg(r::cur_base(Pipe::A)), 0x9000);
}

#[test]
fn an_unaligned_cursor_image_is_refused() {
    let mut m = Gen9::new(1);
    assert_eq!(set(&mut m, Pipe::A, Some(0x7010), 0, 0), Err(Error::Argument));
    assert!(m.writes.is_empty());
}
