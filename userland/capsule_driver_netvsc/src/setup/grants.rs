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

//! The three coherent DMA grants on the record, for the rings, the receive
//! buffer and the send buffer, and the page numbers the host reaches them by.

use nonos_libc::{mk_device_release, mk_dma_map, DmaMapOut, MK_DMA_MAP_COHERENT};

use super::driver::abandon;
use crate::constants::layout::{PAGE, RECV_BYTES, RING_BYTES, SEND_BYTES};

/// One coherent grant of `bytes` on the record: its address in the driver
/// and its first page number, as the host reaches it.
fn grant(dev: u64, epoch: u64, bytes: u64) -> Option<DmaMapOut> {
    let mut d = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
    if mk_dma_map(dev, epoch, bytes, MK_DMA_MAP_COHERENT, &mut d) < 0 {
        return None;
    }
    if d.length < bytes || !d.device_addr.is_multiple_of(PAGE as u64) {
        abandon_grant(d.grant_id);
        return None;
    }
    Some(d)
}

fn abandon_grant(id: u64) {
    let _ = nonos_libc::mk_dma_unmap(id);
}

pub fn pfns<const N: usize>(d: &DmaMapOut) -> [u64; N] {
    let first = d.device_addr / PAGE as u64;
    core::array::from_fn(|i| first + i as u64)
}

/// All three grants, or none: a grant that fails gives back those made
/// before it and the claim.
pub fn map_all(device_id: u64, epoch: u64) -> Result<([u64; 3], [DmaMapOut; 3]), &'static str> {
    let mut grants = [0u64; 3];
    let mut maps = [None, None, None];
    for (i, bytes) in [RING_BYTES, RECV_BYTES, SEND_BYTES].into_iter().enumerate() {
        match grant(device_id, epoch, bytes) {
            Some(d) => {
                grants[i] = d.grant_id;
                maps[i] = Some(d);
            }
            None => {
                abandon(device_id, &grants);
                return Err("dma map failed");
            }
        }
    }
    let [Some(r), Some(rx), Some(tx)] = maps else {
        let _ = mk_device_release(device_id);
        return Err("dma map failed");
    };
    Ok((grants, [r, rx, tx]))
}
