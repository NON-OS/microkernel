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

//! What a modern device is told about a queue.
//!
//! The ring layout is the legacy one, unchanged: descriptors at 0, the
//! available ring at 4096 and the used ring at 8192 of one DMA region, and
//! ring positions taken modulo RING_SLOTS. Those offsets hold only for a
//! 256-entry ring, so the device is told exactly 256 (it may offer more;
//! QEMU offers 256) and one that cannot take 256 is refused rather than
//! driven with rings where it will not look.

use nonos_virtio::common::QueueSpec;

use crate::constants::{RING_SLOTS, VQ_AVAIL_OFFSET, VQ_DESC_OFFSET, VQ_USED_OFFSET};

/// Queue `index` over the ring region at device address `base`; `None`
/// when a part's address would wrap.
pub fn spec(index: u16, base: u64) -> Option<QueueSpec> {
    Some(QueueSpec {
        index,
        size: RING_SLOTS,
        min_size: RING_SLOTS,
        desc: base.checked_add(VQ_DESC_OFFSET as u64)?,
        driver: base.checked_add(VQ_AVAIL_OFFSET as u64)?,
        device: base.checked_add(VQ_USED_OFFSET as u64)?,
        vector: None,
    })
}
