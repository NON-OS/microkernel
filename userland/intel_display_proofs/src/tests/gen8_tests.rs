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

//! The takeover on Broadwell's primary plane: DSPCNTR, a byte stride, PIPESRC.

use super::scanout_tests::{setup, TOTAL};
use crate::engine::gen::{PlaneKind, GEN8};
use crate::engine::pipe::Pipe;
use crate::engine::{plane::program, plane_regs as r, scanout::Scanout, takeover::read_output};
use crate::model::{Gen9, Polls};

/// Firmware's plane: BGRX 8:8:8:8 (0110b at 29:26), 7680 bytes, no size.
fn broadwell(m: &mut Gen9) {
    m.plane = PlaneKind::Primary;
    m.set(r::plane_ctl(Pipe::A, 1), (1 << 31) | (0x6 << 26));
    m.set(r::plane_stride(Pipe::A, 1), 7680);
    m.set(r::plane_size(Pipe::A, 1), 0);
}

#[test]
fn the_plane_reads_as_pipesrc_with_a_byte_stride() {
    let (mut m, ..) = setup();
    broadwell(&mut m);
    let p = read_output(&mut m, &GEN8, Pipe::A).unwrap().plane;
    assert_eq!((p.stride, p.width, p.height, p.flippable()), (7680, 1920, 1080, true));
    m.set(r::plane_ctl(Pipe::A, 1), (1 << 31) | (0xE << 26));
    assert!(read_output(&mut m, &GEN8, Pipe::A).unwrap().plane.flippable(), "RGBX too");
    m.set(r::plane_ctl(Pipe::A, 1), (1 << 31) | (0x6 << 26) | (1 << 10));
    assert!(!read_output(&mut m, &GEN8, Pipe::A).unwrap().plane.flippable(), "X tiled");
}

#[test]
fn programming_writes_bytes_and_never_the_size_register() {
    let mut m = Gen9::new(64);
    let old = (1 << 31) | (0xE << 26) | (1 << 15) | (1 << 30);
    program(&mut m, PlaneKind::Primary, Pipe::A, 0x4_0000, 7680, (1920, 1080), old).unwrap();
    assert_eq!(m.get(r::plane_stride(Pipe::A, 1)), 7680);
    assert_eq!(m.get(r::plane_ctl(Pipe::A, 1)), (1 << 31) | (0x6 << 26) | (1 << 30));
    let sized = |o: u32| o == r::plane_size(Pipe::A, 1) || o == r::plane_pos(Pipe::A, 1);
    assert!(!m.writes.iter().any(|w| sized(w.0)), "DSPCNTR's plane has neither");
    let wide = program(&mut m, PlaneKind::Primary, Pipe::A, 0x4_0000, 0x1_0000, (1920, 1080), 0);
    assert!(wide.is_err(), "past DSPSTRIDE's 15:6");
}

#[test]
fn start_present_and_stop_on_a_broadwell_plane() {
    let (mut m, a, b) = setup();
    broadwell(&mut m);
    let out = read_output(&mut m, &GEN8, Pipe::A).unwrap();
    assert_eq!(m.pixel(Pipe::A, 1919, 1079), Some(0xFF00_0001), "firmware's picture");
    let mut s = Scanout::start(&mut m, &out, TOTAL, [&a, &b], &Polls::new(50)).unwrap();
    assert_eq!(m.pixel(Pipe::A, 1919, 1079), Some(0xFF00_00AA));
    s.present(&mut m, &Polls::new(50)).unwrap();
    assert_eq!(m.pixel(Pipe::A, 0, 0), Some(0xFF00_00BB));
    s.stop(&mut m, TOTAL, &Polls::new(50)).unwrap();
    assert_eq!(m.pixel(Pipe::A, 5, 5), Some(0xFF00_0001), "firmware's picture back");
    let back = (m.get(r::plane_stride(Pipe::A, 1)), m.get(r::plane_size(Pipe::A, 1)));
    assert_eq!(back, (7680, 0), "firmware's stride, in bytes, and no size");
}
