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

//! Reading what firmware left lit, without writing a register.

use crate::engine::pipe::{Pipe, Transcoder};
use crate::engine::regs;
use crate::engine::takeover::{read_output, read_outputs};
use crate::model::firmware::T1080;
use crate::model::Gen9;

/// eDP on pipe A (eDP Input Select 100b), polarities +H +V.
const EDP_ON_A: u32 = (4 << 12) | (3 << 16);

#[test]
fn an_edp_panel_on_pipe_a_is_output_0_with_its_timing() {
    let mut m = Gen9::new(64);
    m.light(Transcoder::Edp, Pipe::A, T1080, EDP_ON_A);
    let o = read_output(&mut m, Pipe::A).unwrap();
    assert_eq!(o.transcoder, Transcoder::Edp);
    assert_eq!((o.width, o.height), (1920, 1080));
    let t = o.timing;
    assert_eq!((t.h_active, t.h_front, t.h_sync, t.h_total()), (1920, 88, 44, 2200));
    assert_eq!((t.v_active, t.v_front, t.v_sync, t.v_total()), (1080, 4, 5, 1125));
    assert!(t.h_positive && t.v_positive);
    assert!(o.plane.enabled() && o.plane.flippable());
    assert_eq!((o.plane.stride, o.plane.width, o.plane.height), (7680, 1920, 1080));
    assert!(m.writes.is_empty(), "a takeover reads only");
}

#[test]
fn edp_on_pipe_b_leaves_pipe_a_to_its_own_transcoder() {
    let mut m = Gen9::new(64);
    m.light(Transcoder::Edp, Pipe::B, T1080, 5 << 12);
    m.light(Transcoder::A, Pipe::A, T1080, 0);
    let outs = read_outputs(&mut m);
    assert_eq!(outs[0].unwrap().transcoder, Transcoder::A);
    assert_eq!(outs[1].unwrap().transcoder, Transcoder::Edp);
    assert!(outs[2].is_none());
}

#[test]
fn a_stopped_transcoder_or_a_dark_plane_is_no_output() {
    let mut m = Gen9::new(64);
    m.light(Transcoder::A, Pipe::A, T1080, 0);
    m.set(regs::trans_conf(Transcoder::A), 1 << 31);
    assert!(read_output(&mut m, Pipe::A).is_none(), "enable asked, state off");
    let mut m = Gen9::new(64);
    m.light(Transcoder::A, Pipe::A, T1080, 0);
    m.set(crate::engine::plane_regs::plane_ctl(Pipe::A, 1), 0);
    assert!(read_output(&mut m, Pipe::A).is_none());
}
