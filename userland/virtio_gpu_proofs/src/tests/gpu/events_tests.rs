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
//! The host's display events: a raised display bit has STATE ask the
//! display info again, take the host's new size as the hint, clear the bit
//! and move the generation. An info the host drops leaves the bit raised.

use nonos_gfx_wire::{ops, State};

use super::ask::ask;
use super::rig::rig;
use super::wrong::{run, Wrong};

use super::kit::EVENTS_READ;
use crate::constants::GPU_CFG_EVENTS_CLEAR;
use crate::tests::model::DEVICE_AT;

/// The device configuration's events-clear word, in the modern window.
const EVENTS_CLEAR: usize = DEVICE_AT + GPU_CFG_EVENTS_CLEAR;

#[test]
fn a_display_event_brings_the_host_s_new_size_as_the_hint() {
    let mut r = rig();
    let before = State::decode(&ask(&mut r, ops::STATE, 0, &[]).1).unwrap();
    assert_eq!(r.regs.wrote32(EVENTS_CLEAR), 0, "nothing raised, nothing cleared");
    r.host.borrow_mut().displays[0] = (1280, 800);
    r.regs.present32(EVENTS_READ, 1);
    let lost = run(&mut r, Wrong::Dead(1, 1), |r| ask(r, ops::STATE, 0, &[]));
    let st = State::decode(&lost.unwrap().1).unwrap();
    assert_eq!((st.generation, st.hint_width), (before.generation, 640), "info dropped");
    assert_eq!(r.regs.wrote32(EVENTS_CLEAR), 0, "the bit left raised");
    let st = State::decode(&ask(&mut r, ops::STATE, 0, &[]).1).unwrap();
    assert_eq!((st.hint_width, st.hint_height), (1280, 800));
    assert_eq!(st.generation, before.generation.wrapping_add(1));
    assert_eq!((st.width, st.height), (640, 480), "the mode itself unchanged");
    assert_eq!(r.regs.wrote32(EVENTS_CLEAR), 1, "the display bit cleared");
}
