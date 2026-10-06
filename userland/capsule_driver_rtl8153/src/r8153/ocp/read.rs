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

//! Register reads, as Linux ocp_read_dword, ocp_read_word and
//! ocp_read_byte: always the aligned dword, the wanted part shifted out.

use nonos_usbnet::xhci::E_IO;
use nonos_usbnet::Bus;

use super::dev::Dev;
use super::request::{get_regs, BYTE_EN_WORD};

/// `out.len()` bytes from `addr` in block `index` (Linux get_registers).
/// A short answer is an error here; Linux would go on with 0xff bytes.
pub fn get<B: Bus>(dev: &mut Dev<B>, index: u16, addr: u16, out: &mut [u8]) -> Result<(), i32> {
    let n = dev.bus.control_in(get_regs(addr, index), out)?;
    if n == out.len() {
        Ok(())
    } else {
        Err(E_IO)
    }
}

pub fn read_dword<B: Bus>(dev: &mut Dev<B>, ty: u16, addr: u16) -> Result<u32, i32> {
    let mut raw = [0u8; 4];
    get(dev, ty, addr & !3, &mut raw)?;
    Ok(u32::from_le_bytes(raw))
}

/// The word read names its two bytes in wIndex; ocp_read_dword and
/// ocp_read_byte send the bare block.
pub fn read_word<B: Bus>(dev: &mut Dev<B>, ty: u16, addr: u16) -> Result<u16, i32> {
    let shift = addr & 2;
    let mut raw = [0u8; 4];
    get(dev, ty | (BYTE_EN_WORD << shift), addr & !3, &mut raw)?;
    Ok((u32::from_le_bytes(raw) >> (shift * 8)) as u16)
}

pub fn read_byte<B: Bus>(dev: &mut Dev<B>, ty: u16, addr: u16) -> Result<u8, i32> {
    let dword = read_dword(dev, ty, addr)?;
    Ok((dword >> ((addr & 3) * 8)) as u8)
}
