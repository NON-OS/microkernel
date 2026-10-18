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

//! `MkAcpiExternalPort(segment, bus << 8 | devfn, flags) -> functions`:
//! the ACPI capsule names a PCIe port whose `_DSD` says ExternalFacingPort
//! (flag bit 0, which must be set) and, bit 1, DmaProperty as well. The
//! kernel reads the buses behind it from the port itself and sets
//! DEVICE_FLAG_EXTERNAL on the port and every function there, answering
//! how many. Only the `acpi` service's owner may call it. EINVAL for other
//! flags or an address out of range; ENOTSUP for a port that is no bridge
//! or whose bus numbers are not set; the config access's own errno else.

use super::errno::region_errno;
use crate::hardware::broker::acpi::external_port::{declare, ExternalError};
use crate::process::current_pid;
use crate::services::registry::lookup_service;
use crate::syscall::microkernel::errnos::{ERRNO_INVAL, ERRNO_NOTSUP, ERRNO_PERM};

const EXTERNAL: u64 = 1 << 0;
const DMA_PROPERTY: u64 = 1 << 1;

/// The same packing MkAcpiFunctionOf answers with, split.
pub(super) fn bdf_of(segment: u64, bus_devfn: u64) -> Option<(u16, u8, u8, u8)> {
    let segment = u16::try_from(segment).ok()?;
    let [devfn, bus] = u16::try_from(bus_devfn).ok()?.to_le_bytes();
    Some((segment, bus, devfn >> 3, devfn & 7))
}

pub fn sys_acpi_external_port(segment: u64, bus_devfn: u64, flags: u64) -> i64 {
    let Some(pid) = lookup_service("acpi").map(|e| e.pid).filter(|p| Some(*p) == current_pid())
    else {
        return ERRNO_PERM;
    };
    let Some(bdf) = bdf_of(segment, bus_devfn) else { return ERRNO_INVAL };
    if flags & EXTERNAL == 0 || flags & !(EXTERNAL | DMA_PROPERTY) != 0 {
        return ERRNO_INVAL;
    }
    match declare(pid, bdf, flags & DMA_PROPERTY != 0) {
        Ok(n) => n as i64,
        Err(ExternalError::NotBridge) => ERRNO_NOTSUP,
        Err(ExternalError::Config(e)) => region_errno(e),
    }
}
