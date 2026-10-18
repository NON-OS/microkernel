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

//! The legacy request queue's size is the device's: QUEUE_NUM is read-only
//! and the device lays the ring out for it. A size the region holds is
//! taken as offered, any other refused; it used to be cut to 16.

use super::model::{device, handshake, regs};
use crate::constants::{
    LEGACY_MAX, LEG_QUEUE_PFN, LEG_QUEUE_SEL, LEG_STATUS, STATUS_DRIVER_OK, STATUS_FAILED,
    VQ_REGION_SIZE, VQ_USED_OFFSET,
};

#[test]
fn a_legacy_size_the_region_holds_is_taken_as_the_device_offers_it() {
    for qmax in [1, 8, 16, 64, 128, LEGACY_MAX] {
        let bar = device(qmax);
        assert_eq!(handshake(regs(&bar), 0x1000), Ok(qmax), "{qmax}");
        assert_eq!(bar.wrote32(LEG_QUEUE_PFN), 1, "{qmax}");
    }
}

#[test]
fn a_legacy_size_the_region_cannot_hold_is_refused_never_cut() {
    for qmax in [LEGACY_MAX + 1, 256, 1024, u16::MAX] {
        let bar = device(qmax);
        let err = handshake(regs(&bar), 0x1000).expect_err("refused");
        assert_eq!(err, "virtio-rng: legacy queue does not fit the ring region", "{qmax}");
        let status = bar.wrote8(LEG_STATUS);
        assert_ne!(status & STATUS_FAILED, 0, "{qmax}: the device is told");
        assert_eq!(status & STATUS_DRIVER_OK, 0, "{qmax}");
        assert_eq!(bar.wrote32(LEG_QUEUE_PFN), 0, "{qmax}: no ring was handed over");
    }
}

/// The end of a legacy ring of `q`: descriptors, then the available ring.
const fn avail_end(q: usize) -> usize {
    16 * q + 6 + 2 * q
}

// The largest legacy ring ends where the legacy layout puts the used ring,
// one more entry would not, and its used ring fits the region.
const _: () = assert!(avail_end(LEGACY_MAX as usize) <= VQ_USED_OFFSET);
const _: () = assert!(avail_end(LEGACY_MAX as usize + 1) > VQ_USED_OFFSET);
const _: () = assert!(VQ_USED_OFFSET + 6 + 8 * LEGACY_MAX as usize <= VQ_REGION_SIZE);

#[test]
fn a_ring_past_the_32_bit_page_number_is_refused() {
    let bar = device(8);
    let err = handshake(regs(&bar), 1 << 44).expect_err("refused");
    assert_eq!(err, "virtio-rng: ring above the legacy page range");
    assert_eq!(bar.wrote32(LEG_QUEUE_PFN), 0);
    assert_ne!(bar.wrote8(LEG_STATUS) & STATUS_FAILED, 0);
}

#[test]
fn the_queue_is_selected_before_its_size_is_believed() {
    let bar = device(8);
    bar.present16(LEG_QUEUE_SEL, 0xFFFF);
    handshake(regs(&bar), 0x1000).expect("bring-up");
    assert_eq!(bar.wrote16(LEG_QUEUE_SEL), 0, "queue 0 is the requestq");
}
