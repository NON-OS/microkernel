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

//! The whole modern bring-up in the order the drivers run it, against the
//! model device.

use super::device::ModelDevice;
use crate::common::{
    accept, driver_ok, reset, set_config_vector, setup_queue, stable_read, start, NotifyArea,
    QueueSpec, GENERATION_TRIES, NO_VECTOR, STATUS_ACKNOWLEDGE, STATUS_DRIVER, STATUS_DRIVER_OK,
    STATUS_FEATURES_OK,
};
use crate::error::VirtioError;
use crate::features::{VIRTIO_F_ACCESS_PLATFORM, VIRTIO_F_VERSION_1};

const NET_F_MAC: u64 = 1 << 5;
const NET_F_STATUS: u64 = 1 << 16;
const AREA: NotifyArea = NotifyArea { multiplier: 4, len: 0x1000 };

fn queue(index: u16) -> QueueSpec {
    let base = 0x10_0000u64 * (index as u64 + 1);
    QueueSpec {
        index,
        size: 256,
        min_size: 256,
        desc: base,
        driver: base + 0x1000,
        device: base + 0x2000,
        vector: None,
    }
}

#[test]
fn a_two_queue_device_is_brought_live_in_the_specified_order() {
    let offered = VIRTIO_F_VERSION_1 | VIRTIO_F_ACCESS_PLATFORM | NET_F_MAC | NET_F_STATUS;
    let dev = ModelDevice::new(offered, &[256, 256]);
    start(&dev).expect("reset");
    let taken = accept(&dev, NET_F_MAC | NET_F_STATUS).expect("features");
    assert_eq!(taken, offered);
    let rx = setup_queue(&dev, &queue(0), AREA).expect("rx");
    let tx = setup_queue(&dev, &queue(1), AREA).expect("tx");
    driver_ok(&dev);
    assert_eq!((rx.notify_offset, tx.notify_offset), (0, 4));
    let s = dev.state();
    let a = STATUS_ACKNOWLEDGE;
    let d = STATUS_DRIVER;
    let f = STATUS_FEATURES_OK;
    let ok = STATUS_DRIVER_OK;
    assert_eq!(s.status_writes, vec![0, a, a | d, a | d | f, a | d | f | ok]);
    assert!(s.queues.iter().all(|q| q.enabled_at_driver_ok), "DRIVER_OK only after the queues");
}

#[test]
fn a_device_left_live_by_a_previous_driver_is_reset_first() {
    let dev = ModelDevice::new(VIRTIO_F_VERSION_1, &[256]);
    dev.with(|s| {
        s.queues[0].enable = 1;
        s.queues[0].desc = 0xDEAD_0000;
    });
    start(&dev).expect("reset");
    let s = dev.state();
    assert_eq!(s.status_writes[0], 0, "the first status write is the reset");
    assert_eq!((s.queues[0].enable, s.queues[0].desc), (0, 0), "the old rings are gone");
}

#[test]
fn a_reset_that_never_completes_stops_the_bring_up() {
    let dev = ModelDevice::new(VIRTIO_F_VERSION_1, &[256]);
    dev.with(|s| s.stuck_reset = true);
    assert_eq!(reset(&dev), Err(VirtioError::ResetTimeout));
    assert_eq!(start(&dev), Err(VirtioError::ResetTimeout));
    let s = dev.state();
    assert_eq!(s.status_writes, vec![0, 0], "nothing past the reset was written");
}

#[test]
fn the_config_vector_reads_back_or_reports_the_refusal() {
    let dev = ModelDevice::new(VIRTIO_F_VERSION_1, &[256]);
    dev.with(|s| s.nvectors = 2);
    assert!(set_config_vector(&dev, NO_VECTOR), "no vector is always taken");
    assert!(set_config_vector(&dev, 1));
    assert!(!set_config_vector(&dev, 2), "the device has two");
    assert_eq!(dev.state().config_vector, NO_VECTOR);
}

#[test]
fn device_config_is_read_again_until_the_generation_holds() {
    let dev = ModelDevice::new(VIRTIO_F_VERSION_1, &[256]);
    // Three generation reads move it on: the first pass sees 0 then 1, the
    // second 2 then 3, the third 3 then 3 and is kept.
    dev.with(|s| s.generation_moves = 3);
    let mut passes = 0;
    let got = stable_read(&dev, || {
        passes += 1;
        passes
    });
    assert_eq!(got, Some(3));
    assert_eq!(passes, 3);
}

#[test]
fn device_config_that_never_settles_is_given_up_on() {
    let dev = ModelDevice::new(VIRTIO_F_VERSION_1, &[256]);
    dev.with(|s| s.generation_moves = u32::MAX);
    let mut passes = 0;
    assert_eq!(stable_read(&dev, || passes += 1), None);
    assert_eq!(passes, GENERATION_TRIES);
}
