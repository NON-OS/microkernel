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

//! The bring-up on the shared transport: the status handshake and the
//! features taken. ACCESS_PLATFORM is taken whenever it is offered, so a
//! device behind an IOMMU sends its DMA through it. Whether the claim is
//! confined is the kernel's to enforce at claim time, so a device that does
//! not offer it still comes up here: the driver does not guess.

use nonos_virtio::common::{DEVICE_STATUS, DRIVER_FEATURE, DRIVER_FEATURE_SELECT};
use nonos_virtio::features::{VIRTIO_F_ACCESS_PLATFORM, VIRTIO_F_VERSION_1};

use super::model::{bring_up, modern_window, NOTIFY_LEN, QUEUE_SIZE};
use crate::constants::{VIRTIO_GPU_F_EDID, VIRTIO_GPU_F_VIRGL};

/// The status byte as the virtio specification numbers it: ACKNOWLEDGE 1,
/// DRIVER 2, DRIVER_OK 4, FEATURES_OK 8.
const LIVE: u8 = 0x0F;
const VIRGL: u64 = VIRTIO_GPU_F_VIRGL as u64;
const EDID: u64 = VIRTIO_GPU_F_EDID as u64;

#[test]
fn a_part_offering_both_features_ends_live_with_both_accepted() {
    let bar = modern_window(VIRTIO_GPU_F_VIRGL | VIRTIO_GPU_F_EDID, QUEUE_SIZE);
    let (features, ..) = bring_up(&bar, NOTIFY_LEN).expect("live");
    assert_eq!(bar.wrote8(DEVICE_STATUS), LIVE);
    assert_eq!(features & (VIRGL | EDID), VIRGL | EDID);
    assert_eq!(bar.wrote32(DRIVER_FEATURE_SELECT), 1, "high page acked last");
    // Bits 0 and 1 of the high page: VERSION_1 and ACCESS_PLATFORM, which
    // is taken whenever it is offered.
    assert_eq!(bar.wrote32(DRIVER_FEATURE), 0b11);
    assert_eq!(features >> 32 << 32, VIRTIO_F_VERSION_1 | VIRTIO_F_ACCESS_PLATFORM);
}

#[test]
fn a_part_that_does_not_offer_access_platform_is_not_given_it() {
    // Bit 0 alone: the high page offers VERSION_1 and not ACCESS_PLATFORM.
    let bar = modern_window(VIRTIO_GPU_F_VIRGL, QUEUE_SIZE);
    let (features, ..) = bring_up(&bar, NOTIFY_LEN).expect("live");
    assert_eq!(features & VIRTIO_F_ACCESS_PLATFORM, 0);
    assert_eq!(bar.wrote32(DRIVER_FEATURE), 1, "VERSION_1 alone");
    assert_eq!(bar.wrote8(DEVICE_STATUS), LIVE);
}

#[test]
fn a_feature_the_part_did_not_offer_is_not_taken() {
    let bar = modern_window(VIRTIO_GPU_F_VIRGL, QUEUE_SIZE);
    let (features, ..) = bring_up(&bar, NOTIFY_LEN).expect("live");
    assert_ne!(features & VIRGL, 0, "offered, so accepted");
    assert_eq!(features & EDID, 0, "not offered, so not claimed");
}

#[test]
fn no_device_bit_the_driver_does_not_handle_is_taken() {
    let bar = modern_window(u32::MAX, QUEUE_SIZE);
    let (features, ..) = bring_up(&bar, NOTIFY_LEN).expect("live");
    assert_eq!(features & 0xFFFF_FFFF, VIRGL | EDID);
    assert_eq!(features >> 32, 0b11, "VERSION_1 and ACCESS_PLATFORM, nothing else");
}
