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

//! Faults under a taken-over output, found and put right, mode untouched.

use super::scanout_tests::{setup, TOTAL};
use crate::engine::error::Error;
use crate::engine::fault::{check, de_pipe_iir, Fault, UNDERRUN};
use crate::engine::gen::{GEN7_5, GEN9};
use crate::engine::pipe::{Pipe, Transcoder};
use crate::engine::scanout::Scanout;
use crate::engine::{plane_regs as r, regs, takeover::read_output};
use crate::model::{Gen9, Polls};

const EDP: Transcoder = Transcoder::Edp;

fn running() -> (Gen9, Scanout) {
    let (mut m, a, b) = setup();
    let out = read_output(&mut m, &GEN9, Pipe::A).unwrap();
    let s = Scanout::start(&mut m, &out, TOTAL, [&a, &b], &Polls::new(50)).unwrap();
    (m, s)
}

#[test]
fn a_running_output_has_no_fault_and_an_underrun_is_told_once() {
    let (mut m, s) = running();
    let front = s.surfaces[s.front];
    assert_eq!(check(&mut m, &GEN9, Pipe::A, EDP, front), None);
    let iir = de_pipe_iir(&GEN9, Pipe::A).unwrap();
    m.set(iir, UNDERRUN | 1);
    assert_eq!(check(&mut m, &GEN9, Pipe::A, EDP, front), Some(Fault::Underrun));
    assert_eq!(m.get(iir), 1, "only the underrun bit cleared");
    assert_eq!(check(&mut m, &GEN9, Pipe::A, EDP, front), None);
    assert_eq!(m.pixel(Pipe::A, 0, 0), Some(0xFF00_00AA), "the picture stands");
    assert_eq!(de_pipe_iir(&GEN7_5, Pipe::A), None, "Haswell's is elsewhere");
    m.set(iir, UNDERRUN);
    assert_eq!(check(&mut m, &GEN7_5, Pipe::A, EDP, front), None);
}

#[test]
fn a_hung_flip_is_put_back_on_the_front_surface() {
    let (mut m, s) = running();
    let front = s.surfaces[s.front];
    m.set(r::plane_surf(Pipe::A, 1), 0);
    m.set(r::plane_surflive(Pipe::A, 1), 0);
    assert_eq!(check(&mut m, &GEN9, Pipe::A, EDP, front), Some(Fault::Hung));
    s.recover(&mut m, EDP, &Polls::new(50)).unwrap();
    assert_eq!(check(&mut m, &GEN9, Pipe::A, EDP, front), None);
    assert_eq!(m.pixel(Pipe::A, 1919, 1079), Some(0xFF00_00AA));
}

#[test]
fn a_stopped_transcoder_is_found_first_and_left_to_a_mode_set() {
    let (mut m, s) = running();
    let front = s.surfaces[s.front];
    m.set(regs::trans_conf(EDP), 1 << 31);
    m.set(de_pipe_iir(&GEN9, Pipe::A).unwrap(), UNDERRUN);
    assert_eq!(check(&mut m, &GEN9, Pipe::A, EDP, front), Some(Fault::Stopped));
    let writes = m.writes.len();
    assert_eq!(s.recover(&mut m, EDP, &Polls::new(50)), Err(Error::Stopped));
    assert_eq!(m.writes.len(), writes, "nothing written");
}
