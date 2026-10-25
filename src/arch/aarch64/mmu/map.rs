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

use super::l2_pool::l2_under;
use super::state;
use super::{tlb, PageAttributes};

pub fn map_page(virt: u64, phys: u64, attrs: PageAttributes) {
    let index = PageIndex::new(virt);
    unsafe {
        if map_inner(index, phys, &attrs) {
            tlb::flush_tlb_page(virt);
        }
    }
}

/// Only the identity map's first `L3_L1_SPAN` gigabytes have level 3 tables,
/// one per 2 MiB, so a page goes there or nowhere. A block already mapping
/// the 2 MiB, or a table other than that one, means the page would be written
/// where no walk reaches it, and the mapping is refused instead.
unsafe fn map_inner(index: PageIndex, phys: u64, attrs: &PageAttributes) -> bool {
    if index.l0 != 0 || index.l1 >= state::L3_L1_SPAN {
        return false;
    }
    let Some(l2) = l2_under(state::l1(), index.l1) else {
        return false;
    };
    let l3 = state::l3_addr(index.l1, index.l2);
    if !l2.is_valid(index.l2) {
        l2.set_table(index.l2, l3);
    }
    if l2.table_address(index.l2) != Some(l3) {
        return false;
    }
    state::l3(index.l1, index.l2).set_page(index.l3, phys, attrs);
    true
}

pub(super) struct PageIndex {
    pub(super) l0: usize,
    pub(super) l1: usize,
    pub(super) l2: usize,
    pub(super) l3: usize,
}

impl PageIndex {
    pub(super) fn new(virt: u64) -> Self {
        Self {
            l0: ((virt >> 39) & 0x1FF) as usize,
            l1: ((virt >> 30) & 0x1FF) as usize,
            l2: ((virt >> 21) & 0x1FF) as usize,
            l3: ((virt >> 12) & 0x1FF) as usize,
        }
    }
}
