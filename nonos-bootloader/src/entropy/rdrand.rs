// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

// The CPU's own generator, as `arch::cpu` reaches it: RDSEED and RDRAND on
// x86_64, RNDRRS and RNDR on aarch64, each only where the CPU reports it.

/// A seed-grade word from the CPU, or None where it has none or it failed.
#[inline(always)]
pub fn rdseed64() -> Option<u64> {
    crate::arch::cpu::hw_seed()
}

/// A word from the CPU's generator, or None where it has none or it failed.
#[inline(always)]
pub fn rdrand64() -> Option<u64> {
    crate::arch::cpu::hw_random()
}
