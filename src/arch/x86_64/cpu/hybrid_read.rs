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

use super::hybrid::{core_type, CoreType, LEAF7_EDX_HYBRID, LEAF_CORE_TYPE};
use crate::arch::x86_64::boot::cpu_ops::cpuid_count;
use crate::smp::topology::CoreKind;

/// On the CPU asked about: leaf 1AH answers for the core that executes it.
/// None on a part that is not hybrid, where every core is alike.
pub(crate) fn own_core_kind() -> Option<CoreKind> {
    let max = cpuid_count(0, 0).0;
    if max < 7 || cpuid_count(7, 0).3 & LEAF7_EDX_HYBRID == 0 {
        return None;
    }
    if max < LEAF_CORE_TYPE {
        return Some(CoreKind::Unknown);
    }
    Some(match core_type(cpuid_count(LEAF_CORE_TYPE, 0).0) {
        CoreType::Core => CoreKind::Performance,
        CoreType::Atom => CoreKind::Efficiency,
        CoreType::Other => CoreKind::Unknown,
    })
}
