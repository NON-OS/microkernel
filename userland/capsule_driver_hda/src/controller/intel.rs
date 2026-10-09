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
//! What differs between the controllers this driver meets, by PCI identity,
//! from the device table in sound/pci/hda/hda_intel.c.
//!
//! - Intel from Skylake on (`AZX_DRIVER_SKL`, Apollo Lake and Gemini Lake
//!   among them) carries an audio DSP, may report PCI class 0x0401 when
//!   firmware enabled it, has a multi-link capability whose link clock Linux
//!   checks after reset (`intel_init_lctl`), and is read by the DMA position
//!   buffer for playback (`POS_FIX_SKL`).
//! - Apollo Lake (8086:5a98) alone has its DMA FIFO threshold lowered
//!   (`bxt_reduce_dma_latency`); Gemini Lake (8086:3198) does not.
//! - AMD's chipset controllers (1022:15e3, 1457, 1487) and every ATI, AMD or
//!   NVIDIA graphics controller are read by LPIB (`AZX_DCAPS_POSFIX_LPIB`,
//!   `POS_FIX_FIFO`); a graphics card's controller carries HDMI only.
//! - AMD's audio coprocessor (1022:15e2, class 0x0480) is not an HD Audio
//!   controller at all.

use super::wait::until;
use crate::clock::pause_ms;
use crate::constants::{PCI_SUBCLASS_AUDIO, PCI_SUBCLASS_HDA, PCI_VENDOR_INTEL, VS_EM4L};
use crate::regs::Regs;

pub const PCI_VENDOR_AMD: u16 = 0x1022;
pub const PCI_VENDOR_ATI: u16 = 0x1002;
pub const PCI_VENDOR_NVIDIA: u16 = 0x10de;
const PCI_SUBCLASS_MULTIMEDIA_OTHER: u8 = 0x80;

/// `AZX_DRIVER_SKL` in hda_intel.c (Linux 6.10).
const SKL_FAMILY: &[u16] = &[
    0xa170, 0x9d70, 0xa171, 0x9d71, 0xa2f0, 0xa348, 0x9dc8, 0x02c8, 0x06c8, 0xf1c8, 0xa3f0,
    0xf0c8, 0x34c8, 0x3dc8, 0x38c8, 0x4dc8, 0xa0c8, 0x43c8, 0x490d, 0x4f90, 0x4f91, 0x4f92,
    0x7ad0, 0x51c8, 0x51c9, 0x51cd, 0x51cc, 0x54c8, 0x4b55, 0x4b58, 0x7a50, 0x51ca, 0x51cb,
    0x51ce, 0x51cf, 0x7e28, 0xe2f7, 0xa828, 0x7f50, 0x7728, 0x5a98, 0x3198, 0x98c8,
];
/// Intel's discrete graphics cards, whose controllers carry HDMI only.
const INTEL_DISCRETE: &[u16] = &[0x490d, 0x4f90, 0x4f91, 0x4f92, 0xe2f7];
const APL: u16 = 0x5a98;

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

/// `bxt_reduce_dma_latency`: keep only VS_EM4L bits 21:20.
pub fn reduce_dma_latency(regs: Regs, vendor: u16, device: u16, bar_size: u64) {
    if vendor != PCI_VENDOR_INTEL || device != APL || bar_size < (VS_EM4L as u64 + 4) {
        return;
    }
    unsafe {
        let v = regs.r32(VS_EM4L);
        regs.w32(VS_EM4L, v & (0x3 << 20));
    }
}

const LLCH: u32 = 0x14;
const CAP_ID_ML: u32 = 0x2;
const ML_LCAP: u32 = 0x40;
const ML_LCTL: u32 = 0x44;
const LCTL_SCF: u32 = 0xf;
const LCTL_SPA: u32 = 1 << 16;
const LCTL_CPA: u32 = 1 << 23;
const MAX_CAPS: u32 = 10;

/// The multi-link capability's offset, walking the linked capability list
/// from LLCH as `snd_hdac_bus_parse_capabilities` does.
pub fn multilink(regs: Regs, bar_size: u64) -> Option<u32> {
    let mut off = unsafe { regs.r16(LLCH) } as u32;
    let mut n = 0u32;
    while off != 0 && n < MAX_CAPS && (off + ML_LCTL + 4) as u64 <= bar_size {
        let hdr = unsafe { regs.r32(off) };
        if hdr == u32::MAX {
            return None;
        }
        if (hdr >> 16) & 0xfff == CAP_ID_ML {
            return Some(off);
        }
        off = hdr & 0xffff;
        n += 1;
    }
    None
}

/// The link clock Linux picks from LCAP: 24, 48, 12, 96, then 192 MHz.
pub fn preferred_scf(lcap: u32) -> u32 {
    for bit in [2u32, 3, 1, 4, 5] {
        if lcap & (1 << bit) != 0 {
            return bit;
        }
    }
    0
}

/// `intel_init_lctl`: a link left on the 6 MHz clock is stopped, moved to
/// a faster one, and started again.
pub fn init_link_clock(regs: Regs, vendor: u16, device: u16, bar_size: u64) {
    if !skl_family(vendor, device) {
        return;
    }
    let Some(ml) = multilink(regs, bar_size) else { return };
    let lctl = ml + ML_LCTL;
    let val = unsafe { regs.r32(lctl) };
    if val & LCTL_SCF != 0 || ((val & LCTL_SPA) != 0) != ((val & LCTL_CPA) != 0) {
        return;
    }
    let set_power = |on: bool| {
        let v = unsafe { regs.r32(lctl) } & !LCTL_SPA;
        unsafe { regs.w32(lctl, v | if on { LCTL_SPA } else { 0 }) };
        until(1, || (unsafe { regs.r32(lctl) } & LCTL_CPA != 0) == on)
    };
    if set_power(false) {
        pause_ms(1);
        let scf = preferred_scf(unsafe { regs.r32(ml + ML_LCAP) });
        unsafe { regs.w32(lctl, (val & !LCTL_SCF & !LCTL_SPA) | scf) };
    }
    set_power(true);
    pause_ms(1);
}

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
