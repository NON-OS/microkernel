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

//! `MkAcpiUsbPorts(segment, bus << 8 | devfn, facts, len) -> device id`:
//! the ACPI capsule publishes a USB host's port facts (`_UPC`, `_PLD`),
//! eight bytes a port, kept beside the host's record. Only the `acpi`
//! service's owner may call it. `MkUsbPortFacts(device_id, out, len) ->
//! bytes` reads them back for anyone who may list devices: which ports are
//! built in (a camera, a Bluetooth module) is no secret of the host's
//! holder. EINVAL for malformed facts or a short `out`; ENODEV for an
//! address with no function; ENOTSUP for a function that is no USB host;
//! ENOENT for a host nothing was published for.

use super::external::bdf_of;
use crate::hardware::broker::acpi::usb_port_wire::{MAX_PORTS, PORT_LEN};
use crate::hardware::broker::acpi::usb_ports::{facts, publish, PublishError};
use crate::process::current_pid;
use crate::services::registry::lookup_service;
use crate::syscall::microkernel::errnos::{
    ERRNO_FAULT, ERRNO_INVAL, ERRNO_NODEV, ERRNO_NOENT, ERRNO_NOTSUP, ERRNO_PERM,
};
use crate::usercopy::{read_user_bytes, write_user_bytes};

pub fn sys_acpi_usb_ports(segment: u64, bus_devfn: u64, ptr: u64, len: u64) -> i64 {
    let owner = lookup_service("acpi").map(|e| e.pid);
    if owner.is_none() || owner != current_pid() {
        return ERRNO_PERM;
    }
    let Some(bdf) = bdf_of(segment, bus_devfn) else { return ERRNO_INVAL };
    let Some(len) = usize::try_from(len).ok().filter(|l| *l <= MAX_PORTS * PORT_LEN) else {
        return ERRNO_INVAL;
    };
    let Ok(bytes) = read_user_bytes(ptr, len) else { return ERRNO_FAULT };
    match publish(bdf, &bytes) {
        Ok(id) => id as i64,
        Err(PublishError::Malformed) => ERRNO_INVAL,
        Err(PublishError::NoFunction) => ERRNO_NODEV,
        Err(PublishError::NotUsbHost) => ERRNO_NOTSUP,
    }
}

pub fn sys_usb_port_facts(device_id: u64, out: u64, len: u64) -> i64 {
    let Some(b) = facts(device_id) else { return ERRNO_NOENT };
    if len < b.len() as u64 {
        return ERRNO_INVAL;
    }
    match write_user_bytes(out, &b) {
        Ok(()) => b.len() as i64,
        Err(_) => ERRNO_FAULT,
    }
}
