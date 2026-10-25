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

//! A held device whose group cannot be read fails closed: a PCI
//! device another capsule holds, whose path no longer resolves, refuses a
//! claim beside it as admit refuses one of its own; a held device off the
//! PCI tree (ACPI, platform, VMBus) has no group and refuses nothing.

use super::acs_refusal::unreadable_line;
use super::member::Member;
use super::port_only::{blocked_by, Blocked};

const AT: (u8, u8, u8) = (2, 0, 0);

#[test]
fn a_held_pci_device_whose_group_cannot_be_read_refuses_the_claim() {
    let unreadable = |_| Some((AT, None::<&[Member]>));
    for ports in [(false, false), (true, false), (false, true), (true, true)] {
        let got = blocked_by(1, &[], &[9], ports, unreadable);
        assert_eq!(
            got,
            Some(Blocked::Unreadable(AT)),
            "admitted beside an unread group, {ports:?}"
        );
    }
}

#[test]
fn a_held_device_off_the_pci_tree_refuses_nothing() {
    let off_tree = |_| None::<((u8, u8, u8), Option<&[Member]>)>;
    assert_eq!(blocked_by(1, &[], &[9], (false, false), off_tree), None);
}

#[test]
fn an_unreadable_group_is_named() {
    let line = unreadable_line(AT, "driver.xhci0");
    assert_eq!(
        line,
        "refused: the isolation group of 02:00.0, which driver.xhci0 holds, can't be read"
    );
}
