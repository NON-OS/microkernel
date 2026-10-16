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
//! read by any caller the port admits; moved, and armed to wake, only for
//! the pid holding its claim (the kernel answers MkAcpiFunctionOf and
//! MkAcpiDeviceName for its holder alone). Named by device id. Request:
//! verb u8 (0 get, 1 set, 2 wake on, 3 wake off), state u8 (0 D0 to 4
//! D3cold: for set, the state; for wake on, the deepest the driver will
//! put it in), device id u64. Reply: the state after, u8 (0xFF when
//! unknown), whether it has power control, u8, its path's length u8 and
//! bytes; then, only to the holder that armed its wake, 1 u8 and the
//! wakes counted since, u32. ENOENT for a device the kernel names neither
//! way, the tables do not describe, or with no wake event; EPERM for any
//! verb but get from anyone but the holder; EINVAL for a state it lacks,
//! a move between low-power states, or a wake target past _S0W; EIO when
//! the AML failed, the device left as it was.

use super::moves::{current, move_to};
use super::target::target;
use super::wake::{code, wake};
use super::wake_sweep::owns;
use crate::boot::{path, Acpi};
use crate::server::protocol::EINVAL;
use crate::sys::function_of::Asker;
use alloc::vec::Vec;
use nonos_aml::device_power::{power_managed, DState};

const GET: u8 = 0;
const SET: u8 = 1;
const WAKE_ON: u8 = 2;
const WAKE_OFF: u8 = 3;
const UNKNOWN: u8 = 0xFF;

pub fn device_power(acpi: &mut Acpi, req: &[u8], sender: u32) -> (i32, Vec<u8>) {
    let Some(r) = req.get(1..11) else { return (EINVAL, Vec::new()) };
    let (verb, state) = (r[0], DState::from_wire(r[1]));
    let id = u64::from_le_bytes(r[2..10].try_into().unwrap_or([0; 8]));
    let asker = if verb == GET { Asker::Capsule } else { Asker::Holder(sender) };
    let dev = match target(acpi, asker, id) {
        Ok(d) => d,
        Err(e) => return (e, Vec::new()),
    };
    let code = match (verb, state) {
        (GET, _) => 0,
        (SET, Some(to)) => move_to(acpi, dev, to, sender),
        (WAKE_ON, Some(to)) => code(wake(acpi, dev, Some(to), (sender, id))),
        (WAKE_OFF, _) => code(wake(acpi, dev, None, (sender, id))),
        _ => EINVAL,
    };
    let now = current(acpi, dev).map_or(UNKNOWN, |s| s as u8);
    let mut v = alloc::vec![now, power_managed(&acpi.aml, dev) as u8];
    let p = path(&acpi.aml, dev);
    let p = &p.as_bytes()[..p.len().min(200)];
    v.push(p.len() as u8);
    v.extend_from_slice(p);
    if owns(&acpi.hidden.wake.owners, dev, sender) {
        v.push(1);
        v.extend_from_slice(&acpi.hidden.wake.rw.wakes(dev).unwrap_or(0).to_le_bytes());
    }
    (code, v)
}
