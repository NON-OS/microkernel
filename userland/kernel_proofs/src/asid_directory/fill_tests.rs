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

use super::read::complete;
use super::table::{FULL, HIGH, SLOTS};
use super::tests::BASE;
use super::write::{publish, withdraw};
use super::TURN;
use std::sync::atomic::Ordering::{Acquire, Release};

#[test]
fn a_withdrawn_slot_is_filled_again() {
    let _turn = TURN.lock().unwrap_or_else(|e| e.into_inner());
    let high = HIGH.load(Acquire);
    for round in 0..(4 * SLOTS as u32) {
        publish(BASE + 1000 + round, round, u64::from(round) << 12);
        withdraw(BASE + 1000 + round);
    }
    let after = HIGH.load(Acquire);
    assert!(after <= high.max(1), "slots leak: high {high} became {after}");
}

#[test]
fn a_full_directory_says_a_miss_proves_nothing() {
    let _turn = TURN.lock().unwrap_or_else(|e| e.into_inner());
    let asid = |i: usize| BASE + 0x10_0000 + i as u32;
    (0..=SLOTS).for_each(|i| publish(asid(i), 9, 0x1000));
    assert!(!complete(), "an address space found no slot and nobody knows");
    (0..=SLOTS).for_each(|i| withdraw(asid(i)));
    // The kernel never clears it; the test does, so the others start clean.
    FULL.store(false, Release);
}
