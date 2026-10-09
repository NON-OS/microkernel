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

//! TSC frequency arithmetic, kept free of CPUID and port access so the host
//! proofs (clock_resolve_proofs) run it on recorded register values.

/// CPU vendor as far as TSC calibration cares.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vendor {
    Intel,
    Amd,
    Other,
}

/// Vendor from the CPUID leaf 0 EBX, EDX, ECX registers.
pub fn vendor_from_leaf0(ebx: u32, edx: u32, ecx: u32) -> Vendor {
    let mut id = [0u8; 12];
    id[0..4].copy_from_slice(&ebx.to_le_bytes());
    id[4..8].copy_from_slice(&edx.to_le_bytes());
    id[8..12].copy_from_slice(&ecx.to_le_bytes());
    match &id {
        b"GenuineIntel" => Vendor::Intel,
        b"AuthenticAMD" | b"HygonGenuine" => Vendor::Amd,
        _ => Vendor::Other,
    }
}

/// Display family and model from CPUID leaf 1 EAX.
pub fn family_model(eax: u32) -> (u32, u32) {
    let base_family = (eax >> 8) & 0xF;
    let base_model = (eax >> 4) & 0xF;
    let family = if base_family == 0xF { base_family + ((eax >> 20) & 0xFF) } else { base_family };
    let model = if base_family == 0x6 || base_family == 0xF {
        base_model | (((eax >> 16) & 0xF) << 4)
    } else {
        base_model
    };
    (family, model)
}

/// Intel Atom Goldmont-D (Denverton): reports no crystal and has no leaf
/// 0x16, so Linux hard-codes its 25 MHz crystal.
const MODEL_ATOM_GOLDMONT_D: u32 = 0x5F;

/// The TSC frequency CPUID enumerates, by Linux's `native_calibrate_tsc`:
///
/// - Intel only; AMD and Hygon do not implement leaf 0x15 and are measured.
/// - Leaf 0x15 gives the TSC/crystal ratio EBX/EAX and, from Gemini Lake and
///   Cannon Lake on, the crystal frequency in ECX. Without the ratio there is
///   no enumerated frequency at all.
/// - Skylake and Kaby Lake client parts report ECX = 0; the crystal is then
///   derived from the leaf 0x16 base frequency and the same ratio.
///
/// `leaf15` is (EAX, EBX, ECX) or None when the maximum leaf is below 0x15;
/// `leaf16_base_mhz` is leaf 0x16 EAX or None when below 0x16.
pub fn cpuid_tsc_hz(
    vendor: Vendor,
    family: u32,
    model: u32,
    leaf15: Option<(u32, u32, u32)>,
    leaf16_base_mhz: Option<u32>,
) -> Option<u64> {
    if vendor != Vendor::Intel {
        return None;
    }
    let (denominator, numerator, crystal_hz) = leaf15?;
    if denominator == 0 || numerator == 0 {
        return None;
    }
    let mut crystal_khz = (crystal_hz / 1000) as u64;
    if crystal_khz == 0 && family == 6 && model == MODEL_ATOM_GOLDMONT_D {
        crystal_khz = 25_000;
    }
    if crystal_khz == 0 {
        if let Some(base_mhz) = leaf16_base_mhz {
            crystal_khz = base_mhz as u64 * 1000 * denominator as u64 / numerator as u64;
        }
    }
    if crystal_khz == 0 {
        return None;
    }
    Some(crystal_khz * numerator as u64 / denominator as u64 * 1000)
}

/// ACPI PM timer frequency (ACPI 6.5 section 4.8.3.3).
pub const PM_TIMER_HZ: u64 = 3_579_545;

/// PM timer ticks between two reads, allowing for one wrap of a 24-bit
/// (TMR_VAL_EXT clear) or 32-bit counter.
pub fn pm_timer_delta(start: u32, end: u32, is_32bit: bool) -> u32 {
    let mask: u32 = if is_32bit { u32::MAX } else { 0x00FF_FFFF };
    end.wrapping_sub(start) & mask
}

/// TSC frequency from a TSC delta measured across `ref_ticks` of a reference
/// clock running at `ref_hz`. None when the reference did not move.
pub fn hz_from_reference(tsc_delta: u64, ref_ticks: u64, ref_hz: u64) -> Option<u64> {
    if ref_ticks == 0 {
        return None;
    }
    let hz = tsc_delta as u128 * ref_hz as u128 / ref_ticks as u128;
    u64::try_from(hz).ok()
}

/// The longest a calibration loop may wait, in TSC ticks, before deciding
/// the reference clock is not running: twice `window_ms` at the fastest TSC
/// the kernel accepts. On a machine whose PIT is gated off (some recent
/// laptops) the loop ends in well under a second instead of polling a dead
/// port for minutes.
pub fn reference_timeout_ticks(window_ms: u64, max_tsc_hz: u64) -> u64 {
    max_tsc_hz / 1000 * window_ms * 2
}
