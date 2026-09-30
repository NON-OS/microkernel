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

use super::super::constants::{GUARD_PAGES, IST_STACK_SIZE, KSTACK_SIZE, PAGE_SIZE};

/* Offset of stack slot `slot` from the start of a CPU's stack area. Slot 0 is
the kernel stack and slot i + 1 is IST stack i. A guard sits below the first
stack, between every two stacks and above the last, so no stack's guard is a
page of another stack. */
pub(crate) const fn stack_slot_offset(slot: usize) -> u64 {
    let guard = (GUARD_PAGES * PAGE_SIZE) as u64;
    if slot == 0 {
        return guard;
    }
    let ist = slot as u64 - 1;
    guard + KSTACK_SIZE as u64 + guard + ist * (IST_STACK_SIZE as u64 + guard)
}
