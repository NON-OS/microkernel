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

//! A UHCI controller's LEGSUP, for any vendor, and no other USB host's.

use super::quirk_bits::{only, writable, Ident};

const UHCI: Ident =
    Ident { vendor: 0x8086, class: 0x0C, subclass: 0x03, prog_if: 0, pcie_cap: None };

#[test]
fn a_uhci_controller_turns_firmware_traps_off_and_clears_their_status() {
    for vendor in [0x8086, 0x1106, 0x15AD] {
        assert_eq!(writable(&Ident { vendor, ..UHCI }, 0xC0), Some(0xAFFF), "{vendor:#x}");
    }
    // What firmware left (traps on, a status raised) to what the driver
    // writes: every trap off, every status written as one, PIRQD off.
    assert!(only(0xAFFF, 0x8F00, 0x2010 | 0x0100));
    assert!(only(0xAFFF, 0x9F00, 0x1010), "bit 12 kept as read");
    assert!(!only(0xAFFF, 0x8F00, 0x1000), "bit 12 changed");
    assert!(!only(0xAFFF, 0xCF00, 0x0000), "bit 14 changed");
}

#[test]
fn ohci_ehci_and_xhci_and_other_registers_stay_closed() {
    for prog_if in [0x10, 0x20, 0x30] {
        assert_eq!(writable(&Ident { prog_if, ..UHCI }, 0xC0), None, "{prog_if:#x}");
    }
    for offset in [0x04, 0x20, 0x3C, 0x60, 0xC2, 0xC4] {
        assert_eq!(writable(&UHCI, offset), None, "{offset:#x}");
    }
}
