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

//! Register writes, as Linux ocp_write_dword, ocp_write_word and
//! ocp_write_byte: one aligned dword whose byte enables name the bytes
//! that change (generic_ocp_write with a single dword, whose enables are
//! the start nibble copied to the end nibble, the same value).

use nonos_usbnet::Bus;

use super::dev::Dev;
use super::request::{set_regs, BYTE_EN_BYTE, BYTE_EN_DWORD, BYTE_EN_WORD};

/// `data` to `addr`, with the block and byte enables in `index` (Linux
/// set_registers).
pub fn set<B: Bus>(dev: &mut Dev<B>, index: u16, addr: u16, data: &[u8]) -> Result<(), i32> {
    dev.bus.control_out(set_regs(addr, index), data)
}

pub fn write_dword<B: Bus>(dev: &mut Dev<B>, ty: u16, addr: u16, v: u32) -> Result<(), i32> {
    set(dev, ty | BYTE_EN_DWORD, addr & !3, &v.to_le_bytes())
}

pub fn write_word<B: Bus>(dev: &mut Dev<B>, ty: u16, addr: u16, v: u16) -> Result<(), i32> {
    let shift = addr & 2;
    let data = (v as u32) << (shift * 8);
    set(dev, ty | (BYTE_EN_WORD << shift), addr & !3, &data.to_le_bytes())
}

pub fn write_byte<B: Bus>(dev: &mut Dev<B>, ty: u16, addr: u16, v: u8) -> Result<(), i32> {
    let shift = addr & 3;
    let data = (v as u32) << (shift * 8);
    set(dev, ty | (BYTE_EN_BYTE << shift), addr & !3, &data.to_le_bytes())
}
