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

//! Which controller this is, by PCI identity, and what that says it carries.

use crate::constants::{PCI_SUBCLASS_AUDIO, PCI_SUBCLASS_HDA, PCI_VENDOR_INTEL};

pub const PCI_VENDOR_AMD: u16 = 0x1022;
pub const PCI_VENDOR_ATI: u16 = 0x1002;
pub const PCI_VENDOR_NVIDIA: u16 = 0x10de;
const PCI_SUBCLASS_MULTIMEDIA_OTHER: u8 = 0x80;

/// `AZX_DRIVER_SKL` in hda_intel.c (Linux 6.10).
const SKL_FAMILY: &[u16] = &[
    0xa170, 0x9d70, 0xa171, 0x9d71, 0xa2f0, 0xa348, 0x9dc8, 0x02c8, 0x06c8, 0xf1c8, 0xa3f0, 0xf0c8,
    0x34c8, 0x3dc8, 0x38c8, 0x4dc8, 0xa0c8, 0x43c8, 0x490d, 0x4f90, 0x4f91, 0x4f92, 0x7ad0, 0x51c8,
    0x51c9, 0x51cd, 0x51cc, 0x54c8, 0x4b55, 0x4b58, 0x7a50, 0x51ca, 0x51cb, 0x51ce, 0x51cf, 0x7e28,
    0xe2f7, 0xa828, 0x7f50, 0x7728, 0x5a98, 0x3198, 0x98c8,
];
/// Intel's discrete graphics cards, whose controllers carry HDMI only.
const INTEL_DISCRETE: &[u16] = &[0x490d, 0x4f90, 0x4f91, 0x4f92, 0xe2f7];

pub fn skl_family(vendor: u16, device: u16) -> bool {
    vendor == PCI_VENDOR_INTEL && SKL_FAMILY.contains(&device)
}

/// A controller that can route its audio through a DSP instead of a codec.
pub fn dsp_capable(vendor: u16, device: u16, subclass: u8) -> bool {
    skl_family(vendor, device) || (vendor == PCI_VENDOR_INTEL && subclass == PCI_SUBCLASS_AUDIO)
}

/// A PCI multimedia function this driver runs as an HD Audio controller:
/// subclass 0x03 from any vendor, or 0x01 from Intel, whose controllers
/// report that class while their DSP is enabled and still run as HD Audio.
pub const fn hda_controller(vendor: u16, subclass: u8) -> bool {
    subclass == PCI_SUBCLASS_HDA || (subclass == PCI_SUBCLASS_AUDIO && vendor == PCI_VENDOR_INTEL)
}

/// A graphics card's audio function: HDMI and DisplayPort only.
pub fn graphics_audio(vendor: u16, device: u16) -> bool {
    vendor == PCI_VENDOR_ATI
        || vendor == PCI_VENDOR_NVIDIA
        || (vendor == PCI_VENDOR_INTEL && INTEL_DISCRETE.contains(&device))
}

/// AMD's audio coprocessor.
pub const fn amd_acp(vendor: u16, class: u8, subclass: u8) -> bool {
    vendor == PCI_VENDOR_AMD && class == 0x04 && subclass == PCI_SUBCLASS_MULTIMEDIA_OTHER
}

/// Whether playback position is read from the DMA position buffer. Intel
/// controllers report it there (`POS_FIX_SKL`, and `POS_FIX_AUTO`'s first
/// choice before it); everything else is read by LPIB.
pub fn position_buffer(vendor: u16) -> bool {
    vendor == PCI_VENDOR_INTEL
}
