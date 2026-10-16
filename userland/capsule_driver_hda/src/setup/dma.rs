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

use nonos_libc::{mk_dma_map, DmaMapOut};

use crate::controller::bdl::RING_BYTES;
use crate::error::{HdaError, HdaResult};

const PAGE_MASK: u64 = 0xfff;
const CORB_BYTES: u64 = 1024;
const RIRB_BYTES: u64 = 2048;
const BDL_BYTES: u64 = 4096;

#[inline]
fn page_round(n: u64) -> u64 {
    (n + PAGE_MASK) & !PAGE_MASK
}

fn alloc(device_id: u64, claim_epoch: u64, bytes: u64) -> HdaResult<DmaMapOut> {
    let mut out = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
    let r = mk_dma_map(device_id, claim_epoch, page_round(bytes), 0, &mut out);
    if r < 0 {
        Err(HdaError::BrokerCallFailed(r))
    } else {
        Ok(out)
    }
}

/// The CORB and the RIRB. A grant made before a failure is not unmapped
/// here: `BrokerHandles` already holds the device, and its drop stops the
/// controller before the release takes every grant back.
pub fn map_verb_rings(device_id: u64, claim_epoch: u64) -> HdaResult<(DmaMapOut, DmaMapOut)> {
    let corb = alloc(device_id, claim_epoch, CORB_BYTES)?;
    let rirb = alloc(device_id, claim_epoch, RIRB_BYTES)?;
    Ok((corb, rirb))
}

/// The buffer descriptor list page and the PCM ring, given back the same way.
pub fn map_stream(device_id: u64, claim_epoch: u64) -> HdaResult<(DmaMapOut, DmaMapOut)> {
    let bdl = alloc(device_id, claim_epoch, BDL_BYTES)?;
    let sample = alloc(device_id, claim_epoch, RING_BYTES)?;
    Ok((bdl, sample))
}
