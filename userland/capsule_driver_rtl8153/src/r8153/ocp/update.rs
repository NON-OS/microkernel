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

//! Read, change, write back: the shape of nearly every step in r8152.c
//! ("ocp_data = ocp_read_word(...); ocp_data &= ~X; ocp_data |= Y;
//! ocp_write_word(...)").

use nonos_usbnet::Bus;

use super::dev::Dev;
use super::read::{read_byte, read_dword, read_word};
use super::write::{write_byte, write_dword, write_word};

pub fn update_dword<B: Bus>(
    dev: &mut Dev<B>,
    ty: u16,
    addr: u16,
    clear: u32,
    set: u32,
) -> Result<(), i32> {
    let v = read_dword(dev, ty, addr)?;
    write_dword(dev, ty, addr, (v & !clear) | set)
}

pub fn update_word<B: Bus>(
    dev: &mut Dev<B>,
    ty: u16,
    addr: u16,
    clear: u16,
    set: u16,
) -> Result<(), i32> {
    let v = read_word(dev, ty, addr)?;
    write_word(dev, ty, addr, (v & !clear) | set)
}

pub fn update_byte<B: Bus>(
    dev: &mut Dev<B>,
    ty: u16,
    addr: u16,
    clear: u8,
    set: u8,
) -> Result<(), i32> {
    let v = read_byte(dev, ty, addr)?;
    write_byte(dev, ty, addr, (v & !clear) | set)
}
