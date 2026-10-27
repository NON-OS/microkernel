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

//! Loading an address space on this CPU.

/// Run on the tables at `root`, which `asid` names. The root comes from the
/// directory, which names an address space from `create_address_space` until
/// `cleanup_address_space` takes it, so no lock is held for the load.
///
/// The root does not pack the asid: `AddressSpace::new` stores the raw
/// page-table frame, so the low bits are zero and the register is loaded
/// untagged, which is correct but means tagged invalidation does nothing for
/// the switch. The asid is still tracked on the CPU below, where the
/// shootdown broadcaster does use it.
///
/// No manager-wide "active table" is written any more: on more than one CPU
/// it was whichever CPU switched last. Walks of the running space read the
/// root register (protection/pte/descend.rs, translation/leaf.rs).
pub(in crate::memory::paging::manager) fn load_root(asid: u32, root: u64) {
    // Recorded on this cpu before CR3 is loaded, and fenced against the
    // broadcaster's fence: set after, a shootdown between the load and the
    // store would skip a cpu already caching the entries it replaces.
    crate::smp::percpu::set_active_asid(asid);
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
    crate::arch::paging::write_root(root, (root & 0xFFF) as u16);
}
