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

//! DEVICE_POWER (10): a device's D-state, PCI function or ACPI-bus device,
//! read by any caller the port admits, and moved only for the pid that
//! holds the device's claim (the kernel answers MkAcpiFunctionOf and
//! MkAcpiDeviceName for its holder alone), so a driver powers its own
//! device and no other. Drivers name it by device id.
//!
//! Request: verb u8 (0 get, 1 set), state u8 (0 D0, 1 D1, 2 D2, 3 D3hot,
//! 4 D3cold; set only), device id u64. Reply: the state after, u8 (0xFF
//! when unknown), whether the device has power control at all, u8, then
//! its path's length u8 and bytes. ENOENT for a device the kernel names
//! neither way or that the tables do not describe; EPERM for a set from
//! anyone but the holder; EINVAL for a state the device lacks or a move
//! between two low-power states; EIO when the AML failed, the device left
//! where it was.

use super::moves::{current, move_to};
use super::target::target;
use crate::boot::{path, Acpi};
use crate::server::protocol::EINVAL;
use crate::sys::function_of::Asker;
use alloc::vec::Vec;
use nonos_aml::device_power::{power_managed, DState};

const GET: u8 = 0;
const SET: u8 = 1;
const UNKNOWN: u8 = 0xFF;

pub fn device_power(acpi: &mut Acpi, req: &[u8], sender: u32) -> (i32, Vec<u8>) {
    let Some(r) = req.get(1..11) else { return (EINVAL, Vec::new()) };
    let (verb, state) = (r[0], DState::from_wire(r[1]));
    let mut id = [0u8; 8];
    id.copy_from_slice(&r[2..10]);
    let asker = if verb == SET { Asker::Holder(sender) } else { Asker::Capsule };
    let dev = match target(acpi, asker, u64::from_le_bytes(id)) {
        Ok(d) => d,
        Err(e) => return (e, Vec::new()),
    };
    let code = match (verb, state) {
        (GET, _) => 0,
        (SET, Some(to)) => move_to(acpi, dev, to, sender),
        _ => EINVAL,
    };
    let now = current(acpi, dev).map_or(UNKNOWN, |s| s as u8);
    let mut v = alloc::vec![now, power_managed(&acpi.aml, dev) as u8];
    let p = path(&acpi.aml, dev);
    let p = &p.as_bytes()[..p.len().min(200)];
    v.push(p.len() as u8);
    v.extend_from_slice(p);
    (code, v)
}
