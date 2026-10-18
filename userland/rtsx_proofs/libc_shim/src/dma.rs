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

//! DMA grants backed by host memory. The driver writes through the user
//! address; a modelled reader reaches the same bytes by device address
//! (`reach`), and only while the grant is live. An unmap gives the device
//! its turn after the grant is gone: what it writes then lands in a freed
//! frame, the race C2 forbids.

use std::cell::Cell;

use crate::events::note;
use crate::grants::{Grant, GRANTS};
use crate::hook::device_acts;
use crate::types::DmaMapOut;

thread_local! {
    static REFUSE_AFTER: Cell<Option<usize>> = const { Cell::new(None) };
}

/// Refuse the map that comes after `after` more have been granted.
pub fn refuse_dma_map(after: usize) {
    REFUSE_AFTER.with(|r| r.set(Some(after)));
}

pub fn mk_dma_map(device: u64, _epoch: u64, len: u64, _flags: u32, out: &mut DmaMapOut) -> i64 {
    if let Some(left) = REFUSE_AFTER.with(|r| r.get()) {
        REFUSE_AFTER.with(|r| r.set(left.checked_sub(1)));
        if left == 0 {
            note("dma_map refused");
            return -12;
        }
    }
    let ptr = vec![0u8; len as usize].leak().as_mut_ptr();
    let mapped = GRANTS.with(|g| {
        let mut g = g.borrow_mut();
        let (id, iova) = (0x90 + g.len() as u64, 0x1000_0000 + g.len() as u64 * 0x10_0000);
        g.push(Grant { id, device, iova, ptr, len: len as usize, live: true });
        DmaMapOut { user_va: ptr as u64, device_addr: iova, length: len, grant_id: id }
    });
    note(format!("dma_map {:#x}", mapped.grant_id));
    *out = mapped;
    0
}

pub fn mk_dma_unmap(grant_id: u64) -> i64 {
    note(format!("dma_unmap {grant_id:#x}"));
    let found = GRANTS.with(|g| {
        let mut g = g.borrow_mut();
        g.iter_mut().find(|x| x.id == grant_id && x.live).map(|x| x.live = false).is_some()
    });
    device_acts();
    if found {
        0
    } else {
        -2
    }
}
