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

//! F-103: an EC that lies. One that stalls is given up on after three
//! bounded waits and stays off; bytes it left over are read away before a
//! command, so its answers do not shift; and an output buffer that never
//! empties is a bounded failure.

use crate::ec::{read, EcError, Health, Off, STRIKES};
use crate::ec_model::{EcModel, PORTS};

#[test]
fn a_stalled_ec_is_given_up_on_after_three_bounded_waits() {
    let (mut ec, mut h) = (EcModel::new(0), Health::default());
    ec.stuck = true;
    let mut tries = 0;
    while h.usable() {
        h.note(&read(&PORTS, &mut ec, 0x10));
        tries += 1;
    }
    assert_eq!((tries, h.off), (STRIKES, Some(Off::Stalled)));
    assert_eq!(ec.slept_ms, 1000 * u64::from(STRIKES), "a second a wait, no more");
    assert!(!h.note(&read(&PORTS, &mut ec, 0x10)), "turned off once, said once");
}

#[test]
fn an_answer_clears_the_strikes_and_a_refused_port_is_none() {
    let mut h = Health::default();
    for _ in 0..10 {
        h.note::<()>(&Err(EcError::Timeout));
        h.note(&Ok(()));
        h.note::<()>(&Err(EcError::Io));
    }
    assert!(h.usable(), "a slow but answering EC stays on");
}

#[test]
fn stale_bytes_are_read_away_and_a_stuck_buffer_is_bounded() {
    let mut ec = EcModel::new(1);
    ec.mem[0x40] = 0x5A;
    ec.tricks.stale.extend([0xEE, 0xDD, 0xCC]);
    assert_eq!(read(&PORTS, &mut ec, 0x40), Ok(0x5A), "not the stale 0xEE");
    let mut stuck = EcModel::new(0);
    stuck.tricks.obf_stuck = true;
    assert_eq!(read(&PORTS, &mut stuck, 0x40), Err(EcError::Timeout));
    assert_eq!(stuck.slept_ms, 0, "sixteen reads, no waiting");
}
