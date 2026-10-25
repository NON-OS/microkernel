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

//! The page steps the model's unmap and repoint stand for, read (L79): an
//! unmap holds its claim, bound to a name, from before the grant is removed
//! until its pages are gone and its VA is given back (K10); a take-back
//! drains before it asks for the holder's tables; the repoint moves a page
//! only while it still shows its grant's frame, in one hold of the paging
//! lock, with one round after it.

use super::glue_text::{body, count, in_order};

const RELEASE: &str = "hardware/broker/mmio/release.rs";
const TAKE_BACK: &str = "hardware/broker/mmio/take_back.rs";
const REMAP: &str = "memory/paging/manager/api/mapping/remap_in_asid.rs";

#[test]
fn an_unmap_holds_its_claim_across_the_removal_and_the_unmap() {
    assert_eq!(
        body(RELEASE, "unmap_grant"),
        "{ let _inside = grant::find(pid, grant_id).and_then(|(d, e)| claim::inside(pid, d, e)); \
         let g = grant::remove(pid, grant_id)?; unmap_one(&g); \
         grant::release_user_va(pid, g.user_va, g.length); Ok(()) }"
    );
}

#[test]
fn a_take_back_drains_first_and_moves_each_grant_from_its_own_frame() {
    let f = body(TAKE_BACK, "take_back");
    in_order(
        &f,
        &[
            "{ let drained = grant::drain_for_device(holder, device_id, epoch);",
            "let Some(asid) = crate::kernel_core::surface_registry::space_of(holder) \
         else { return 0; };",
            "let (va, from) = (VirtAddr::new(g.user_va), PhysAddr::new(g.physical_start));",
            "repoint_user_mmio(asid, va, pages_needed(g.length as usize), from, sink) };",
            "drained.iter().map(moved).sum() }",
        ],
    );
}

#[test]
fn a_page_moves_only_while_it_shows_its_frame_and_the_round_follows_the_lock() {
    let f = body(REMAP, "repoint_user_mmio");
    in_order(
        &f,
        &[
            "let moved = without_interrupts(|| { let mut mgr = lock_responsive(&PAGING_MANAGER);",
            "for i in 0..count {",
            "if mgr.translate_in_asid(asid, va) != Some(PhysAddr::new(from.as_u64() + step)) \
         { continue; }",
            "match mgr.map_page_in_asid(asid, va, to, perms, PageSize::Size4KiB, &PAGING_STATS) {",
            "} moved });",
            "if moved > 0 { flush_tlb_range_smp(base, count, asid); } moved }",
        ],
    );
    assert_eq!(count(&f, "lock_responsive("), 1, "more than one hold of the paging lock");
    assert_eq!(count(&f, "flush_tlb"), 1, "more than one round");
}
