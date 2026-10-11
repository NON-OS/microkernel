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

//! The SMBus host registers in the I/O BAR, and the HOSTC config byte, as
//! Linux's i2c-i801.c names them (Intel ICH and PCH datasheets, "SMBus
//! I/O Registers").

pub const HSTSTS: u16 = 0x00;
pub const HSTCNT: u16 = 0x02;
pub const HSTCMD: u16 = 0x03;
pub const HSTADD: u16 = 0x04;
pub const HSTDAT0: u16 = 0x05;
pub const HSTDAT1: u16 = 0x06;
pub const BLKDAT: u16 = 0x07;
pub const AUXCTL: u16 = 0x0D;
/// The registers above end before this: the BAR must hold at least these.
pub const IO_SPAN: u64 = 0x10;

pub const STS_HOST_BUSY: u8 = 0x01;
pub const STS_INTR: u8 = 0x02;
pub const STS_DEV_ERR: u8 = 0x04;
pub const STS_BUS_ERR: u8 = 0x08;
pub const STS_FAILED: u8 = 0x10;
/// Reading HSTSTS sets it: the hardware semaphore firmware and ACPI share.
/// Writing it back releases the host.
pub const STS_INUSE: u8 = 0x40;
pub const STS_BYTE_DONE: u8 = 0x80;
pub const STS_ERROR: u8 = STS_DEV_ERR | STS_BUS_ERR | STS_FAILED;
/// Every bit a transaction leaves behind, cleared by writing it back.
pub const STS_FLAGS: u8 = STS_BYTE_DONE | STS_FAILED | STS_BUS_ERR | STS_DEV_ERR | STS_INTR;

pub const CNT_KILL: u8 = 0x02;
pub const CNT_START: u8 = 0x40;
pub const CNT_QUICK: u8 = 0x00;
pub const CNT_BYTE: u8 = 0x04;
pub const CNT_BYTE_DATA: u8 = 0x08;
pub const CNT_WORD_DATA: u8 = 0x0C;
pub const CNT_BLOCK_DATA: u8 = 0x14;

/// AUXCTL: the 32-byte block buffer, ICH4 and later.
pub const AUX_E32B: u8 = 0x02;
pub const AUX_CRC: u8 = 0x01;

/// Config byte 0x40, HOSTC: the host enable, and I2C mode, which Linux
/// clears for SMBus timing.
pub const HOSTC: u32 = 0x40;
pub const HOSTC_HST_EN: u16 = 0x01;
pub const HOSTC_I2C_EN: u16 = 0x04;
