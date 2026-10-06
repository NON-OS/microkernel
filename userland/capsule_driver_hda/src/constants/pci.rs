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

pub const CLASS_AUDIO: u32 = 0x0050;
pub const HDA_BAR_INDEX: u8 = 0;
pub const HDA_BAR_MIN_SIZE: u64 = 0x1000;

pub const PCI_VENDOR_INTEL: u16 = 0x8086;
pub const PCI_CLASS_MULTIMEDIA: u8 = 0x04;
/// Multimedia audio controller: what Intel's controllers from Skylake on
/// report while their audio DSP is enabled in firmware.
pub const PCI_SUBCLASS_AUDIO: u8 = 0x01;
/// High Definition Audio controller.
pub const PCI_SUBCLASS_HDA: u8 = 0x03;

/// Intel PCH configuration registers Linux sets in `azx_init_pci` and
/// `hda_intel_init_chip` (sound/pci/hda/hda_intel.c).
pub const PCI_CFG_TCSEL: u32 = 0x44;
pub const PCI_CFG_CGCTL: u32 = 0x48;
pub const PCI_CFG_DEVC: u32 = 0x78;
pub const TCSEL_MASK: u32 = 0x07;
pub const CGCTL_MISCBDCGE: u32 = 1 << 6;
pub const DEVC_NOSNOOP: u32 = 1 << 11;
