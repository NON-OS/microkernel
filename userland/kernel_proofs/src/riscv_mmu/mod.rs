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

/*
 * RISC-V translation must not fail open.
 *
 * The kernel's MmuMode and PteFlags are included by path. satp_mode encoded
 * Unknown as 0, Bare, so a switch that re-encoded an unreadable live mode
 * turned translation off; is_leaf ignored V and accepted the reserved W
 * without R encoding. The checks below fail against that code.
 */

#[path = "../../../../src/arch/riscv64/mmu/attributes/flags.rs"]
pub mod flags;
#[path = "../../../../src/arch/riscv64/mmu/mode.rs"]
pub mod mode;
mod tests;
