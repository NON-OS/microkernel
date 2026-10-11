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

//! Programming the primary plane: nothing changes on screen until the
//! blank after PLANE_SURF, and then everything changes at once.

use crate::engine::ggtt;
use crate::engine::pipe::{Pipe, Transcoder};
use crate::engine::plane::program;
use crate::model::firmware::T1080;
use crate::model::Gen9;

pub(super) const OURS: u32 = 0x4000_0000;

pub(super) fn lit() -> Gen9 {
    let mut m = Gen9::new(0x80000);
    m.light(Transcoder::A, Pipe::A, T1080, 0);
    m.fill(0x10_0000, 1, 0xFF11_2233);
    let first = [0x10_0000];
    for i in 0..2025 {
        ggtt::map(&mut m, i, &first, 0x80000).unwrap();
    }
    m.fill(0x20_0000, 1, 0xFF44_5566);
    let ours = [0x20_0000u64; 2025];
    ggtt::map(&mut m, OURS / 4096, &ours, 0x80000).unwrap();
    m
}

#[test]
fn the_new_surface_appears_whole_at_the_next_blank() {
    let mut m = lit();
    assert_eq!(m.pixel(Pipe::A, 5, 5), Some(0xFF11_2233));
    program(&mut m, Pipe::A, OURS, 7680, (1920, 1080), (1 << 31) | (4 << 24) | (1 << 20)).unwrap();
    assert_eq!(m.pixel(Pipe::A, 5, 5), Some(0xFF11_2233), "armed, not latched");
    m.vblank();
    assert_eq!(m.pixel(Pipe::A, 5, 5), Some(0xFF44_5566));
    assert_eq!(m.pixel(Pipe::A, 1919, 1079), Some(0xFF44_5566));
    let ctl = m.live_reg(crate::engine::plane_regs::plane_ctl(Pipe::A, 1));
    assert_eq!(ctl & (1 << 20), 0, "B in the low byte, ARGB8888's order");
}

#[test]
fn plane_surf_is_written_last() {
    let mut m = lit();
    m.writes.clear();
    program(&mut m, Pipe::A, OURS, 7680, (1920, 1080), 1 << 31).unwrap();
    let last = m.writes.last().unwrap().0;
    assert_eq!(last, crate::engine::plane_regs::plane_surf(Pipe::A, 1));
}
