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

use super::msr_regs::*;

#[test]
fn syscall_clears_nested_task_so_iretq_cannot_fault_on_it() {
    assert_eq!(SYSCALL_FMASK & (1 << 14), 1 << 14);
}

#[test]
fn syscall_still_clears_interrupts_trap_direction_and_alignment_check() {
    for bit in [9, 8, 10, 18] {
        assert_ne!(SYSCALL_FMASK & (1 << bit), 0, "bit {bit}");
    }
}

#[test]
fn syscall_clears_nothing_else() {
    assert_eq!(SYSCALL_FMASK, (1 << 9) | (1 << 8) | (1 << 10) | (1 << 18) | (1 << 14));
}
