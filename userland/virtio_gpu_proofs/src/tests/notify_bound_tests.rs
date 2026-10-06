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

//! The doorbell must fall inside the mapped notify region.
//!
//! queue_notify_off and the multiplier are both the device's word. Their
//! product used to be added to the notify base unchecked, so a device could
//! aim the driver's 16-bit doorbell write at any address. The model's
//! doorbell for the control queue is NOTIFY_OFF * NOTIFY_MULT = 12 bytes in.

use crate::constants::{
    MOD_DEVICE_STATUS, MOD_QUEUE_ENABLE, MOD_QUEUE_NOTIFY_OFF, STATUS_DRIVER_OK, STATUS_FAILED,
    VIRTIO_GPU_F_VIRGL, VIRTIO_GPU_MODERN,
};
use crate::init::bring_up;
use crate::tests::model::{
    modern_regs, modern_window, NOTIFY_AT, NOTIFY_MULT, NOTIFY_OFF, QUEUE_SIZE, REGION_PHYS,
};

const DOORBELL: usize = NOTIFY_OFF as usize * NOTIFY_MULT;

#[test]
fn a_doorbell_that_fits_the_mapped_region_is_taken() {
    let bar = modern_window(VIRTIO_GPU_F_VIRGL, QUEUE_SIZE);
    let regs = modern_regs(&bar).with_notify_len(DOORBELL + 2);
    let out = bring_up(regs, REGION_PHYS, VIRTIO_GPU_MODERN).expect("live");
    bar.present16(NOTIFY_AT + DOORBELL, 0xFFFF);
    unsafe { out.regs.notify(0) };
    assert_eq!(bar.wrote16(NOTIFY_AT + DOORBELL), 0);
}

#[test]
fn a_doorbell_past_the_mapped_region_is_refused_and_the_part_marked_failed() {
    for len in [0, 1, DOORBELL, DOORBELL + 1] {
        let bar = modern_window(VIRTIO_GPU_F_VIRGL, QUEUE_SIZE);
        let regs = modern_regs(&bar).with_notify_len(len);
        let err = bring_up(regs, REGION_PHYS, VIRTIO_GPU_MODERN).err();
        assert_eq!(err, Some("virtio-gpu: notify address outside the notify region"), "{len}");
        let status = bar.wrote8(MOD_DEVICE_STATUS);
        assert_ne!(status & STATUS_FAILED, 0);
        assert_eq!(status & STATUS_DRIVER_OK, 0);
        assert_eq!(bar.wrote16(MOD_QUEUE_ENABLE), 0, "no queue was enabled");
    }
}

#[test]
fn the_largest_offset_a_device_can_name_is_refused_not_wrapped() {
    let bar = modern_window(VIRTIO_GPU_F_VIRGL, QUEUE_SIZE);
    bar.present16(MOD_QUEUE_NOTIFY_OFF, u16::MAX);
    let regs = modern_regs(&bar).with_notify_len(0x100);
    assert!(bring_up(regs, REGION_PHYS, VIRTIO_GPU_MODERN).is_err());
    assert_eq!(bar.wrote16(MOD_QUEUE_ENABLE), 0);
}
