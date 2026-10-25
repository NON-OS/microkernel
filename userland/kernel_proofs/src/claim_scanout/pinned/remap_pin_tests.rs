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

//! remap_in_asid.rs read whole (L79). The ordered checks beside it left free
//! the lines that step each page's address and frame a page at a time, map
//! the sink user, read and write with nothing to run (an executable one is
//! refused, and nothing would move), count every page moved so the one
//! round follows, and stop the run only on an error, which only freed
//! tables give. Each of those, changed, leaves bootfb's pages on the scanout
//! or its stores faulting, and the model takes them as the kernel's step.

use super::super::glue_text::body;

const REMAP: &str = "memory/paging/manager/api/mapping/remap_in_asid.rs";

#[test]
fn the_repoint_is_line_for_line_the_step_the_model_takes() {
    let want = [
        "{ let perms = PagePermissions::USER | PagePermissions::READ | PagePermissions::WRITE;",
        "let moved = without_interrupts(|| { let mut mgr = lock_responsive(&PAGING_MANAGER);",
        "let mut moved = 0; for i in 0..count { let step = (i * PAGE_SIZE_4K) as u64;",
        "let va = VirtAddr::new(base.as_u64() + step);",
        "if mgr.translate_in_asid(asid, va) != Some(PhysAddr::new(from.as_u64() + step)) \
         { continue; }",
        "match mgr.map_page_in_asid(asid, va, to, perms, PageSize::Size4KiB, &PAGING_STATS) {",
        "Ok(_owed) => moved += 1, Err(_) => break, } } moved });",
        "if moved > 0 { flush_tlb_range_smp(base, count, asid); } moved }",
    ];
    assert_eq!(body(REMAP, "repoint_user_mmio"), want.join(" "));
}
