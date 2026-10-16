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

//! The turn OP_ACPI_HID takes over the reach, against what a hostile or
//! broken firmware table makes of it: no declaration, more than the reach
//! holds, one address declared over and over, an ask count at its end, and
//! asks after the reach has narrowed.

use core::cell::Cell;

use crate::discover::MAX_TARGETS;
use crate::transaction::Reach;

const DESC: u16 = 0x0001;

fn named(r: &Reach, asks: &Cell<usize>, times: usize) -> Vec<Option<u8>> {
    (0..times).map(|_| r.named(asks).map(|(a, _)| a)).collect()
}

#[test]
fn with_no_declaration_in_reach_nothing_is_named_however_often_asked() {
    let asks = Cell::new(0);
    assert_eq!(named(&Reach::declared(std::iter::empty()), &asks, 4), [None; 4]);
    let unreachable = Reach::declared([(0, DESC), (0x80, DESC), (0xFF, DESC)]);
    assert_eq!(named(&unreachable, &asks, 3), [None; 3]);
}

#[test]
fn of_sixty_four_declarations_the_first_eight_are_named_in_turn_and_no_other() {
    let asks = Cell::new(0);
    let first: Vec<_> = (1..=MAX_TARGETS as u8).map(Some).collect();
    let many = Reach::declared((1..=64).map(|a| (a, DESC)));
    assert_eq!(named(&many, &asks, 2 * MAX_TARGETS), [first.clone(), first].concat());
    let same = Reach::declared([(0x2C, 0x0020); 64]);
    assert_eq!(named(&same, &asks, 3), [Some(0x2C); 3]);
    assert_eq!(same.named(&asks), Some((0x2C, 0x0020)));
}

#[test]
fn an_ask_count_at_its_end_wraps_without_a_fault() {
    let r = Reach::declared([(0x15, DESC), (0x2C, DESC), (0x10, DESC)]);
    let asks = Cell::new(usize::MAX);
    assert!(r.named(&asks).is_some());
    assert_eq!((asks.get(), r.named(&asks)), (0, Some((0x15, DESC))));
}

#[test]
fn an_ask_after_the_reach_narrowed_names_the_pad_alone() {
    let wide = Reach::declared([(0x15, DESC), (0x2C, 0x0020), (0x10, DESC)]);
    let asks = Cell::new(0);
    assert_eq!(wide.named(&asks), Some((0x15, DESC)));
    let descriptor = [30, 0, 0x00, 0x01];
    let narrowed = wide.after_read(0x2C, &0x0020u16.to_le_bytes(), &descriptor);
    assert_eq!(named(&narrowed, &asks, 5), [Some(0x2C); 5]);
    assert_eq!(narrowed.named(&asks), Some((0x2C, 0x0020)), "named with its own register");
}
