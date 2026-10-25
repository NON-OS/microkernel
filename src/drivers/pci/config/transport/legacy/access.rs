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

//! The original PC mechanism: an address port and a data port.
//!
//! Only dwords move, so narrower reads shift out of the containing dword and
//! narrower writes are read-modify-write. That is a real limitation of the
//! mechanism and one reason ECAM is preferred where it exists. Every access
//! answers `None` for a register the address word cannot name.

use super::dword;

pub(in crate::drivers::pci::config) fn read8(b: u8, d: u8, f: u8, offset: u16) -> Option<u8> {
    let shift = (offset & 3) * 8;
    Some(((dword::read(b, d, f, offset)? >> shift) & 0xFF) as u8)
}

pub(in crate::drivers::pci::config) fn read16(b: u8, d: u8, f: u8, offset: u16) -> Option<u16> {
    let shift = (offset & 2) * 8;
    Some(((dword::read(b, d, f, offset)? >> shift) & 0xFFFF) as u16)
}

pub(in crate::drivers::pci::config) fn read32(b: u8, d: u8, f: u8, offset: u16) -> Option<u32> {
    dword::read(b, d, f, offset)
}

pub(in crate::drivers::pci::config) fn write8(
    b: u8,
    d: u8,
    f: u8,
    offset: u16,
    v: u8,
) -> Option<()> {
    let shift = ((offset & 3) * 8) as u32;
    dword::modify(b, d, f, offset, 0xFF << shift, (v as u32) << shift)
}

pub(in crate::drivers::pci::config) fn write16(
    b: u8,
    d: u8,
    f: u8,
    offset: u16,
    v: u16,
) -> Option<()> {
    let shift = ((offset & 2) * 8) as u32;
    dword::modify(b, d, f, offset, 0xFFFF << shift, (v as u32) << shift)
}

pub(in crate::drivers::pci::config) fn write32(
    b: u8,
    d: u8,
    f: u8,
    offset: u16,
    v: u32,
) -> Option<()> {
    dword::write(b, d, f, offset, v)
}
