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

//! Giving DMA grants back, one by hand or a device's at its release, and
//! the grants a device can still reach. A grant given back is freed, so a
//! model that follows a bus address reaches only the live ones, as the
//! device does through the IOMMU.

use std::alloc::dealloc;

use crate::calls::Call;
use crate::dma::{Grant, GRANTS};
use crate::log::note;

const EINVAL: i64 = -22;

pub fn mk_dma_unmap(grant_id: u64) -> i64 {
    let gone = GRANTS.with(|g| {
        let mut all = g.borrow_mut();
        all.iter().position(|x| x.id == grant_id).map(|at| all.swap_remove(at))
    });
    let Some(grant) = gone else {
        note(Call::StaleUnmap);
        return EINVAL;
    };
    note(Call::DmaUnmap);
    free(grant);
    0
}

/// Every grant of `device`, freed as the kernel's release frees them.
pub(crate) fn free_device(device: u64) {
    let gone: Vec<Grant> = GRANTS.with(|g| {
        let (gone, kept) = g.borrow_mut().drain(..).partition(|x| x.device == device);
        *g.borrow_mut() = kept;
        gone
    });
    gone.into_iter().for_each(free);
}

fn free(grant: Grant) {
    // SAFETY: `mk_dma_map` allocated it with this layout, and it is freed
    // once, having just left the table.
    unsafe { dealloc(grant.host as *mut u8, grant.layout) };
}

/// The live grants as a device reaches them: bus address, host address and
/// length of each.
pub fn live_dma() -> Vec<(u64, usize, usize)> {
    GRANTS.with(|g| g.borrow().iter().map(|x| (x.bus, x.host, x.layout.size())).collect())
}
