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

//! The vendor request every register access is (Linux get_registers and
//! set_registers; the values from include/linux/usb/r8152.h): request
//! 0x05, wValue the register address, wIndex the MCU block in its high
//! byte and the byte enables in its low byte.

use nonos_usbnet::setup::{DIR_IN, TYPE_VENDOR};
use nonos_usbnet::Setup;

/// RTL8152_REQ_GET_REGS and RTL8152_REQ_SET_REGS are the same request.
const RTL8152_REQ_REGS: u8 = 0x05;

pub const MCU_TYPE_PLA: u16 = 0x0100;
pub const MCU_TYPE_USB: u16 = 0x0000;

/// Byte enables: all four bytes of a dword, the low two, the low one. A
/// word or byte higher in the dword shifts its enables up with it.
pub const BYTE_EN_DWORD: u16 = 0xff;
pub const BYTE_EN_WORD: u16 = 0x33;
pub const BYTE_EN_BYTE: u16 = 0x11;

/// RTL8152_REQT_READ (0xC0): vendor, device to host.
pub const fn get_regs(addr: u16, index: u16) -> Setup {
    Setup::new(DIR_IN | TYPE_VENDOR, RTL8152_REQ_REGS, addr, index)
}

/// RTL8152_REQT_WRITE (0x40): vendor, host to device.
pub const fn set_regs(addr: u16, index: u16) -> Setup {
    Setup::new(TYPE_VENDOR, RTL8152_REQ_REGS, addr, index)
}
