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

use super::constants::{CQ_BYTES, DATA_BYTES, IO_ENTRIES, PRP_LIST_BYTES, SQ_BYTES};
use super::doorbell::{cq_head_doorbell, sq_tail_doorbell};
use super::geometry::NamespaceGeometry;
use super::queue::IoQueue;
use crate::admin::CqCursor;
use crate::dma::DmaRegion;
use crate::error::NvmeResult;

impl IoQueue {
    pub(super) fn allocate(
        device_id: u64,
        epoch: u64,
        stride: u8,
        qid: u16,
        geometry: &NamespaceGeometry,
    ) -> NvmeResult<Self> {
        Ok(Self {
            sq: DmaRegion::map(device_id, epoch, SQ_BYTES)?,
            cq: DmaRegion::map(device_id, epoch, CQ_BYTES)?,
            prp_list: DmaRegion::map(device_id, epoch, PRP_LIST_BYTES)?,
            data: DmaRegion::map(device_id, epoch, DATA_BYTES)?,
            sq_tail: 0,
            cursor: CqCursor::new(IO_ENTRIES),
            cid: 1,
            out: None,
            sq_db: sq_tail_doorbell(qid, stride),
            cq_db: cq_head_doorbell(qid, stride),
            nsid: geometry.nsid,
            capacity_sectors: geometry.capacity_sectors,
            lba_size: geometry.lba_size,
            max_sectors: geometry.max_sectors,
        })
    }
}
