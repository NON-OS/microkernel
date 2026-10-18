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

//! Giving a DMA grant back: recorded (`log`), then the host memory freed.

use std::alloc::{dealloc, Layout};

use crate::dma::EINVAL;
use crate::grants::{grants, PAGE};
use crate::log::{record, Call};

pub fn mk_dma_unmap(grant_id: u64) -> i64 {
    record(Call::DmaUnmap(grant_id));
    let grant = {
        let mut all = grants();
        let Some(at) = all.iter().position(|g| g.id == grant_id) else {
            return EINVAL;
        };
        all.swap_remove(at)
    };
    // SAFETY: the grant was allocated by `mk_dma_map` with this layout and is
    // freed once, having just left the table.
    unsafe {
        dealloc(
            grant.host as *mut u8,
            Layout::from_size_align_unchecked(grant.len as usize, PAGE as usize),
        )
    };
    0
}
