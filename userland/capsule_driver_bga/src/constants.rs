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

//! The adapter as the PCI bus and the Bochs VBE interface name it.
//!
//! Register facts are the Bochs VBE ("DISPI") interface's, as QEMU's
//! device model implements them (hw/display/vga.c for the registers,
//! hw/display/vga-pci.c for the MMIO BAR layout), recorded here as facts
//! about the device; positions not checked against those files are marked.

pub const CLASS_DISPLAY: u32 = 0x0030;
pub const PCI_CLASS_DISPLAY: u8 = 0x03;
pub const VENDOR_QEMU_BOCHS: u16 = 0x1234;
pub const DEVICE_BGA: u16 = 0x1111;
/// BAR0: video memory, prefetchable, mapped write-combining (L17).
pub const FB_BAR: u8 = 0;
/// BAR2: the MMIO window: the EDID blob at 0, DISPI at 0x500.
pub const REG_BAR: u8 = 2;
/// `MK_MMIO_MAP_WC`, the lead's L17 flag; libc defines it after the relay.
pub const MK_MMIO_MAP_WC: u32 = 1 << 0;

pub const DISPI_IOPORT_OFFSET: u32 = 0x500;
pub const DISPI_INDEX_XRES: u32 = 1;
pub const DISPI_INDEX_YRES: u32 = 2;
pub const DISPI_INDEX_BPP: u32 = 3;
pub const DISPI_INDEX_ENABLE: u32 = 4;
pub const DISPI_INDEX_VIRT_WIDTH: u32 = 6;
pub const DISPI_INDEX_X_OFFSET: u32 = 8;
pub const DISPI_INDEX_Y_OFFSET: u32 = 9;
pub const DISPI_BPP_32: u16 = 32;
pub const DISPI_ENABLED: u16 = 0x01;
pub const DISPI_LFB_ENABLED: u16 = 0x40;
/// Keep video memory across the mode set, so no black frame shows before
/// the new primary is copied in.
pub const DISPI_NOCLEARMEM: u16 = 0x80;

/// The EDID blob QEMU exposes at BAR2+0 when the device has one (edid=on,
/// the default for stdvga and bochs-display): up to 1 KiB (unverified).
pub const EDID_WINDOW: usize = 0x400;
/// The mode the driver sets when firmware left DISPI off and the monitor
/// names nothing better.
pub const FALLBACK_MODE: (u32, u32) = (1024, 768);
