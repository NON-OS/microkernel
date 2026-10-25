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

//! The executing CPU's dense index, where the arch has one to give.

/// Below MAX_CPUS, the boot CPU's 0: aarch64's roster index, whose hardware
/// id is an affinity, sparse across clusters. `None` elsewhere, which
/// leaves a caller to what it did before.
#[inline]
pub fn dense_cpu_index() -> Option<usize> {
    #[cfg(target_arch = "aarch64")]
    return Some(crate::arch::aarch64::cpu::cpu_id());
    #[cfg(not(target_arch = "aarch64"))]
    None
}
