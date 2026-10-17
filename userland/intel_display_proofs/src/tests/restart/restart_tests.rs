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
//! A driver restarted onto its own leftovers, proved through the
//! takeover's own `settle_heads`: a flip cut off, its cursor on and stray
//! entries all go. The GGTT is rewritten whole, so every present entry is
//! the new driver's or firmware's; nothing the engine reaches is the killed
//! driver's; the plane shows the new surface; and a mode set cut off part
//! way leaves an output taken whole at its new size.

use super::kit::{restart, TOTAL};
use super::leftovers::leftovers;
use crate::engine::ggtt::map;
use crate::engine::ggtt_pte::Pte;
use crate::engine::pipe::{Pipe, Transcoder};
use crate::engine::plane_regs as r;
use crate::engine::regs;
use crate::model::confine::names;
use crate::model::firmware::T1080;
use crate::model::Gen9;
use crate::tests::confine::kit::{confined, FIRMWARE};

/// Firmware's picture on pipe A.
pub fn base() -> Gen9 {
    let mut m = Gen9::new(TOTAL as usize);
    m.light(Transcoder::Edp, Pipe::A, T1080, 4 << 12);
    m.fill(0x9000_0000, 1, 0xFF00_0001);
    map(&mut m, Pte::Wide, 0, &[0x9000_0000; 2025], TOTAL).unwrap();
    m
}

/// Everything the engine reaches is held by the new driver or firmware's.
pub fn reaches_only_ours(m: &Gen9, tag: &str) {
    let held = m.confine.held.clone().unwrap_or_default();
    let bad: Vec<_> = m.reaching().into_iter().filter(|&a| !names(&held, a | 1)).collect();
    let bad: Vec<_> = bad.into_iter().filter(|&a| !names(&[FIRMWARE], a | 1)).collect();
    assert!(bad.is_empty(), "{tag}: reaches {:x?}", &bad[..bad.len().min(3)]);
}

#[test]
fn a_flip_cut_off_a_cursor_on_and_strays_all_go() {
    let mut m = base();
    leftovers(&mut m, Pipe::A, TOTAL, (1920, 1080));
    assert!(m.reaching().iter().any(|&a| a >= 0x3000_0000), "the killed driver's, reached");
    let (_d, model) = restart(m).expect("lit");
    let m = model.borrow();
    confined(&m, "restart");
    reaches_only_ours(&m, "restart");
    assert_eq!(m.pixel(Pipe::A, 5, 5), Some(0xFF00_00AA), "the new surface shows");
    assert_eq!(m.live_reg(r::cur_ctl(Pipe::A)) & 0x3F, 0, "its cursor off");
}

#[test]
fn a_mode_set_cut_off_part_way_is_taken_whole() {
    let mut m = base();
    leftovers(&mut m, Pipe::A, TOTAL, (1920, 1080));
    m.set(regs::pipe_src(Pipe::A), (1023 << 16) | 767);
    let (_d, model) = restart(m).expect("lit");
    let m = model.borrow();
    confined(&m, "cut off");
    reaches_only_ours(&m, "cut off");
    assert_eq!(m.pixel(Pipe::A, 1023, 767), Some(0xFF00_00AA), "at the pipe's size");
}
