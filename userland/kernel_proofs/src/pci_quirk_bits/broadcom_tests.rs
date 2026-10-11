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

//! A Broadcom Ethernet controller's bits around Linux tg3's chip reset, and
//! the Device Control bits it may only turn off.

use super::quirk_bits::{clear_only, cleared_only, only, writable, Ident};

const PCIE: u16 = 0xA0;
const TG3: Ident = Ident { vendor: 0x14E4, class: 0x02, subclass: 0x00, pcie_cap: Some(PCIE) };

#[test]
fn tg3_gets_host_control_pci_state_and_the_error_status() {
    assert_eq!(writable(&TG3, 0x68), Some(0x03FF));
    // tg3's own value from a reset default: swaps and indirect access on.
    assert!(only(0x03FF, 0x009A, 0x0000), "MISC_HOST_CTRL as Linux sets it");
    assert_eq!(writable(&TG3, 0x6A), None, "the chip revision half");
    assert_eq!(writable(&TG3, 0x70), Some(0x0060));
    assert!(only(0x0060, 0x0064, 0x0004), "ROM enable and retry, the mode bit kept");
    assert!(!only(0x0060, 0x0061, 0x0000), "PCISTATE's force reset");
    assert_eq!(writable(&TG3, PCIE as u32 + 0x0A), Some(0x000F), "Device Status");
}

#[test]
fn tg3_may_clear_relaxed_ordering_and_no_snoop_but_never_set_them() {
    let devctl = PCIE as u32 + 0x08;
    assert_eq!(clear_only(&TG3, devctl), Some(0x0810));
    assert_eq!(writable(&TG3, devctl), None, "Device Control is clear-only");
    assert!(cleared_only(0x0810, 0x2000, 0x2810), "both cleared, the payload kept");
    assert!(cleared_only(0x0810, 0x2810, 0x2810), "left as it was");
    assert!(!cleared_only(0x0810, 0x2810, 0x2000), "no snoop turned on");
    assert!(!cleared_only(0x0810, 0x2010, 0x2000), "relaxed ordering turned on");
    assert!(!cleared_only(0x0810, 0x0000, 0x2000), "the read request size");
}

#[test]
fn no_other_vendor_or_class_and_no_other_offset_gets_them() {
    for offset in [0x00, 0x04, 0x10, 0x6C, 0x74, 0x78, 0x7C, 0x84] {
        assert_eq!(writable(&TG3, offset), None, "{offset:#x}");
    }
    let intel = Ident { vendor: 0x8086, ..TG3 };
    let wifi = Ident { subclass: 0x80, ..TG3 };
    for id in [intel, wifi] {
        assert_eq!(writable(&id, 0x68), None);
        assert_eq!(clear_only(&id, PCIE as u32 + 0x08), None);
    }
    let no_pcie = Ident { pcie_cap: None, ..TG3 };
    assert_eq!(clear_only(&no_pcie, 0x08), None, "no capability, no Device Control");
}
