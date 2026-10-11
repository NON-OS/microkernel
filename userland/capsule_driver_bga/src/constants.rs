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

//! The adapter's ids and the DISPI register set, as Linux bochs.c and QEMU's
//! stdvga name them.

pub const CLASS_DISPLAY: u32 = 0x0030;
pub const PCI_CLASS_DISPLAY: u8 = 0x03;
pub const VENDOR_QEMU_BOCHS: u16 = 0x1234;
pub const DEVICE_BGA: u16 = 0x1111;
pub const FB_BAR: u8 = 0;
pub const REG_BAR: u8 = 2;
/// The register BAR must reach QEMU's extension block (0x600..0x608).
pub const REG_BAR_MIN: u64 = 0x608;

pub const DISPI_IOPORT_OFFSET: u32 = 0x500;
pub const DISPI_INDEX_ID: u32 = 0;
pub const DISPI_INDEX_XRES: u32 = 1;
pub const DISPI_INDEX_YRES: u32 = 2;
pub const DISPI_INDEX_BPP: u32 = 3;
pub const DISPI_INDEX_ENABLE: u32 = 4;
pub const DISPI_INDEX_BANK: u32 = 5;
pub const DISPI_INDEX_VIRT_WIDTH: u32 = 6;
pub const DISPI_INDEX_VIRT_HEIGHT: u32 = 7;
pub const DISPI_INDEX_X_OFFSET: u32 = 8;
pub const DISPI_INDEX_Y_OFFSET: u32 = 9;
pub const DISPI_INDEX_VIDEO_MEMORY_64K: u32 = 0xA;
/// VBE_DISPI_ID0; every revision keeps the top twelve bits (bochs_hw_init).
pub const DISPI_ID0: u16 = 0xB0C0;
pub const DISPI_BPP_32: u16 = 32;
pub const DISPI_ENABLED: u16 = 0x01;
pub const DISPI_LFB_ENABLED: u16 = 0x40;

/// QEMU's extension registers: their size, then the framebuffer byte order.
pub const QEXT_SIZE: u32 = 0x600;
pub const QEXT_ENDIAN: u32 = 0x604;
pub const QEXT_LITTLE_ENDIAN: u32 = 0x1e1e_1e1e;
/// The VGA ports 0x3c0.. sit in the register BAR from 0x400.
pub const VGA_MMIO_BASE: u32 = 0x400;
pub const VGA_PORT_BASE: u16 = 0x3C0;
pub const VGA_ATT_W: u16 = 0x3C0;
pub const VGA_IS1_RC: u16 = 0x3DA;
/// Attribute controller palette address source: the screen is shown.
pub const VGA_ATT_PAS: u8 = 0x20;

pub const MODE_WIDTH: u32 = 1024;
pub const MODE_HEIGHT: u32 = 768;
pub const BYTES_PER_PIXEL: u32 = 4;
pub const CLEAR_COLOR: u32 = 0x0010_2a3a;
