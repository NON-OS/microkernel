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

//! `MkAcpiPci(address, width, write, value) -> value read, or 0`: one
//! PCI_Config access for the ACPI capsule. `address` is laid out as an ECAM
//! offset with the segment above it (PCI Express Base 6.0, 7.2.2): segment
//! in bits 32-47, bus 20-27, device 15-19, function 12-14, register 0-11.

use super::errno::region_errno;
use crate::hardware::broker::acpi::{pci_read, pci_write};
use crate::process::current_pid;
use crate::syscall::microkernel::errnos::{ERRNO_INVAL, ERRNO_PERM};
use crate::syscall::microkernel::narrow::{u32_arg, u8_arg};

pub fn sys_acpi_pci(address: u64, width: u64, write: u64, value: u64) -> i64 {
    let Some(pid) = current_pid() else { return ERRNO_PERM };
    let Some(width) = u8_arg(width) else { return ERRNO_INVAL };
    if address >> 48 != 0 {
        return ERRNO_INVAL;
    }
    let bdf = (
        (address >> 32) as u16,
        (address >> 20) as u8,
        ((address >> 15) & 0x1F) as u8,
        ((address >> 12) & 0x7) as u8,
    );
    let register = (address & 0xFFF) as u16;
    if address & 0xF000_0000 != 0 {
        return ERRNO_INVAL;
    }
    if write == 0 {
        return pci_read(pid, bdf, register, width).map_or_else(region_errno, i64::from);
    }
    let Some(value) = u32_arg(value) else { return ERRNO_INVAL };
    pci_write(pid, bdf, register, width, value).map_or_else(region_errno, |()| 0)
}
