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

//! DEVICE_POWER (10): a device's D-state, read by any caller the port
//! admits; moved, and armed to wake, only for its claim's holder (the
//! kernel's MkAcpiFunctionOf and MkAcpiDeviceName). Request: verb u8 (0
//! get, 1 set, 2 wake on, 3 wake off, 4 sleep facts, a get), state u8 (0
//! D0 to 4 D3cold: for set, the state; for wake on, the deepest the
//! driver will put it in), device id u64. Reply: `reply.rs`.
//! ENOENT for a device not named, not described, or with no wake event;
//! EPERM for any verb but get from anyone but the holder; EINVAL for a
//! state it lacks, a move between low-power states, or a wake target past
//! _S0W; ELOSES (-95) for a PCI function's move to D3cold, or to D3hot
//! without No_Soft_Reset, which its configuration would not survive
//! (`keep`); EIO when the AML failed.

use super::moves::move_to;
use super::reply::reply;
use super::target::target;
use super::{wake::wake, wake_check::code};
use crate::boot::Acpi;
use crate::server::protocol::EINVAL;
use crate::sys::function_of::Asker;
use alloc::vec::Vec;
use nonos_aml::device_power::DState;

const GET: u8 = 0;
const SET: u8 = 1;
const WAKE_ON: u8 = 2;
const WAKE_OFF: u8 = 3;
const FACTS: u8 = 4;

pub fn device_power(acpi: &mut Acpi, req: &[u8], sender: u32) -> (i32, Vec<u8>) {
    let Some(r) = req.get(1..11) else { return (EINVAL, Vec::new()) };
    let (verb, state) = (r[0], DState::from_wire(r[1]));
    let id = u64::from_le_bytes(r[2..10].try_into().unwrap_or([0; 8]));
    let look = verb == GET || verb == FACTS;
    let asker = if look { Asker::Capsule } else { Asker::Holder(sender) };
    let dev = match target(acpi, asker, id) {
        Ok(d) => d,
        Err(e) => return (e, Vec::new()),
    };
    let code = match (verb, state) {
        (GET | FACTS, _) => 0,
        (SET, Some(to)) => move_to(acpi, dev, to, sender),
        (WAKE_ON, Some(to)) => code(wake(acpi, dev, Some(to), (sender, id))),
        (WAKE_OFF, _) => code(wake(acpi, dev, None, (sender, id))),
        _ => EINVAL,
    };
    (code, reply(acpi, dev, sender, verb == FACTS))
}
