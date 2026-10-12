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

use super::hold_rule::{
    hold_for, hold_for_firmware, Hold, BOOTFB_SERVICE, FIRMWARE_BOOT_FRAMEBUFFER,
};

#[test]
fn the_boot_framebuffer_is_held_for_its_own_service() {
    // Whoever holds it can read the whole screen, so no other driver may.
    assert_eq!(hold_for_firmware(FIRMWARE_BOOT_FRAMEBUFFER), Hold::ForService(BOOTFB_SERVICE));
    assert_eq!(BOOTFB_SERVICE, "driver.bootfb0");
}

#[test]
fn every_other_firmware_device_is_refused_until_named() {
    for device in [0x0000u16, 0x0002, 0x0003, 0x00FF, 0x0100, 0x8000, 0xFFFF] {
        assert_eq!(hold_for_firmware(device), Hold::Refused, "{device:#x}");
    }
}

#[test]
fn a_pci_device_with_the_same_number_is_not_held() {
    // The firmware rule is keyed by bus: PCI 0000:0001 stays open.
    assert_eq!(hold_for(0x0000, FIRMWARE_BOOT_FRAMEBUFFER), Hold::Open);
}
