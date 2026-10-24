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

//! The host controller's I/O registers, offsets from its I/O BAR.

pub const USBCMD: u16 = 0x00;
pub const USBSTS: u16 = 0x02;
pub const USBINTR: u16 = 0x04;
pub const FRNUM: u16 = 0x06;
/// The frame list's base, 32 bits, 4 KiB aligned.
pub const FLBASEADD: u16 = 0x08;
/// Root port 1; port 2 follows two bytes up.
pub const PORTSC1: u16 = 0x10;
pub const ROOT_PORTS: u8 = 2;
/// The registers above end before this: the BAR holds at least 32 bytes.
pub const IO_SPAN: u64 = 0x20;

/// USBCMD: run, host reset (self-clearing), global reset (held by
/// software), configure flag, and 64-byte max packet for bandwidth
/// reclamation.
pub const CMD_RS: u16 = 1 << 0;
pub const CMD_HCRESET: u16 = 1 << 1;
pub const CMD_GRESET: u16 = 1 << 2;
pub const CMD_CF: u16 = 1 << 6;
pub const CMD_MAXP: u16 = 1 << 7;

/// USBSTS, each cleared by writing it back: transfer interrupt, error
/// interrupt, resume detect, system and process errors; and halted.
pub const STS_USBINT: u16 = 1 << 0;
pub const STS_ERROR: u16 = 1 << 1;
pub const STS_RD: u16 = 1 << 2;
pub const STS_HSE: u16 = 1 << 3;
pub const STS_HCPE: u16 = 1 << 4;
pub const STS_HCH: u16 = 1 << 5;
pub const STS_ALL: u16 = STS_USBINT | STS_ERROR | STS_RD | STS_HSE | STS_HCPE | STS_HCH;

/// PORTSC: connected and its change, enabled and its change (the changes
/// cleared by writing them back), low-speed device attached, port reset.
pub const PORT_CCS: u16 = 1 << 0;
pub const PORT_CSC: u16 = 1 << 1;
pub const PORT_PE: u16 = 1 << 2;
pub const PORT_PEC: u16 = 1 << 3;
pub const PORT_LSDA: u16 = 1 << 8;
pub const PORT_PR: u16 = 1 << 9;
/// The write-back-to-clear bits, kept out of any write meant for others.
pub const PORT_WC: u16 = PORT_CSC | PORT_PEC;
