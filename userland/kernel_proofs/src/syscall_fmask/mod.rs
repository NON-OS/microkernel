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
 * The flags SYSCALL clears on entry, included by path. NT left set let any
 * capsule turn the kernel's own IRETQ into #GP(0) and a panic.
 */

#[path = "../../../../src/arch/x86_64/syscall/msr_regs.rs"]
pub mod msr_regs;
#[cfg(test)]
mod tests;
