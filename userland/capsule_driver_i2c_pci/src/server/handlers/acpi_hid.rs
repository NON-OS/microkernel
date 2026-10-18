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

use crate::driver::Driver;
use crate::protocol::{Request, E_OK};
use crate::server::respond;

/// Answer with a touchpad a client may reach: its I2C slave address, HID
/// descriptor register, GPIO pin and interrupt facts (the kernel's HID_INFO
/// bits), packed little-endian. Multi-SKU firmware declares several pads
/// and only one is fitted, so until a client has read a pad's descriptor
/// every pad declared on the controller stays in reach, and each ask names
/// the next of them, in turn, the one setup bound first
/// (transaction/types/named.rs). The bound address is
/// the declared one or, for a declaration with none, the first the sweep
/// found (setup/sequence/sweep.rs), and its register the one setup's probe
/// found the descriptor at: firmware that computes its `_DSM` at run time
/// leaves the declared register a guess. The pin and the bits are the bound
/// declaration's, the line OP_GPIO_DOORBELL senses, and zero for any other
/// pad. An empty body means nothing is in reach, and the HID driver finds
/// no pad.
pub fn handle(driver: &Driver, sender_pid: u32, req: &Request, out: &mut [u8]) {
    let Some((addr, reg)) = driver.reach.get().named(&driver.acpi_hid_asks) else {
        let _ = respond::send(sender_pid, req, E_OK, &[], out);
        return;
    };
    let bound = addr == driver.bound_addr;
    let (pin, info) = if bound { (driver.bound_pin, driver.bound_info) } else { (0, 0) };
    let mut body = [0u8; 8];
    body[0..2].copy_from_slice(&u16::from(addr).to_le_bytes());
    body[2..4].copy_from_slice(&reg.to_le_bytes());
    body[4..6].copy_from_slice(&pin.to_le_bytes());
    body[6] = info;
    let _ = respond::send(sender_pid, req, E_OK, &body, out);
}
