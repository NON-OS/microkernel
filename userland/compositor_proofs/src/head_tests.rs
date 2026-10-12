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

//! A mirrored output's poll: a moved serial or a stale FLUSH asks SURFACE
//! again, LOST repaints, silence waits; and how many heads an output count
//! asks for.

use nonos_gfx_wire::body::state::{LOST, SCANNING_OUT};
use nonos_gfx_wire::State;

use crate::display::head::Head;
use crate::display::head_rule::{head_step, heads_wanted, HeadStep};
use crate::sw_blitter::Surface;

fn head(stale: bool) -> Head {
    Head { output: 1, serial: 0x0028_0002, handle: 9, screen: Surface::default(), stale }
}

fn state(serial: u32, flags: u32) -> Option<State> {
    Some(State { serial, flags, ..State::default() })
}

#[test]
fn a_head_still_showing_its_primary_is_kept() {
    assert_eq!(head_step(&head(false), state(0x0028_0002, SCANNING_OUT)), HeadStep::Keep);
    assert_eq!(head_step(&head(false), None), HeadStep::Keep, "no answer this time");
}

#[test]
fn a_new_serial_or_a_stale_flush_asks_for_the_primary_again() {
    let s = state(0x0028_0003, SCANNING_OUT);
    assert_eq!(head_step(&head(false), s), HeadStep::Resurface);
    assert_eq!(head_step(&head(true), state(0x0028_0002, SCANNING_OUT)), HeadStep::Resurface);
    assert_eq!(head_step(&head(true), None), HeadStep::Resurface);
}

#[test]
fn a_lost_head_is_repainted() {
    let s = state(0x0028_0002, SCANNING_OUT | LOST);
    assert_eq!(head_step(&head(false), s), HeadStep::Repaint);
}

#[test]
fn every_output_past_the_first_is_a_head_up_to_the_servers_four() {
    assert_eq!((heads_wanted(0), heads_wanted(1), heads_wanted(2)), (0, 0, 1));
    assert_eq!((heads_wanted(4), heads_wanted(9)), (3, 3));
}
