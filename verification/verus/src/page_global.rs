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

//! A user page is never GLOBAL (P-06). With CR4.PGE on, a GLOBAL entry
//! survives the CR3 write that leaves an address space, so a GLOBAL user
//! entry would stay readable to the next process. `to_pte_g` is `to_pte` with
//! the GLOBAL request, as `PagePermissions::to_pte_flags` and
//! `PageTableEntry::to_raw` encode it: kept on a kernel page, dropped on a
//! user page. The aarch64 encoders (the manager's `leaf` and
//! `PageAttributes`) set nG on every EL0 page by the same rule, which
//! `userland/arch_paging_proofs` checks by execution; the model is
//! arch-neutral.

use vstd::prelude::*;
use crate::page_permissions::*;

verus! {

pub open spec fn global_bit() -> u64 { 1u64 << 8 }

pub open spec fn to_pte_g(
    phys: u64,
    writable: bool,
    user: bool,
    executable: bool,
    cache_disabled: bool,
    global: bool,
) -> u64 {
    to_pte(phys, writable, user, executable, cache_disabled)
    | if global && !user { global_bit() } else { 0 }
}

pub proof fn a_user_entry_is_never_global(phys: u64, w: bool, x: bool, cd: bool, g: bool)
    ensures
        !(has(to_pte_g(phys, w, true, x, cd, g), global_bit())),
{
    assert(!(has(to_pte_g(phys, w, true, x, cd, g), global_bit()))) by (bit_vector);
}

pub proof fn a_kernel_entry_keeps_global_when_asked(phys: u64, w: bool, x: bool, cd: bool, g: bool)
    ensures
        has(to_pte_g(phys, w, false, x, cd, g), global_bit()) == g,
{
    assert(has(to_pte_g(phys, w, false, x, cd, g), global_bit()) == g) by (bit_vector);
}

pub proof fn global_changes_no_other_bit(phys: u64, w: bool, u: bool, x: bool, cd: bool, g: bool)
    ensures
        to_pte_g(phys, w, u, x, cd, g) & !global_bit() == to_pte(phys, w, u, x, cd) & !global_bit(),
{
    assert(to_pte_g(phys, w, u, x, cd, g) & !global_bit() == to_pte(phys, w, u, x, cd) & !global_bit())
        by (bit_vector);
}

} // verus!
