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

//! STATE on the taken-over engine: a healthy output says nothing new, a
//! hung flip or an underrun is put right and told as LOST, a stopped
//! transcoder is told as LOST and not scanning out.

use nonos_gfx_serve::Display;
use nonos_gfx_wire::body::state::{LOST, SCANNING_OUT};

use super::wire_rig::rig;
use crate::engine::fault::{de_pipe_iir, UNDERRUN};
use crate::engine::gen::GEN9;
use crate::engine::pipe::{Pipe, Transcoder};
use crate::engine::{plane_regs as r, regs};

#[test]
fn a_healthy_output_scans_out_and_its_generation_holds() {
    let (mut d, _) = rig();
    let s = d.state(0).unwrap();
    assert_eq!((s.width, s.height, s.flags), (1920, 1080, SCANNING_OUT));
    assert_eq!(d.state(0).unwrap().generation, s.generation);
}

#[test]
fn a_hung_flip_is_put_right_and_told_as_lost() {
    let (mut d, model) = rig();
    let before = d.state(0).unwrap().generation;
    model.borrow_mut().set(r::plane_surf(Pipe::A, 1), 0);
    model.borrow_mut().set(r::plane_surflive(Pipe::A, 1), 0);
    let s = d.state(0).unwrap();
    assert_eq!(s.flags, SCANNING_OUT | LOST);
    assert!(s.generation != before);
    assert_eq!(model.borrow().pixel(Pipe::A, 9, 9), Some(0xFF00_00AA), "front shown again");
    assert_eq!(d.state(0).unwrap().flags, SCANNING_OUT, "and then healthy");
}

#[test]
fn an_underrun_is_told_once() {
    let (mut d, model) = rig();
    model.borrow_mut().set(de_pipe_iir(&GEN9, Pipe::A).unwrap(), UNDERRUN);
    assert_eq!(d.state(0).unwrap().flags, SCANNING_OUT | LOST);
    assert_eq!(d.state(0).unwrap().flags, SCANNING_OUT);
}

#[test]
fn a_stopped_transcoder_is_lost_and_not_scanning_out() {
    let (mut d, model) = rig();
    model.borrow_mut().set(regs::trans_conf(Transcoder::Edp), 1 << 31);
    let (a, b) = (d.state(0).unwrap(), d.state(0).unwrap());
    assert_eq!((a.flags, b.flags), (LOST, LOST));
    assert!(b.generation != a.generation, "each answer asks for a repaint");
}
