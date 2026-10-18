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

//! Waiting on a codec that does not answer, without holding the core.
//!
//! A verb is given 100 ms. The wait used to spin for all of it, and the jack
//! poll sends a verb twice a second, so a codec that stopped answering held
//! a core for a fifth of every second and starved the period refill and the
//! request loop. A working codec answers in microseconds, inside the short
//! spin; past that the wait sleeps between polls, over the ring and over the
//! Immediate Command interface alike.

use std::time::Instant;

use crate::clock::sleeps;
use crate::constants::{ICIS, ICIS_ICB};
use crate::controller::immediate::get_parameter;
use crate::controller::verb::{Link, VERB_MS};
use crate::error::HdaError;
use crate::model::{rings, window};
use crate::regs::Regs;

#[test]
fn an_unanswered_verb_sleeps_through_its_deadline_instead_of_spinning() {
    let bar = window();
    let (corb, rirb) = rings(0);
    let mut link = Link::new(Regs::new(bar.base()), corb.base(), rirb.base(), 256);
    let (before, t) = (sleeps(), Instant::now());
    assert_eq!(link.send(0x000f_0000), Err(HdaError::VerbTimeout));
    let slept = sleeps() - before;
    assert!(t.elapsed().as_millis() as u64 >= VERB_MS, "gave up before its deadline");
    // Every poll past the first millisecond sleeps one: a hundred on an idle
    // host, a few on a loaded one whose sleeps run long, none at all for a
    // wait that spins.
    assert!(slept >= 1, "slept {slept} times in a {VERB_MS} ms wait; spun the rest");
}

#[test]
fn an_immediate_command_left_busy_sleeps_through_its_deadline_too() {
    let bar = window();
    bar.present16(ICIS as usize, ICIS_ICB);
    let (before, t) = (sleeps(), Instant::now());
    assert_eq!(get_parameter(Regs::new(bar.base()), 0, 0), None);
    let slept = sleeps() - before;
    assert!(t.elapsed().as_millis() as u64 >= VERB_MS, "gave up before its deadline");
    assert!(slept >= 1, "slept {slept} times in a {VERB_MS} ms wait; spun the rest");
}
