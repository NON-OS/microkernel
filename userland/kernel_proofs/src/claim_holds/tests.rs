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

use super::hold_rule::{hold_for, untranslated, Hold, VMMDEV_SERVICE};

#[test]
fn vmmdev_is_held_for_its_own_service() {
    assert_eq!(hold_for(0x80EE, 0xCAFE), Hold::ForService(VMMDEV_SERVICE));
    assert_eq!(VMMDEV_SERVICE, "driver.vmmdev0");
    assert!(untranslated(0x80EE, 0xCAFE));
}

#[test]
fn both_balloon_ids_are_refused_to_everyone() {
    assert_eq!(hold_for(0x1AF4, 0x1002), Hold::Refused);
    assert_eq!(hold_for(0x1AF4, 0x1045), Hold::Refused);
    // The transitional id is 0x1000 + (type - 1) and the modern 0x1040 + type
    // for type 5 (Virtio 1.2, 4.1.2.1); the neighbours are other devices.
    for other in [0x1000, 0x1001, 0x1003, 0x1005, 0x1041, 0x1042, 0x1044, 0x1046, 0x1050] {
        assert_eq!(hold_for(0x1AF4, other), Hold::Open, "{other:#x}");
    }
}

#[test]
fn every_other_device_is_open_and_translated() {
    for vendor in [0x0000u16, 0x1022, 0x1234, 0x15AD, 0x1AF4, 0x8086, 0x80EE, 0xFFFF] {
        for device in [0x0000u16, 0x0405, 0x1000, 0xBEEF, 0xCAFF, 0xCAFD, 0xFFFF] {
            let held = (vendor, device) == (0x80EE, 0xCAFE)
                || (vendor == 0x1AF4 && (device == 0x1002 || device == 0x1045));
            assert_eq!(hold_for(vendor, device) != Hold::Open, held, "{vendor:x}:{device:x}");
            assert_eq!(untranslated(vendor, device), (vendor, device) == (0x80EE, 0xCAFE));
        }
    }
    // VBoxSVGA and VBoxVGA share the vendor and stay open.
    assert_eq!(hold_for(0x80EE, 0xBEEF), Hold::Open);
    assert!(!untranslated(0x80EE, 0xBEEF));
}
