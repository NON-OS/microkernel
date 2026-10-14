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

/// Answer with the touchpad setup bound: its I2C slave address, HID
/// descriptor register, GPIO pin and interrupt facts (the kernel's HID_INFO
/// bits), packed little-endian. The address is the declared one or, for a
/// declaration with none, the first the sweep found (setup/sequence/
/// sweep.rs); the register is the one the probe found the descriptor at:
/// multi-SKU firmware declares several pads and only one is fitted, so "the
/// first record" can be a phantom, and firmware that computes its `_DSM` at
/// run time leaves the declared register a guess. With nothing bound, the
/// first declared 7-bit address is answered. An empty body means no device
/// this driver may reach is known, and the HID driver finds no pad.
pub fn handle(driver: &Driver, sender_pid: u32, req: &Request, out: &mut [u8]) {
    let answer = match driver.bound_addr {
        0 => first_declared(),
        addr => Some((addr, driver.bound_desc_reg, driver.bound_pin, driver.bound_info)),
    };
    let Some((addr, reg, pin, info)) = answer else {
        let _ = respond::send(sender_pid, req, E_OK, &[], out);
        return;
    };
    let mut body = [0u8; 8];
    body[0..2].copy_from_slice(&u16::from(addr).to_le_bytes());
    body[2..4].copy_from_slice(&reg.to_le_bytes());
    body[4..6].copy_from_slice(&pin.to_le_bytes());
    body[6] = info;
    let _ = respond::send(sender_pid, req, E_OK, &body, out);
}

/// The first declaration with a 7-bit address, as (address, descriptor
/// register, pin, HID_INFO bits).
fn first_declared() -> Option<(u8, u16, u16, u8)> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    let count = if n > 0 { core::cmp::min(n as usize, MAX_DEVICES) } else { 0 };
    let tp = buf[..count].iter().filter_map(touchpad_of).find(|tp| tp.addressed())?;
    Some((tp.addr, tp.desc_reg, tp.gpio_pin, tp.info))
}
