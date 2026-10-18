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

//! The PCI configuration bits a controller needs changed, and the masked update.

use super::{skl_family, PCI_VENDOR_AMD};
use crate::constants::PCI_VENDOR_INTEL;

/// The PCI configuration writes Linux's hda_intel.c makes for a controller,
/// each within the bits the broker lets a driver change.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct PciQuirks {
    /// TCSEL (0x44) bits 0-2 to 0: `azx_init_pci` on every Intel part (none
    /// carries `AZX_DCAPS_NO_TCSEL`); a nonzero traffic class gives playback
    /// static on some codecs.
    pub clear_tcsel: bool,
    /// DEVC (0x78) NOSNOOP to 0, so the controller snoops the rings and the
    /// stream (`AZX_SNOOP_TYPE_SCH`, the PCH and Skylake presets).
    pub clear_nosnoop: bool,
    /// CGCTL (0x48) MISCBDCGE off around the controller reset and back on
    /// after it (`hda_intel_init_chip`, Skylake and later, Gemini Lake among
    /// them). With the clock gated through the reset the codecs may never
    /// show in STATESTS.
    pub gate_cgctl: bool,
    /// MISC_CNTR2 (0x42) bits 0-2 to ENABLE_SNOOP (`AZX_SNOOP_TYPE_ATI`, the
    /// AMD southbridge preset Ryzen's controllers use).
    pub amd_snoop: bool,
}

pub const AMD_MISC_CNTR2: u32 = 0x42;
pub const AMD_SNOOP_MASK: u16 = 0x07;
pub const AMD_ENABLE_SNOOP: u16 = 0x02;

pub fn pci_quirks(vendor: u16, device: u16) -> PciQuirks {
    let skl = skl_family(vendor, device);
    PciQuirks {
        clear_tcsel: vendor == PCI_VENDOR_INTEL,
        clear_nosnoop: skl,
        gate_cgctl: skl,
        amd_snoop: vendor == PCI_VENDOR_AMD,
    }
}

/// `current` with the bits under `mask` replaced by `want`'s.
pub const fn with_bits(current: u16, mask: u16, want: u16) -> u16 {
    (current & !mask) | (want & mask)
}
