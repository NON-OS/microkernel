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

//! What a user root holds, and what is owed when one dies.

/// `root_pa` is being freed and no CPU has it loaded: whatever the TLB holds
/// under its tag goes before the tag can name another root.
#[inline]
pub fn retire_root(root_pa: u64) {
    #[cfg(target_arch = "aarch64")]
    crate::arch::aarch64::mmu::asid::retire_root(root_pa & 0x0000_FFFF_FFFF_F000);
    #[cfg(not(target_arch = "aarch64"))]
    let _ = root_pa;
}

/// Whether a user root carries the kernel's half of the address space. On
/// x86_64 and riscv64 one root translates both halves, so it must. On
/// aarch64 TTBR1_EL1 translates the kernel's half on its own and a user root
/// is TTBR0's alone: entries copied into its top half would map user-range
/// addresses onto kernel memory, for nothing the kernel reaches through.
pub const USER_ROOT_HOLDS_KERNEL: bool = !cfg!(target_arch = "aarch64");

/// Put into a fresh, zeroed user root what every root carries besides its
/// own mappings: on aarch64, the EPD1 trampoline's slot, where a CPU leaves
/// for EL0 through it (security::epd1). It maps two EL1-only pages and the
/// slot lies above every user address, so teardown never reaches it.
#[inline]
pub fn seed_user_root(root: &mut [u64; 512]) {
    #[cfg(target_arch = "aarch64")]
    if let Some((slot, word)) = crate::arch::aarch64::security::epd1::user_root_word() {
        root[slot] = word;
    }
    #[cfg(not(target_arch = "aarch64"))]
    let _ = root;
}
