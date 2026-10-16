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

//! A slot path by path: arrival, a glitch, a link that never comes,
//! departure, and power for a card in an unpowered slot.

pub use super::native_reads::{rd, OUT, PRESS, UP};
use super::native_reads::DOWN;
use crate::native::regs::PRESENT;
use crate::native::slot::{Act, Phase, Read, Slot};
use crate::native::times::*;

pub fn slot(r: Read) -> Slot {
    Slot::new(true, true, r)
}

#[test]
fn a_card_is_probed_once_up_for_ready_ms() {
    let mut s = slot(OUT);
    assert_eq!(s.step(UP, 0), Act::Nothing);
    assert_eq!(s.step(UP, READY_MS - 1), Act::Nothing);
    assert_eq!((s.step(UP, READY_MS), s.phase), (Act::Probe, Phase::Occupied));
    assert_eq!(s.step(UP, READY_MS + 1), Act::Nothing, "probed once");
    let mut g = slot(OUT);
    g.step(UP, 0);
    assert_eq!((g.step(OUT, 50), g.phase), (Act::Nothing, Phase::Empty), "a glitch probes nothing");
}

#[test]
fn a_link_that_never_comes_is_let_go() {
    let mut s = slot(OUT);
    s.step(DOWN, 0);
    assert_eq!(s.step(DOWN, LINK_UP_MS - 1), Act::Nothing);
    assert_eq!((s.step(DOWN, LINK_UP_MS), s.phase), (Act::PowerOff, Phase::Off));
    assert_eq!(s.step(UP, LINK_UP_MS + 500), Act::Nothing, "off until taken out");
    assert_eq!((s.step(OUT, LINK_UP_MS + 600), s.phase), (Act::Nothing, Phase::Empty));
    let mut n = Slot::new(true, false, OUT);
    n.step(DOWN, 0);
    assert_eq!(n.step(DOWN, LINK_UP_MS), Act::Nothing, "no power controller: never switched");
}

#[test]
fn a_card_departs_once_its_absence_has_held() {
    let mut s = slot(UP);
    assert_eq!(s.phase, Phase::Occupied);
    s.step(OUT, 0);
    assert_eq!((s.step(UP, 50), s.phase), (Act::Nothing, Phase::Occupied), "a glitch keeps it");
    s.step(OUT, 100);
    assert_eq!(s.step(OUT, 100 + DEBOUNCE_MS - 1), Act::Nothing);
    assert_eq!((s.step(OUT, 100 + DEBOUNCE_MS), s.phase), (Act::Depart, Phase::Empty));
    assert_eq!(slot(UP).step(DOWN, 0), Act::Nothing, "link down starts the same wait");
}

#[test]
fn power_comes_on_for_a_card_in_an_unpowered_slot() {
    let off = rd(PRESENT, Some(false), false);
    assert_eq!(slot(OUT).step(off, 0), Act::PowerOn);
    assert_eq!(Slot::new(true, false, OUT).step(off, 0), Act::Nothing);
}
