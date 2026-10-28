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

//! DMA grants in host memory, page aligned and zeroed as the kernel's are,
//! each at a bus address of its own, never its host address: a model reaches
//! a grant through `live_dma`, as a device does through the IOMMU. A map
//! that asks `MK_DMA_MAP_DMA32` lies below 4 GiB; any other lies above it,
//! its high half not zero, as the kernel may place one once low memory runs
//! out (`src/hardware/broker/dma/map/alloc.rs`, `take`), so a driver that
//! drops an upper half names no grant.

use std::alloc::{alloc_zeroed, Layout};
use std::cell::{Cell, RefCell};

use crate::calls::Call;
use crate::types::DmaMapOut;
use crate::MK_DMA_MAP_DMA32;
use crate::{log::note, refuse::short};

const PAGE: usize = 4096;
/// Where grants start on the bus: DMA32 ones, and all others.
const LOW_BASE: u64 = 0x4000_0000;
const HIGH_BASE: u64 = 0x7_0000_0000;
/// Room between grants on the bus; no driver asks for more in one.
const BUS_STRIDE: u64 = 1 << 20;
const EINVAL: i64 = -22;
const ENOMEM: i64 = -12;

pub(crate) struct Grant {
    pub(crate) id: u64,
    pub(crate) device: u64,
    pub(crate) host: usize,
    pub(crate) bus: u64,
    pub(crate) layout: Layout,
}

thread_local! {
    pub(crate) static GRANTS: RefCell<Vec<Grant>> = const { RefCell::new(Vec::new()) };
    static NEXT: Cell<u64> = const { Cell::new(1) };
}

pub fn mk_dma_map(device: u64, _epoch: u64, length: u64, flags: u32, out: &mut DmaMapOut) -> i64 {
    if !note(Call::DmaMap) {
        return ENOMEM;
    }
    let size = (length as usize).div_ceil(PAGE) * PAGE;
    let layout = match Layout::from_size_align(size, PAGE) {
        Ok(l) if size != 0 && size as u64 <= BUS_STRIDE => l,
        _ => return EINVAL,
    };
    // SAFETY: the layout's size is not zero.
    let host = unsafe { alloc_zeroed(layout) } as usize;
    if host == 0 {
        return ENOMEM;
    }
    let id = NEXT.with(|n| n.replace(n.get() + 1));
    let base = if flags & MK_DMA_MAP_DMA32 != 0 { LOW_BASE } else { HIGH_BASE };
    let bus = base + id * BUS_STRIDE;
    GRANTS.with(|g| g.borrow_mut().push(Grant { id, device, host, bus, layout }));
    *out = DmaMapOut { user_va: host as u64, device_addr: bus, length: short(size), grant_id: id };
    0
}
