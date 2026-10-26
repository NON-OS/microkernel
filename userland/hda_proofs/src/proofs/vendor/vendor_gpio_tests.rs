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
//! Which GPIOs a codec's machine gets driven (controller/codec/vendor_gpio):
//! every one on an HP machine for a Realtek codec alone, an Apple machine's
//! amplifier GPIOs for Cirrus's CS4206, CS4207 and CS4208, and none for any
//! other codec or for a codec without the GPIO named.

use crate::controller::codec::vendor_gpio::gpio;

const HP: u32 = 0x103c_8651;

#[test]
fn the_hp_rule_is_realteks_alone() {
    assert_eq!(gpio(0x10ec_0236, HP, 3), Some(0x07));
    assert_eq!(gpio(0x19e5_8326, HP, 3), Some(0x07), "the Realtek-built Huawei codec");
    // A Conexant HP board lights its mute LED with a GPIO high.
    for vendor in [0x14f1_50f2u32, 0x14f1_1f86, 0x111d_76d1, 0x1013_4206, 0x1013_4208] {
        assert_eq!(gpio(vendor, HP, 8), None, "{vendor:#x} on HP");
    }
}

#[test]
fn a_cirrus_codec_in_a_mac_drives_its_amplifier_gpios() {
    assert_eq!(gpio(0x1013_4206, 0x106b_1c00, 4), Some(0x0a), "MacBook Pro 8,1: GPIO1 and 3");
    assert_eq!(gpio(0x1013_4207, 0x10de_cb79, 4), Some(0x0a), "MacBook Pro 5,5");
    for imac in [0x106b_0600, 0x106b_0900, 0x106b_2000] {
        assert_eq!(gpio(0x1013_4206, imac, 4), Some(0x0c), "{imac:#x}: GPIO2 and 3");
    }
    assert_eq!(gpio(0x1013_4208, 0x106b_7100, 4), Some(0x01), "MacBook Air 6,1: GPIO0");
    assert_eq!(gpio(0x1013_4208, 0x8086_7270, 1), Some(0x01), "MacBook Air 7,2");
}

#[test]
fn no_gpio_outside_those_machines_or_past_the_codecs_own() {
    assert_eq!(gpio(0x1013_4206, 0x1028_0001, 4), None, "a CS4206 not in a Mac");
    assert_eq!(gpio(0x1013_4208, 0x10de_cb79, 4), None, "those NVIDIA Macs carry a CS4206/7");
    assert_eq!(gpio(0x1013_4206, 0x106b_1c00, 3), None, "GPIO3 missing");
    assert_eq!(gpio(0x1013_4208, 0x106b_7100, 0), None, "no GPIO at all");
    assert_eq!(gpio(0x1013_4210, 0x106b_7100, 8), None, "a CS4210 is not a CS4208");
}
