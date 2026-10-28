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

//! The chip's registers over endpoint 0: vendor request 05h, a dword at a
//! time, wValue the dword's address and wIndex the block (PLA 0100h, USB
//! 0000h) with, on a write, a byte-enable per lane (11h a byte, 33h a word,
//! FFh the dword) shifted to the lanes written. A word read names its lanes
//! too, as Linux v6.12 r8152.c sends it; a byte or dword read names none.

use nonos_usbnet::xhci::E_IO;
use nonos_usbnet::{Bus, Setup};

use super::reg::{Reg, Width};

const REGISTERS: u8 = 0x05;
const READ: u8 = 0xc0;
const WRITE: u8 = 0x40;

/// The dword's address, the lane shift in bytes, the byte enables, the mask.
fn lanes(r: Reg) -> (u16, u32, u16, u32) {
    let (shift, enable, mask) = match r.width {
        Width::W8 => (r.addr & 3, 0x11u16, 0xffu32),
        Width::W16 => (r.addr & 2, 0x33, 0xffff),
        Width::W32 => (0, 0xff, u32::MAX),
    };
    (r.addr & !3, shift as u32, enable << shift, mask)
}

pub fn read<B: Bus>(bus: &mut B, r: Reg) -> Result<u32, i32> {
    let (at, shift, enable, mask) = lanes(r);
    let lanes = if r.width == Width::W16 { enable } else { 0 };
    let mut d = [0u8; 4];
    let n = bus.control_in(Setup::new(READ, REGISTERS, at, r.block.index() | lanes), &mut d)?;
    if n != d.len() {
        return Err(E_IO);
    }
    Ok((u32::from_le_bytes(d) >> (8 * shift)) & mask)
}

pub fn write<B: Bus>(bus: &mut B, r: Reg, value: u32) -> Result<(), i32> {
    let (at, shift, enable, mask) = lanes(r);
    let d = ((value & mask) << (8 * shift)).to_le_bytes();
    bus.control_out(Setup::new(WRITE, REGISTERS, at, r.block.index() | enable), &d)
}
