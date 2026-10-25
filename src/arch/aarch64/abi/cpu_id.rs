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

/// This CPU's hardware id: its packed affinity, all four levels, the value
/// the GIC compares and the descriptors are filled from (`local_id`). It
/// used to leave Aff3 out, so on a part with Aff3 set it matched no
/// descriptor. Not an index (`cpu::cpu_id` is).
#[inline(always)]
pub(super) fn current() -> u32 {
    crate::arch::aarch64::cpu::cpu_affinity()
}
