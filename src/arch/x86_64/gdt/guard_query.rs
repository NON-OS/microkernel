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

//! Which pages of the kernel image are missing on purpose.
//!
//! The stack blocks live in .bss, so a check that walks .bss expecting every
//! page to be mapped finds the armed guards and calls them holes. They are
//! the opposite: a guard that is still mapped is the failure. Anything asking
//! whether the image is intact has to be able to tell the two apart, and only
//! this module knows where the guards are.

use core::ptr::addr_of;

use super::guarded_stack::{GuardedStack, GUARD_BYTES};
use super::percpu_stacks::{CpuStacks, AP_STACKS, BSP_STACKS};

fn is_guard_of(stack: &GuardedStack, va: u64) -> bool {
    let base = stack.guard_base();
    (base..base + GUARD_BYTES as u64).contains(&va)
}

fn block_holds(stacks: &CpuStacks, va: u64) -> bool {
    stacks.ist.iter().any(|slot| is_guard_of(slot, va)) || is_guard_of(&stacks.kernel, va)
}

/// True when `va` falls in a guard page under one of this kernel's stacks,
/// whether or not it has been armed yet.
pub fn is_stack_guard(va: u64) -> bool {
    // SAFETY: ek@nonos.systems - both statics live for the whole run and are
    // read here only for the addresses of their guard fields. No stack in use
    // is reached through this path; those go through RSP and the TSS.
    let bsp = unsafe { &*addr_of!(BSP_STACKS) };
    if block_holds(bsp, va) {
        return true;
    }
    // SAFETY: ek@nonos.systems - as above, and the slice is read, never written.
    let aps = unsafe { &*addr_of!(AP_STACKS) };
    aps.iter().any(|stacks| block_holds(stacks, va))
}
