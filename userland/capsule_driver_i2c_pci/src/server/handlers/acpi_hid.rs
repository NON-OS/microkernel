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

use nonos_libc::{mk_device_list, DeviceRecord};

use crate::discover::touchpad_of;
use crate::driver::Driver;
use crate::protocol::{Request, E_OK};
use crate::server::respond;

const MAX_DEVICES: usize = 128;

/// Answer with the touchpad the kernel registered from ACPI: its I2C slave
/// address, HID descriptor register, GPIO pin and interrupt facts (the
/// kernel's HID_INFO bits), packed little-endian. When setup settled on a
/// candidate address (the one that answered the probe on the bound bus), that
/// record is answered, with the descriptor register the probe found the
/// descriptor at: multi-SKU firmware declares several pads and only one is
/// fitted, so "the first record" can be a phantom, and firmware that computes
/// its `_DSM` at run time leaves the declared register a guess. An empty body
/// means the firmware declared no addressable device and the HID driver
/// scans the bus.
pub fn handle(driver: &Driver, sender_pid: u32, req: &Request, out: &mut [u8]) {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    let count = if n > 0 { core::cmp::min(n as usize, MAX_DEVICES) } else { 0 };
    let mut first = None;
    let mut matched = None;
    for tp in buf[..count].iter().filter_map(touchpad_of).filter(|tp| tp.addr != 0) {
        if first.is_none() {
            first = Some(tp);
        }
        if driver.bound_addr != 0 && tp.addr == driver.bound_addr {
            matched = Some(tp);
            break;
        }
    }
    let answer = match (matched, first) {
        (Some(tp), _) => Some((tp, if driver.bound_desc_reg != 0 { driver.bound_desc_reg } else { tp.desc_reg })),
        (None, _) if driver.bound_addr != 0 => None,
        (None, Some(tp)) => Some((tp, tp.desc_reg)),
        (None, None) => None,
    };
    match answer {
        Some((tp, reg)) => {
            let mut body = [0u8; 8];
            body[0..2].copy_from_slice(&u16::from(tp.addr).to_le_bytes());
            body[2..4].copy_from_slice(&reg.to_le_bytes());
            body[4..6].copy_from_slice(&tp.gpio_pin.to_le_bytes());
            body[6] = tp.info;
            let _ = respond::send(sender_pid, req, E_OK, &body, out);
        }
        // A blind scan bound an address the firmware never declared.
        None if driver.bound_addr != 0 => {
            let mut body = [0u8; 8];
            body[0..2].copy_from_slice(&u16::from(driver.bound_addr).to_le_bytes());
            body[2..4].copy_from_slice(&driver.bound_desc_reg.max(1).to_le_bytes());
            let _ = respond::send(sender_pid, req, E_OK, &body, out);
        }
        None => {
            let _ = respond::send(sender_pid, req, E_OK, &[], out);
        }
    }
}
