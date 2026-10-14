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

//! The set of addresses a client of driver.i2c_pci0 may reach, built from
//! what the firmware declared and setup bound, against every byte a client
//! can put in the address field.

use crate::transaction::Reach;

/// `addrs` as declarations naming the usual descriptor register.
fn of(addrs: impl IntoIterator<Item = u8>) -> Reach {
    Reach::declared(addrs.into_iter().map(|a| (a, 0x0001)))
}

fn reached(r: &Reach) -> Vec<u8> {
    (0..=u8::MAX).filter(|&a| r.allows(a)).collect()
}

#[test]
fn with_nothing_bound_or_declared_nothing_is_in_reach() {
    assert!(reached(&Reach::default()).is_empty());
    assert!(reached(&of([])).is_empty());
    assert!(reached(&of([0])).is_empty(), "zero means no device was bound");
}

#[test]
fn a_bound_device_is_the_only_address_in_reach() {
    assert_eq!(reached(&of([0x15])), [0x15]);
    assert_eq!(reached(&of([0x7F])), [0x7F]);
}

#[test]
fn the_general_call_and_wider_than_seven_bits_never_count() {
    let r = of([0x00, 0x80, 0x95, 0xFF, 0x2C]);
    assert_eq!(reached(&r), [0x2C], "0x95 is not 0x15 with a stray bit");
}

#[test]
fn repeats_count_once_and_the_set_holds_at_most_eight() {
    assert_eq!(reached(&of([0x15, 0x15, 0x2C, 0x15])), [0x15, 0x2C]);
    let many = of(1..=40);
    assert_eq!(reached(&many), (1..=8).collect::<Vec<u8>>(), "the first eight declared");
}
