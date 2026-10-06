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

use super::super::asm::{cpuid, cpuid_max_leaf};
use super::super::constants::{MAX_FREQUENCY, MIN_FREQUENCY};
use super::math::{cpuid_tsc_hz, family_model, vendor_from_leaf0};

/// The TSC frequency CPUID enumerates, or None (AMD, older Intel, or a part
/// that reports no ratio), in which case the TSC has to be measured against
/// the PIT or the ACPI PM timer. See `math::cpuid_tsc_hz`.
pub fn get_cpuid_frequency() -> Option<u64> {
    let max_leaf = cpuid_max_leaf();
    let (_, ebx0, ecx0, edx0) = cpuid(0, 0);
    let vendor = vendor_from_leaf0(ebx0, edx0, ecx0);
    let (eax1, _, _, _) = if max_leaf >= 1 { cpuid(1, 0) } else { (0, 0, 0, 0) };
    let (family, model) = family_model(eax1);
    let leaf15 = (max_leaf >= 0x15).then(|| {
        let (a, b, c, _) = cpuid(0x15, 0);
        (a, b, c)
    });
    let leaf16 = (max_leaf >= 0x16).then(|| cpuid(0x16, 0).0);
    cpuid_tsc_hz(vendor, family, model, leaf15, leaf16)
        .filter(|hz| (MIN_FREQUENCY..=MAX_FREQUENCY).contains(hz))
}
