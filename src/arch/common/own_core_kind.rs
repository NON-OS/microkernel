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

use crate::smp::topology::CoreKind;

/// This CPU's core kind on a part whose cores differ, read on that CPU.
///
/// x86_64: Intel hybrid parts, through CPUID. aarch64 and riscv64 report
/// none: no board the kernel supports there mixes core kinds.
pub(crate) fn own_core_kind() -> Option<CoreKind> {
    #[cfg(target_arch = "x86_64")]
    return crate::arch::x86_64::cpu::own_core_kind();

    #[cfg(not(target_arch = "x86_64"))]
    return None;
}
