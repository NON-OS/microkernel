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
//! A dark plane on a running transcoder: this driver's own, left by
//! a mode set killed after the plane went off, is taken whole and
//! confined; firmware's stays dark, as G-12 says.

use super::kit::{restart, TOTAL};
use super::leftovers::leftovers;
use super::restart_tests::{base, reaches_only_ours};
use crate::engine::access::Mmio;
use crate::engine::gen::GEN9;
use crate::engine::pipe::Pipe;
use crate::engine::plane_read::ENABLE;
use crate::engine::plane_regs as r;
use crate::engine::takeover::read_output;
use crate::engine::takeover_dark::dark_leftover;
use crate::model::Gen9;
use crate::tests::confine::kit::confined;

pub(super) fn plane_off(m: &mut Gen9) {
    let (ctl, surf) = (m.get(r::plane_ctl(Pipe::A, 1)), m.get(r::plane_surf(Pipe::A, 1)));
    m.write(r::plane_ctl(Pipe::A, 1), ctl & !ENABLE);
    m.write(r::plane_surf(Pipe::A, 1), surf);
    m.vblank();
}

/// Pipe A as a mode set killed after the plane went off leaves it.
pub(super) fn own_dark() -> Gen9 {
    let mut m = base();
    leftovers(&mut m, Pipe::A, TOTAL, (1920, 1080));
    plane_off(&mut m);
    m
}

pub(super) fn taken(m: &mut Gen9) -> bool {
    dark_leftover(m, &GEN9, Pipe::A, TOTAL).is_some()
}

#[test]
fn its_own_dark_plane_is_taken_whole() {
    let mut m = own_dark();
    assert!(read_output(&mut m, &GEN9, Pipe::A).is_none(), "no output, as G-12 reads it");
    assert!(taken(&mut m));
    let (_d, model) = restart(m).expect("taken");
    let m = model.borrow();
    confined(&m, "own dark");
    reaches_only_ours(&m, "own dark");
    assert_eq!(m.pixel(Pipe::A, 1919, 1079), Some(0xFF00_00AA));
}

#[test]
fn firmware_s_dark_plane_stays_dark() {
    let mut m = base();
    plane_off(&mut m);
    assert!(!taken(&mut m));
    assert!(restart(m).is_none(), "nothing taken");
}
