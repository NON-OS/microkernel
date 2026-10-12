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

//! E0PD (FEAT_E0PD): TCR_EL1.E0PD1 makes every EL0 access to the TTBR1 half
//! fault as a level 0 translation fault, in the time an unmapped address
//! takes, whatever is mapped there (Arm ARM, TCR_EL1.E0PD1). Without it, EL0
//! can tell the kernel's mapped high addresses (the direct map) from unmapped
//! ones by how long a permission fault takes against a translation fault.
//!
//! E0PD0 stays clear: TTBR0 is where EL0 runs.

use crate::arch::aarch64::cpu::{has_feature, CpuFeature};

const E0PD1: u64 = 1 << 56;

/// The bit to add to TCR_EL1, or none on a core without the feature, where
/// bit 56 is RES0.
pub(super) fn bit() -> u64 {
    if has_feature(CpuFeature::E0pd) {
        E0PD1
    } else {
        0
    }
}
