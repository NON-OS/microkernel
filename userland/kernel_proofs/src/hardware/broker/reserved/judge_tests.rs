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

//! A grant runs into a reservation only by sharing a byte of the same
//! space with it; a device's own BAR wholly inside one passes, and one
//! that straddles a reservation's edge does not. Coverage joins ranges
//! that abut.

use super::judge::{conflict, covered};
use super::wire::{Reservation, IO, MEMORY};

fn r(space: u8, base: u64, len: u64) -> Reservation {
    Reservation { space, base, len }
}

fn board() -> Vec<Reservation> {
    vec![r(MEMORY, 0xE000_0000, 0x1000_0000), r(MEMORY, 0xFED1_0000, 0x8000), r(IO, 0x400, 0x80)]
}

#[test]
fn overlaps_are_refused_and_neighbours_pass() {
    let b = board();
    assert_eq!(conflict(&b, MEMORY, 0xDFFF_F000, 0x2000, false), Some(b[0]));
    assert_eq!(conflict(&b, MEMORY, 0xEFFF_FFFF, 1, false), Some(b[0]));
    assert_eq!(conflict(&b, MEMORY, 0xDFFF_F000, 0x1000, false), None, "abuts below");
    assert_eq!(conflict(&b, MEMORY, 0xF000_0000, 0x1000, false), None, "abuts above");
    assert_eq!(conflict(&b, IO, 0x47F, 2, false), Some(b[2]));
    assert_eq!(conflict(&b, IO, 0x480, 8, false), None);
}

#[test]
fn spaces_do_not_mix() {
    let b = board();
    assert_eq!(conflict(&b, IO, 0xE000_0000, 0x10, false), None);
    assert_eq!(conflict(&b, MEMORY, 0x400, 0x10, false), None);
}

#[test]
fn a_bar_inside_a_reservation_is_the_device_firmware_placed() {
    let b = board();
    assert_eq!(conflict(&b, MEMORY, 0xFED1_4000, 0x1000, true), None);
    assert_eq!(conflict(&b, MEMORY, 0xFED1_4000, 0x1000, false), Some(b[1]), "a buffer is not");
    assert_eq!(conflict(&b, MEMORY, 0xFED1_7000, 0x2000, true), Some(b[1]), "straddles");
    assert_eq!(conflict(&b, IO, 0x400, 0x80, true), None, "exactly the reservation");
}

#[test]
fn empty_lists_and_empty_or_wrapping_requests() {
    assert_eq!(conflict(&[], MEMORY, 0, u64::MAX, false), None);
    assert_eq!(conflict(&board(), MEMORY, 0xE000_0000, 0, false), None);
    assert!(conflict(&board(), MEMORY, u64::MAX - 0xFFF, 0x2000, false).is_none());
    assert!(conflict(&board(), MEMORY, 0xD000_0000, u64::MAX, false).is_some(), "wraps");
}

#[test]
fn coverage_joins_abutting_and_overlapping_ranges() {
    let ecam = |rs: &[Reservation]| covered(rs, MEMORY, 0xE000_0000, 0x1000_0000);
    assert!(ecam(&board()));
    let halves = [r(MEMORY, 0xE800_0000, 0x0800_0000), r(MEMORY, 0xE000_0000, 0x0800_0000)];
    assert!(ecam(&halves), "two halves, listed out of order");
    let gap = [r(MEMORY, 0xE000_0000, 0x0800_0000), r(MEMORY, 0xE800_1000, 0x0800_0000)];
    assert!(!ecam(&gap));
    assert!(!ecam(&[r(IO, 0xE000_0000, 0x1000_0000)]), "wrong space");
    assert!(!ecam(&[]));
    let over = [r(MEMORY, 0xE000_0000, 0x0900_0000), r(MEMORY, 0xE800_0000, 0x0800_0000)];
    assert!(ecam(&over));
}
