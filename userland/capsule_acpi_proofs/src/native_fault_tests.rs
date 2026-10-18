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

//! A power fault and the retention latch: a fault takes out what was
//! probed and cuts power, and the slot stays off until its card is out;
//! an open latch makes a card absent, keeps an empty slot unpowered and
//! never powered on; the latch counts only where there is a sensor.

use super::native_reads::{rd, OUT, UP};
use crate::native::read::{MRL_OPEN, MRL_SENSOR};
use crate::native::regs::{POWER_CONTROLLER, POWER_FAULT, POWER_OFF, PRESENT};
use crate::native::slot::{Act, Phase, Read, Slot};
use crate::native::times::*;

const FAULT: Read = rd(PRESENT | POWER_FAULT, Some(true), true);

#[test]
fn a_fault_departs_cuts_power_and_holds_until_the_card_is_out() {
    let mut s = Slot::new(true, true, UP);
    assert_eq!((s.step(FAULT, 0), s.phase), (Act::Fault, Phase::Off));
    assert_eq!(s.step(UP, 10), Act::Nothing, "off while the card stays");
    assert_eq!(s.step(FAULT, 20), Act::Nothing, "a fault while off changes nothing");
    assert_eq!((s.step(OUT, 30), s.phase), (Act::Nothing, Phase::Empty));
    let mut c = Slot::new(true, true, OUT);
    c.step(UP, 0);
    assert_eq!((c.step(FAULT, 10), c.phase), (Act::PowerOff, Phase::Off), "nothing probed yet");
    let mut n = Slot::new(true, false, OUT);
    n.step(UP, 0);
    assert_eq!(n.step(FAULT, 10), Act::Nothing, "no controller: nothing to switch");
}

#[test]
fn an_open_latch_makes_the_card_absent_and_the_slot_unpowered() {
    let open = |powered| Read { latch_open: true, ..rd(PRESENT, Some(true), powered) };
    let mut s = Slot::new(true, true, UP);
    s.step(open(true), 0);
    assert_eq!((s.step(open(true), DEBOUNCE_MS), s.phase), (Act::Depart, Phase::Empty));
    assert_eq!(s.step(open(true), DEBOUNCE_MS + 10), Act::PowerOff, "unpowered while open");
    assert_eq!(s.step(open(false), DEBOUNCE_MS + 20), Act::Nothing, "never powered on open");
    let closed = rd(PRESENT, Some(false), false);
    assert_eq!(s.step(closed, DEBOUNCE_MS + 30), Act::PowerOn, "closed again: power on");
}

#[test]
fn the_latch_counts_only_with_a_sensor_and_power_only_with_a_controller() {
    let status = PRESENT | MRL_OPEN;
    assert!(Read::from_regs(status, MRL_SENSOR, None, 0).latch_open);
    assert!(!Read::from_regs(status, 0, None, 0).latch_open, "no sensor: the bit means nothing");
    assert!(!Read::from_regs(status, MRL_SENSOR, None, 0).here());
    assert!(Read::from_regs(PRESENT, 0, None, POWER_OFF).powered, "no controller: always on");
    assert!(!Read::from_regs(PRESENT, POWER_CONTROLLER, None, POWER_OFF).powered);
    assert_eq!(Read::from_regs(PRESENT, 0, Some(1 << 13), 0).link, Some(true));
    assert_eq!(Read::from_regs(PRESENT, 0, Some(0), 0).link, Some(false));
}
