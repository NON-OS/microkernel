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

//! Queue programming against the model device.

use super::device::ModelDevice;
use crate::common::{
    choose_size, queue_max, setup_queue, NotifyArea, QueueSpec, NO_VECTOR, STATUS_FAILED,
};
use crate::error::VirtioError;
use crate::features::VIRTIO_F_VERSION_1;

const AREA: NotifyArea = NotifyArea { multiplier: 4, len: 0x1000 };

/// A ring above 4 GiB, so the high halves are checked too.
fn spec(index: u16, size: u16, min_size: u16) -> QueueSpec {
    let base = 0x0000_0001_8000_0000u64 + index as u64 * 0x10_0000;
    QueueSpec {
        index,
        size,
        min_size,
        desc: base,
        driver: base + 0x1000,
        device: base + 0x2000,
        vector: None,
    }
}

#[test]
fn the_three_ring_addresses_and_the_size_reach_the_selected_queue() {
    let dev = ModelDevice::new(VIRTIO_F_VERSION_1, &[256, 256]);
    let ready = setup_queue(&dev, &spec(1, 256, 256), AREA).expect("queue 1");
    assert_eq!(ready.size, 256);
    assert_eq!(ready.notify_offset, 4, "queue 1 at multiplier 4");
    let s = dev.state();
    let q = s.queues[1];
    assert_eq!((q.desc, q.driver, q.device), (0x1_8010_0000, 0x1_8010_1000, 0x1_8010_2000));
    assert_eq!((q.size, q.enable), (256, 1));
    assert_eq!(s.queues[0].enable, 0, "queue 0 untouched");
}

#[test]
fn a_larger_device_ring_is_told_the_layout_size() {
    let dev = ModelDevice::new(VIRTIO_F_VERSION_1, &[1024]);
    let ready = setup_queue(&dev, &spec(0, 256, 256), AREA).expect("shrunk");
    assert_eq!(ready.size, 256);
    assert_eq!(dev.state().queues[0].size, 256);
}

#[test]
fn a_smaller_device_ring_is_taken_only_when_the_layout_adapts() {
    let dev = ModelDevice::new(VIRTIO_F_VERSION_1, &[8]);
    assert_eq!(setup_queue(&dev, &spec(0, 16, 1), AREA).expect("adapts").size, 8);
    let dev = ModelDevice::new(VIRTIO_F_VERSION_1, &[128]);
    assert_eq!(setup_queue(&dev, &spec(0, 256, 256), AREA).err(), Some(VirtioError::QueueTooSmall));
    let s = dev.state();
    assert_ne!(s.status & STATUS_FAILED, 0);
    assert_eq!(s.queues[0].enable, 0, "never enabled");
    assert_eq!(s.queues[0].desc, 0, "never placed");
}

#[test]
fn a_missing_queue_is_refused() {
    let dev = ModelDevice::new(VIRTIO_F_VERSION_1, &[256]);
    assert_eq!(setup_queue(&dev, &spec(1, 256, 1), AREA).err(), Some(VirtioError::QueueMissing));
    assert_ne!(dev.state().status & STATUS_FAILED, 0);
    assert_eq!(choose_size(0, 256, 1), Err(VirtioError::QueueMissing));
    assert_eq!(choose_size(300, 256, 3), Ok(256));
    assert_eq!(choose_size(2, 256, 3), Err(VirtioError::QueueTooSmall));
    assert_eq!(choose_size(5, 0, 0), Err(VirtioError::QueueTooSmall));
}

#[test]
fn a_doorbell_outside_the_mapped_notify_region_is_refused() {
    let dev = ModelDevice::new(VIRTIO_F_VERSION_1, &[256]);
    dev.with(|s| s.queues[0].notify_off = 0x400);
    let err = setup_queue(&dev, &spec(0, 256, 1), AREA).err();
    assert_eq!(err, Some(VirtioError::NotifyOutOfRange));
    assert_eq!(dev.state().queues[0].enable, 0);
}

#[test]
fn unaligned_rings_are_refused_before_the_device_is_touched() {
    for (d, dr, dv) in [(8u64, 0u64, 0u64), (0, 1, 0), (0, 0, 2)] {
        let dev = ModelDevice::new(VIRTIO_F_VERSION_1, &[256]);
        let mut sp = spec(0, 256, 1);
        sp.desc += d;
        sp.driver += dr;
        sp.device += dv;
        assert_eq!(setup_queue(&dev, &sp, AREA).err(), Some(VirtioError::RingUnaligned));
        assert_eq!(dev.state().queues[0].desc, 0);
    }
}

#[test]
fn a_queue_that_does_not_enable_is_refused() {
    let dev = ModelDevice::new(VIRTIO_F_VERSION_1, &[256]);
    dev.with(|s| s.enable_stuck = true);
    let err = setup_queue(&dev, &spec(0, 256, 1), AREA).err();
    assert_eq!(err, Some(VirtioError::QueueNotEnabled));
    assert_ne!(dev.state().status & STATUS_FAILED, 0);
}

#[test]
fn a_vector_the_device_has_reads_back_and_one_it_lacks_reads_no_vector() {
    let dev = ModelDevice::new(VIRTIO_F_VERSION_1, &[256]);
    dev.with(|s| s.nvectors = 1);
    let mut sp = spec(0, 256, 1);
    sp.vector = Some(0);
    assert!(setup_queue(&dev, &sp, AREA).expect("queue").vector_taken);
    assert_eq!(dev.state().queues[0].vector, 0);

    let dev = ModelDevice::new(VIRTIO_F_VERSION_1, &[256]);
    dev.with(|s| s.nvectors = 0);
    let ready = setup_queue(&dev, &sp, AREA).expect("the queue still works by polling");
    assert!(!ready.vector_taken);
    assert_eq!(dev.state().queues[0].vector, NO_VECTOR);

    let dev = ModelDevice::new(VIRTIO_F_VERSION_1, &[256]);
    assert!(!setup_queue(&dev, &spec(0, 256, 1), AREA).expect("queue").vector_taken);
}

#[test]
fn a_layout_placed_by_ring_size_gets_exactly_the_size_it_was_placed_for() {
    // virtio-rng and virtio-blk place the available ring after a descriptor
    // table sized by the ring, as their legacy layouts do: the maximum is
    // read, the size chosen, the addresses worked out from it, and setup
    // must then program that same size.
    let dev = ModelDevice::new(VIRTIO_F_VERSION_1, &[8]);
    assert_eq!(queue_max(&dev, 1), 0, "no second queue");
    let size = choose_size(queue_max(&dev, 0), 16, 1).expect("8 fits a 16-entry layout");
    let base = 0x4000u64;
    let sp = QueueSpec {
        index: 0,
        size,
        min_size: size,
        desc: base,
        driver: base + size as u64 * 16,
        device: base + 0x1000,
        vector: None,
    };
    assert_eq!(setup_queue(&dev, &sp, AREA).expect("queue").size, 8);
    let q = dev.state().queues[0];
    assert_eq!((q.size, q.driver, q.device), (8, base + 128, base + 0x1000));
}
