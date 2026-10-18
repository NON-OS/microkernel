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

//! Loading a user root into TTBR0_EL1 with its ASID, and giving the ASID
//! back when the root dies.

use core::arch::asm;

use spin::Mutex;

use super::pool::{Pool, Tag};
use crate::arch::aarch64::mmu::{control, state, ttbr};

static POOL: Mutex<Option<Pool>> = Mutex::new(None);

/// Run `f` on the pool with IRQs masked: a switch can be reached from an
/// interrupt, and one taken while this CPU holds the lock would spin on it.
fn with_pool<T>(f: impl FnOnce(&mut Pool) -> T) -> T {
    let daif: u64;
    // SAFETY: reading DAIF and masking IRQ at EL1; restored below.
    unsafe { asm!("mrs {}, daif", "msr daifset, #2", out(reg) daif, options(nostack)) };
    let out = {
        let mut held = POOL.lock();
        let bits = if control::has_16_bit_asid() { 16 } else { 8 };
        f(held.get_or_insert_with(|| Pool::new(bits)))
    };
    // SAFETY: puts back exactly the DAIF read above.
    unsafe { asm!("msr daif, {}", in(reg) daif, options(nostack)) };
    out
}

/// Point TTBR0_EL1 at `root`. The kernel's own root runs as ASID 0, as boot
/// loaded it. A root on the shared ASID drops that ASID's entries from this
/// CPU after the load: another root may have left them here. No other
/// root's entries are touched, so nothing else pays for the switch.
pub fn switch_root(root: u64) {
    // SAFETY: the kernel root's address, read only.
    let kernel = root == unsafe { state::l0_addr() };
    let tag = if kernel { Tag::Own(0) } else { with_pool(|p| p.tag(root)) };
    match tag {
        Tag::Own(asid) => ttbr::set_ttbr0(root, asid),
        Tag::Shared(asid) => {
            ttbr::set_ttbr0(root, asid);
            // SAFETY: TLB maintenance for this CPU's own entries of one ASID.
            unsafe {
                asm!("tlbi aside1, {}", "dsb nsh", "isb", in(reg) u64::from(asid) << 48, options(nostack));
            }
        }
    }
}

/// `root` is gone and no CPU has it loaded. Its ASID is invalidated on every
/// CPU before it can be handed out again, under the same lock.
pub fn retire_root(root: u64) {
    with_pool(|pool| {
        if let Some(asid) = pool.retire(root) {
            crate::arch::aarch64::mmu::flush_tlb_asid(asid);
        }
    });
}
