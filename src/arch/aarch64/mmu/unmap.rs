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

use super::l2_pool::existing_l2;
use super::map::PageIndex;
use super::{state, tlb};

pub fn unmap_page(virt: u64) {
    let index = PageIndex::new(virt);
    unsafe {
        if unmap_inner(index) {
            tlb::flush_tlb_page(virt);
        }
    }
}

unsafe fn unmap_inner(index: PageIndex) -> bool {
    if index.l0 != 0 || index.l1 >= state::L3_L1_SPAN {
        return false;
    }
    let Some(l2) = existing_l2(state::l1(), index.l1) else {
        return false;
    };
    // Clearing a block for one page would take the 511 around it too, so a
    // page inside a block, like one under a table this file did not install,
    // is not this function's to unmap.
    if l2.table_address(index.l2) != Some(state::l3_addr(index.l1, index.l2)) {
        return false;
    }
    state::l3(index.l1, index.l2).clear_entry(index.l3);
    true
}
