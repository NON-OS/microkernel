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
//! Each reading bring-up takes to tell its own dark plane (G-79) refuses
//! alone when it is not as this driver sets it.

use super::dark_tests::{own_dark, taken};
use super::kit::TOTAL;
use crate::engine::pipe::{Pipe, Transcoder};
use crate::engine::plane_read::ENABLE;
use crate::engine::plane_regs as r;
use crate::engine::regs;
use crate::model::Gen9;

/// A way the pipe or plane differs from what this driver sets.
type Spoil = (&'static str, fn(&mut Gen9));

#[test]
fn each_reading_refuses_alone() {
    let cases: [Spoil; 7] = [
        ("the pipe not its timing's size", |m| m.set(regs::pipe_src(Pipe::A), (1023 << 16) | 767)),
        ("a layout never programmed", |m| m.set(r::plane_ctl(Pipe::A, 1), m.get(r::plane_ctl(Pipe::A, 1)) | 1 << 10)),
        ("the plane on", |m| m.set(r::plane_ctl(Pipe::A, 1), m.get(r::plane_ctl(Pipe::A, 1)) | ENABLE)),
        ("the transcoder stopped", stopped),
        ("past the table's end", |m| m.set(r::plane_surflive(Pipe::A, 1), (TOTAL - 64) * 4096)),
        ("below the upper half", |m| m.set(r::plane_surflive(Pipe::A, 1), (TOTAL / 2 - 64) * 4096)),
        ("no surface at all", |m| m.set(r::plane_stride(Pipe::A, 1), 0)),
    ];
    for (why, spoil) in cases {
        let mut m = own_dark();
        assert!(taken(&mut m), "{why}: taken before");
        spoil(&mut m);
        assert!(!taken(&mut m), "{why}: refused");
    }
    let mut m = own_dark();
    m.set(r::plane_surflive(Pipe::A, 1), (TOTAL / 2) * 4096);
    assert!(taken(&mut m), "the upper half's first entry is in it");
}

/// The eDP transcoder stopped with its timing still there, pipe A's own
/// transcoder holding the same timing but not running either.
fn stopped(m: &mut Gen9) {
    for reg in [regs::trans_htotal, regs::trans_hsync, regs::trans_vtotal, regs::trans_vsync] {
        m.set(reg(Transcoder::A), m.get(reg(Transcoder::Edp)));
    }
    m.set(regs::trans_conf(Transcoder::Edp), 0);
}
