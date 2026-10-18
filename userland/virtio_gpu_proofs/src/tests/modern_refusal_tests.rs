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

//! The bring-up against parts that cannot be driven, which must be left
//! marked FAILED and never DRIVER_OK, with no queue enabled.

use nonos_devmodel::FakeBar;
use nonos_virtio::common::{DEVICE_STATUS, QUEUE_ENABLE, STATUS_DRIVER_OK, STATUS_FAILED};

use super::model::{bring_up, modern_window, NOTIFY_LEN, QUEUE_SIZE};
use crate::constants::VIRTIO_GPU_F_VIRGL;

fn refused(bar: &FakeBar, why: &str) {
    assert_eq!(bring_up(bar, NOTIFY_LEN).err(), Some(why));
    let status = bar.wrote8(DEVICE_STATUS);
    assert_ne!(status & STATUS_FAILED, 0, "{why}: the part is told");
    assert_eq!(status & STATUS_DRIVER_OK, 0, "{why}: and never told the driver is ready");
    assert_eq!(bar.wrote16(QUEUE_ENABLE), 0, "{why}: no queue was enabled");
}

#[test]
fn a_part_without_version_1_is_marked_failed_and_refused() {
    refused(&modern_window(0, QUEUE_SIZE), "virtio: VERSION_1 not offered");
}

#[test]
fn a_part_without_a_control_queue_is_marked_failed_and_refused() {
    refused(&modern_window(VIRTIO_GPU_F_VIRGL, 0), "virtio: queue missing");
}

#[test]
fn a_ring_of_one_entry_is_refused() {
    // A command is a chain of two descriptors.
    refused(&modern_window(VIRTIO_GPU_F_VIRGL, 1), "virtio: queue smaller than the ring layout");
}
