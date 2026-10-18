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

//! MkAcpiUsbPorts and MkAcpiExternalPort: a USB host's port facts, and a
//! PCIe port firmware calls external-facing, each named by its function.

use super::acpi::{ok, tag4, Errno};
use nonos_libc::mk_syscall_raw;

const USB_PORTS: i64 = tag4(b"MAUP");
const EXTERNAL_PORT: i64 = tag4(b"MAXP");

pub type Bdf = (u16, u8, u8, u8);

fn bus_devfn(f: Bdf) -> u64 {
    u64::from(f.1) << 8 | u64::from(f.2 << 3 | f.3)
}

/// The host's broker device id.
pub fn usb_ports(f: Bdf, facts: &[u8]) -> Result<u64, Errno> {
    let (ptr, len) = (facts.as_ptr() as u64, facts.len() as u64);
    ok(mk_syscall_raw(USB_PORTS, [u64::from(f.0), bus_devfn(f), ptr, len, 0, 0]))
}

/// How many functions the kernel flagged, the port among them.
pub fn external_port(f: Bdf, dma_property: bool) -> Result<u64, Errno> {
    let flags = 1 | u64::from(dma_property) << 1;
    ok(mk_syscall_raw(EXTERNAL_PORT, [u64::from(f.0), bus_devfn(f), flags, 0, 0, 0]))
}
