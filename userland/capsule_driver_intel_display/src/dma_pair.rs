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

//! Two scanout surfaces of one size as DMA grants, high and write-combining
//! (the display pool's), each contiguous from its first IOVA. Half a pair is
//! never kept: the first grant goes back when the second is refused.

use nonos_libc::{mk_dma_map, mk_dma_unmap, DmaMapOut, MK_DMA_MAP_HIGH, MK_DMA_MAP_WC};

use crate::error::{IntelError, IntelResult};
use crate::frames::DmaFrames;

#[derive(Clone, Copy, Debug)]
pub struct Pair {
    pub va: [u64; 2],
    pub iova: [u64; 2],
    pub grant: [u64; 2],
    pub len: usize,
}

fn one(device_id: u64, epoch: u64, len: u64) -> IntelResult<DmaMapOut> {
    let mut out = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
    let rc = mk_dma_map(device_id, epoch, len, MK_DMA_MAP_HIGH | MK_DMA_MAP_WC, &mut out);
    if rc < 0 {
        return Err(IntelError::Broker(rc));
    }
    Ok(out)
}

pub fn map_pair(device_id: u64, epoch: u64, len: u64) -> IntelResult<Pair> {
    let a = one(device_id, epoch, len)?;
    let b = match one(device_id, epoch, len) {
        Ok(b) => b,
        Err(e) => {
            let _ = mk_dma_unmap(a.grant_id);
            return Err(e);
        }
    };
    Ok(Pair {
        va: [a.user_va, b.user_va],
        iova: [a.device_addr, b.device_addr],
        grant: [a.grant_id, b.grant_id],
        len: len as usize,
    })
}

/// Give both grants of `p` back.
pub fn unmap(p: &Pair) {
    for g in p.grant {
        let _ = mk_dma_unmap(g);
    }
}

/// The driver's end gives back the pair shown and any a mode set left.
impl Drop for DmaFrames {
    fn drop(&mut self) {
        unmap(&self.now);
        if let Some(w) = self.was.take() {
            unmap(&w);
        }
    }
}
