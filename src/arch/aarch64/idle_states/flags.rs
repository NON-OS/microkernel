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

//! Which optional parts of a suspend context this CPU has (asm/suspend.h).

use super::layout::{FLAG_GICV3, FLAG_MTE2, FLAG_PAN, FLAG_SSBS, FLAG_SVE};
use crate::arch::aarch64::cpu::{has_feature, CpuFeature};
use crate::arch::aarch64::gic::{chosen_version, GicVersion};

/// The optional parts this CPU has, each from its own ID register field.
pub(super) fn flags() -> u64 {
    let mut flags = 0;
    let parts = [
        (chosen_version() == Some(GicVersion::V3), FLAG_GICV3),
        (has_feature(CpuFeature::Pan), FLAG_PAN),
        (has_feature(CpuFeature::Sve), FLAG_SVE),
        (has_feature(CpuFeature::Mte2), FLAG_MTE2),
        (has_feature(CpuFeature::Ssbs), FLAG_SSBS),
    ];
    for (present, bit) in parts {
        if present {
            flags |= bit;
        }
    }
    flags
}
