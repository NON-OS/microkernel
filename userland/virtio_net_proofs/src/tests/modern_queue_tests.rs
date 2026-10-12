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

//! The modern queue as the driver asks for it, programmed by the shared
//! transport into a common configuration in host memory: the device is told
//! exactly 256 entries, the size the grant and the offsets hold, and one
//! that cannot take 256 is refused.

use nonos_virtio::common::QUEUE_SIZE;
use nonos_virtio::common::{setup_queue, NotifyArea, QUEUE_DESC, QUEUE_DEVICE, QUEUE_DRIVER};
use nonos_virtio::VirtioError;

use super::mem::Mem;
use crate::constants::{Q_RX, RING_SLOTS, VQ_AVAIL_OFFSET, VQ_USED_OFFSET};
use crate::modern::spec::spec;

const AREA: NotifyArea = NotifyArea { multiplier: 4, len: 0x100 };
const BASE: u64 = 0x30_0000;

fn program(max: u16) -> (Result<u16, VirtioError>, Mem) {
    let common = Mem::new(0x40);
    common.put(QUEUE_SIZE, &max.to_le_bytes());
    let got = setup_queue(&common.view(), &spec(Q_RX, BASE).unwrap(), AREA).map(|r| r.size);
    (got, common)
}

#[test]
fn the_device_is_told_256_whatever_larger_size_it_offers() {
    for max in [256u16, 257, 512, 1024, u16::MAX] {
        let (got, common) = program(max);
        assert_eq!(got, Ok(RING_SLOTS), "max {max}");
        assert_eq!(common.u16_at(QUEUE_SIZE), RING_SLOTS);
        assert_eq!(common.u32_at(QUEUE_DESC), BASE as u32);
        assert_eq!(common.u32_at(QUEUE_DRIVER), (BASE + VQ_AVAIL_OFFSET as u64) as u32);
        assert_eq!(common.u32_at(QUEUE_DEVICE), (BASE + VQ_USED_OFFSET as u64) as u32);
    }
}

#[test]
fn a_device_that_cannot_take_256_is_refused() {
    for max in [1u16, 64, 128, 255] {
        assert_eq!(program(max).0, Err(VirtioError::QueueTooSmall), "max {max}");
    }
    assert_eq!(program(0).0, Err(VirtioError::QueueMissing));
}

#[test]
fn a_ring_whose_parts_would_wrap_has_no_spec() {
    assert!(spec(Q_RX, u64::MAX - 0x1000).is_none());
    assert!(spec(Q_RX, !0xFFF_u64).is_none());
    assert!(spec(Q_RX, u64::MAX - VQ_USED_OFFSET as u64).is_some());
}
