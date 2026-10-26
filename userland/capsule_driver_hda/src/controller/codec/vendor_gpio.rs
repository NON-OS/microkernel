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
//! The GPIOs a machine wires its amplifier enable (or its LEDs) to, driven
//! as each vendor's Linux code drives them on every machine of a kind
//! (values only, no code taken, Linux at 21a7f44fd076):
//!
//! - Realtek on HP: every GPIO an output, high (`program::hp_gpio`).
//! - Cirrus CS4206 and CS4207 on Apple: GPIO1 (headphones) and GPIO3
//!   (speakers), or GPIO2 and GPIO3 on the iMac 12,1, 12,2 and 14,1
//!   (sound/hda/codecs/cirrus/cs420x.c:440-463, the machines at :335-350),
//!   both high, as Linux leaves them with auto-mute off.
//! - Cirrus CS4208 on Apple, and as the MacBook Air 7,2 (8086:7270): GPIO0,
//!   the speakers (cs420x.c:573-616).
//! - Any other: none. A Conexant HP board lights its mute LED with a GPIO
//!   set high (conexant.c `cxt_update_gpio_led`), the opposite of
//!   Realtek's, so the HP rule is Realtek's alone.
//!
//! The machine is the codec's subsystem id, which firmware sets to the
//! board's PCI one that Linux's tables match. Unverified on hardware.

use super::program::hp_gpio;
use super::realtek::VENDOR_REALTEK;

const APPLE: u16 = 0x106b;
/// The MacBook Pro 5,3, 5,5, 7,1 and MacBook Air 3,1 (cs420x.c:335-338).
const NVIDIA_MACS: [u32; 4] = [0x10de_0ac0, 0x10de_0d94, 0x10de_cb79, 0x10de_cb89];
/// The iMac 14,1, 12,1 and 12,2 (cs420x.c:343-346).
const GPIO_23_MACS: [u32; 3] = [0x106b_0600, 0x106b_0900, 0x106b_2000];

fn bits(vendor_id: u32, subsystem_id: u32, count: u8) -> Option<u8> {
    let realtek = (vendor_id >> 16) as u16 == VENDOR_REALTEK || vendor_id == 0x19e5_8326;
    let apple = (subsystem_id >> 16) as u16 == APPLE;
    match vendor_id {
        _ if realtek => hp_gpio(subsystem_id, count),
        0x1013_4206 | 0x1013_4207 if GPIO_23_MACS.contains(&subsystem_id) => Some(0x0c),
        0x1013_4206 | 0x1013_4207 if apple || NVIDIA_MACS.contains(&subsystem_id) => Some(0x0a),
        0x1013_4208 if apple || subsystem_id == 0x8086_7270 => Some(0x01),
        _ => None,
    }
}

/// The GPIOs to drive as outputs, high, for codec `vendor_id` on machine
/// `subsystem_id` with `count` GPIOs; none when the codec lacks one of them.
pub fn gpio(vendor_id: u32, subsystem_id: u32, count: u8) -> Option<u8> {
    bits(vendor_id, subsystem_id, count).filter(|&b| count >= 8 || b >> count == 0)
}
