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

//! The id table against Linux's rtsx_pci_ids and the BAR rtsx_pci_probe
//! maps.

use crate::chip::id::{LINUX_IDS, VENDOR_REALTEK};
use crate::chip::{family, is_linux_reader, register_bar, Family};

#[test]
fn every_linux_id_is_a_reader_of_class_ff_only() {
    for id in LINUX_IDS {
        assert!(is_linux_reader(VENDOR_REALTEK, id, 0xFF), "{id:04x}");
        assert!(!is_linux_reader(VENDOR_REALTEK, id, 0x08), "{id:04x}");
        assert!(!is_linux_reader(0x8086, id, 0xFF), "{id:04x}");
    }
    // An RTL8821CE (the Wi-Fi card) is Realtek too, and no reader.
    assert!(!is_linux_reader(VENDOR_REALTEK, 0xC821, 0xFF));
}

#[test]
fn only_the_rts5227_family_is_brought_up() {
    assert_eq!(family(VENDOR_REALTEK, 0x5227, 0xFF), Some(Family::Rts5227));
    assert_eq!(family(VENDOR_REALTEK, 0x522A, 0xFF), Some(Family::Rts522a));
    for id in LINUX_IDS.iter().filter(|&&id| id != 0x5227 && id != 0x522A) {
        assert_eq!(family(VENDOR_REALTEK, *id, 0xFF), None, "{id:04x}");
    }
}

#[test]
fn the_registers_sit_in_bar1_only_on_the_rts525a_and_rts5264() {
    for id in LINUX_IDS {
        let want = if id == 0x525A || id == 0x5264 { 1 } else { 0 };
        assert_eq!(register_bar(id), want, "{id:04x}");
    }
}
