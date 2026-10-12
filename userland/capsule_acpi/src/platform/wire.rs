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

//! The record MkAcpiPlatformDevice takes, as the kernel's
//! `broker/acpi/platform_wire.rs` lays it out: header, `_HID`, `_UID`,
//! GSI, windows, path. The kernel checks every field again.

use alloc::vec;
use alloc::vec::Vec;
use nonos_aml::platform_device::{Class, PlatformDevice};

pub const MAX_WINDOWS: usize = 5;
const MAX_PATH: usize = 127;
const HAS_IRQ: u8 = 1;

fn class(c: Class) -> u8 {
    match c {
        Class::I2c => 1,
        Class::Sd => 2,
        Class::Gpio => 3,
    }
}

/// None when the device does not fit the record: more windows than the
/// kernel takes, or a path longer than it keeps.
pub fn record(d: &PlatformDevice, path: &[u8]) -> Option<Vec<u8>> {
    let n = d.windows.len();
    if !(1..=MAX_WINDOWS).contains(&n) || !(1..=MAX_PATH).contains(&path.len()) {
        return None;
    }
    let flags = d.irq.map_or(0, |i| {
        let bits = [i.edge, i.active_low, i.shared, i.wake];
        bits.iter().enumerate().fold(HAS_IRQ, |f, (k, on)| f | u8::from(*on) << (k + 1))
    });
    let mut b = vec![1, class(d.class), n as u8, flags, 0, 0, 0, 0];
    let mut hid = [0u8; 8];
    hid.get_mut(..d.matched.len())?.copy_from_slice(&d.matched);
    b.extend_from_slice(&hid);
    b.extend_from_slice(&d.uid.to_le_bytes());
    b.extend_from_slice(&d.irq.map_or(0, |i| i.gsiv).to_le_bytes());
    b.extend_from_slice(&[0; 4]);
    for w in &d.windows {
        b.extend_from_slice(&w.base.to_le_bytes());
        b.extend_from_slice(&w.len.to_le_bytes());
    }
    b.push(path.len() as u8);
    b.extend_from_slice(path);
    Some(b)
}
