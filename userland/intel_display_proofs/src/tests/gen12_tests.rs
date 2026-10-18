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

//! Gen11 and Gen12 takeovers: the wider format and stride fields, pipe D,
//! and no eDP transcoder from Gen12.

use crate::engine::gen::{PlaneKind, GEN11, GEN12, GEN12_3, GEN9};
use crate::engine::pipe::{Pipe, Transcoder};
use crate::engine::{plane::program, plane_regs as r, takeover::read_outputs};
use crate::model::firmware::T1080;
use crate::model::Gen9;

#[test]
fn pipe_d_is_output_3_from_gen12_and_never_read_before() {
    let mut m = Gen9::new(64);
    m.light(Transcoder::D, Pipe::D, T1080, 0);
    let o = read_outputs(&mut m, &GEN12)[3].unwrap();
    assert_eq!((o.transcoder, o.width, o.height), (Transcoder::D, 1920, 1080));
    assert!(read_outputs(&mut m, &GEN9)[3].is_none());
    assert!(read_outputs(&mut m, &GEN12_3)[3].is_none(), "Rocket Lake has three");
    assert_eq!(Pipe::from_output(&GEN12, 3), Some(Pipe::D));
    assert_eq!(Pipe::from_output(&GEN11, 3), None);
}

#[test]
fn gen12_feeds_each_pipe_from_its_own_transcoder() {
    let mut m = Gen9::new(64);
    m.light(Transcoder::Edp, Pipe::A, T1080, 4 << 12);
    assert_eq!(read_outputs(&mut m, &GEN11)[0].unwrap().transcoder, Transcoder::Edp);
    assert!(read_outputs(&mut m, &GEN12)[0].is_none(), "transcoder A is dark");
    m.light(Transcoder::A, Pipe::A, T1080, 0);
    assert_eq!(read_outputs(&mut m, &GEN12)[0].unwrap().transcoder, Transcoder::A);
}

#[test]
fn gen11s_format_field_reaches_bit_23() {
    let mut m = Gen9::new(64);
    m.light(Transcoder::A, Pipe::A, T1080, 0);
    m.set(r::plane_ctl(Pipe::A, 1), (1 << 31) | (0x4 << 24) | (1 << 23));
    assert!(read_outputs(&mut m, &GEN9)[0].unwrap().plane.flippable(), "a Gen9 CTL bit");
    assert!(!read_outputs(&mut m, &GEN11)[0].unwrap().plane.flippable(), "01001b");
    let old = (1 << 31) | (1 << 23) | (1 << 20) | (1 << 30);
    program(&mut m, PlaneKind::Universal11, Pipe::A, 0x4_0000, 7680, (1920, 1080), old).unwrap();
    assert_eq!(m.get(r::plane_ctl(Pipe::A, 1)), (1 << 31) | (0x8 << 23) | (1 << 30));
    assert_eq!(m.get(r::plane_stride(Pipe::A, 1)), 120);
}

#[test]
fn gen11s_stride_field_is_eleven_bits() {
    assert_eq!(PlaneKind::Universal.stride_bytes(0x400), 0);
    assert_eq!(PlaneKind::Universal11.stride_bytes(0x400), 0x1_0000);
    assert_eq!(PlaneKind::Universal.max_stride(), 1023 * 64);
    assert_eq!(PlaneKind::Universal11.max_stride(), 2047 * 64);
    assert_eq!(PlaneKind::Primary.max_stride(), 0xFFC0);
    let mut m = Gen9::new(64);
    let wide = |m: &mut Gen9, k| program(m, k, Pipe::A, 0x4_0000, 1024 * 64, (8192, 64), 0);
    assert!(wide(&mut m, PlaneKind::Universal).is_err());
    assert!(wide(&mut m, PlaneKind::Universal11).is_ok());
}
