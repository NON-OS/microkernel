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

//! MkAcpiPci: one configuration access.

use super::acpi::{ok, Errno, PCI};
use nonos_libc::mk_syscall_raw;

/// (segment, bus, device, function) and register, laid out as the call
/// takes them: the ECAM offset with the segment above it.
pub fn pci_address(f: (u16, u8, u8, u8), reg: u16) -> u64 {
    let (s, b, d, fun) = (f.0 as u64, f.1 as u64, f.2 as u64, f.3 as u64);
    (s << 32) | (b << 20) | (d << 15) | (fun << 12) | (reg as u64 & 0xFFF)
}

pub fn pci_read(f: (u16, u8, u8, u8), reg: u16, width: u8) -> Result<u32, Errno> {
    ok(mk_syscall_raw(PCI, [pci_address(f, reg), width as u64, 0, 0, 0, 0])).map(|v| v as u32)
}

pub fn pci_write(f: (u16, u8, u8, u8), reg: u16, width: u8, v: u32) -> Result<(), Errno> {
    ok(mk_syscall_raw(PCI, [pci_address(f, reg), width as u64, 1, v as u64, 0, 0])).map(|_| ())
}
