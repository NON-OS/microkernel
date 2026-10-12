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

use alloc::vec::Vec;

use spin::Mutex;

use crate::kernel_core::surface_registry::attach_map::snapshot::has_foreign_holder;
use crate::kernel_core::surface_registry::types::SurfaceHandle;
use crate::smp::lock_responsive;

use super::dma_rule::{due, DmaOrphan};
use super::gate::GATE;

/* DMA grants the registry holds for their surfaces' other holders (K15). */
pub(super) static HELD: Mutex<Vec<DmaOrphan<SurfaceHandle>>> = Mutex::new(Vec::new());

/* Give back every held grant no other process maps any more. */
pub(super) fn reclaim_dma() {
    let ready = {
        let _gate = lock_responsive(&GATE);
        due(&mut HELD.lock(), has_foreign_holder)
    };
    for h in ready {
        crate::hardware::broker::dma::free_frames(h.start, h.len);
    }
}
