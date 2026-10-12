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

//! A found controller as the kernel's `Platform`, encoded by the kernel's
//! own encoder (`record/`): None for one the kernel would refuse as
//! malformed, which is then not sent.

use super::record::{encode, Platform, GPIO, HAS_IRQ, I2C, SD};
use alloc::vec::Vec;
use nonos_aml::platform_device::{Class, PlatformDevice};

fn class(c: Class) -> u8 {
    match c {
        Class::I2c => I2C,
        Class::Sd => SD,
        Class::Gpio => GPIO,
    }
}

/// HAS_IRQ, then edge, active low, shared and wake, one bit each.
fn irq_flags(d: &PlatformDevice) -> u8 {
    d.irq.map_or(0, |i| {
        let bits = [i.edge, i.active_low, i.shared, i.wake];
        bits.iter().enumerate().fold(HAS_IRQ, |f, (k, on)| f | u8::from(*on) << (k + 1))
    })
}

pub fn record(d: &PlatformDevice, path: &[u8]) -> Option<Vec<u8>> {
    let mut hid = [0u8; 8];
    hid.get_mut(..d.matched.len())?.copy_from_slice(&d.matched);
    encode(&Platform {
        class: class(d.class),
        irq_flags: irq_flags(d),
        hid,
        uid: d.uid,
        gsiv: d.irq.map_or(0, |i| i.gsiv),
        windows: d.windows.iter().map(|w| (w.base, w.len)).collect(),
        path: path.to_vec(),
    })
}
