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

//! Feature negotiation, pure and against the model device.

use super::device::ModelDevice;
use crate::common::{accept, device_features, STATUS_FAILED, STATUS_FEATURES_OK};
use crate::error::VirtioError;
use crate::features::{negotiate, DEVICE_TYPE_BITS, VIRTIO_F_ACCESS_PLATFORM, VIRTIO_F_VERSION_1};

const NET_F_CSUM: u64 = 1 << 0;
const NET_F_MAC: u64 = 1 << 5;
const NET_F_MRG_RXBUF: u64 = 1 << 15;
const NET_F_STATUS: u64 = 1 << 16;
const NET_F_RSS: u64 = 1 << 60;
const F_INDIRECT_DESC: u64 = 1 << 28;
const F_EVENT_IDX: u64 = 1 << 29;
const F_RING_PACKED: u64 = 1 << 34;

#[test]
fn a_device_without_version_1_is_refused() {
    assert_eq!(negotiate(NET_F_MAC | NET_F_STATUS, NET_F_MAC), Err(VirtioError::NoVersion1));
    assert_eq!(negotiate(0, 0), Err(VirtioError::NoVersion1));
    assert_eq!(negotiate(VIRTIO_F_ACCESS_PLATFORM, 0), Err(VirtioError::NoVersion1));
}

#[test]
fn access_platform_is_taken_whenever_offered() {
    let with = negotiate(VIRTIO_F_VERSION_1 | VIRTIO_F_ACCESS_PLATFORM, 0).expect("modern");
    assert_eq!(with, VIRTIO_F_VERSION_1 | VIRTIO_F_ACCESS_PLATFORM);
    let without = negotiate(VIRTIO_F_VERSION_1, 0).expect("modern");
    assert_eq!(without, VIRTIO_F_VERSION_1, "never taken unoffered");
}

#[test]
fn device_bits_are_masked_to_what_the_driver_handles() {
    let offered = VIRTIO_F_VERSION_1
        | VIRTIO_F_ACCESS_PLATFORM
        | NET_F_CSUM
        | NET_F_MAC
        | NET_F_MRG_RXBUF
        | NET_F_STATUS
        | NET_F_RSS
        | F_INDIRECT_DESC
        | F_EVENT_IDX
        | F_RING_PACKED;
    let taken = negotiate(offered, NET_F_MAC | NET_F_STATUS).expect("modern");
    assert_eq!(taken, VIRTIO_F_VERSION_1 | VIRTIO_F_ACCESS_PLATFORM | NET_F_MAC | NET_F_STATUS);
    // A bit the driver handles but the device did not offer is not taken.
    let taken = negotiate(VIRTIO_F_VERSION_1 | NET_F_MAC, NET_F_MAC | NET_F_STATUS).expect("v1");
    assert_eq!(taken, VIRTIO_F_VERSION_1 | NET_F_MAC);
}

#[test]
fn no_transport_bit_is_ever_taken_even_if_a_driver_asks() {
    let offered = u64::MAX;
    let taken = negotiate(offered, u64::MAX).expect("modern");
    let transport = !DEVICE_TYPE_BITS & !(VIRTIO_F_VERSION_1 | VIRTIO_F_ACCESS_PLATFORM);
    assert_eq!(taken & transport, 0, "event idx, indirect, packed and the rest stay off");
    assert_eq!(taken & F_EVENT_IDX, 0);
    assert_eq!(taken & F_RING_PACKED, 0);
}

#[test]
fn every_offered_set_negotiates_a_subset_with_version_1() {
    let mut s = 0x2545_F491_4F6C_DD1Du64;
    for _ in 0..100_000 {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        let offered = s;
        let handles = s.rotate_left(17);
        match negotiate(offered, handles) {
            Ok(taken) => {
                assert_ne!(offered & VIRTIO_F_VERSION_1, 0);
                assert_eq!(taken & !offered, 0, "taken but not offered");
                assert_ne!(taken & VIRTIO_F_VERSION_1, 0);
                assert_eq!(taken & VIRTIO_F_ACCESS_PLATFORM, offered & VIRTIO_F_ACCESS_PLATFORM);
                let device_taken = taken & DEVICE_TYPE_BITS;
                assert_eq!(device_taken & !handles, 0, "a device bit the driver does not handle");
            }
            Err(e) => {
                assert_eq!(e, VirtioError::NoVersion1);
                assert_eq!(offered & VIRTIO_F_VERSION_1, 0);
            }
        }
    }
}

#[test]
fn the_feature_pages_are_read_and_written_by_select() {
    let offered = VIRTIO_F_VERSION_1 | VIRTIO_F_ACCESS_PLATFORM | NET_F_MAC | NET_F_RSS;
    let dev = ModelDevice::new(offered, &[256]);
    assert_eq!(device_features(&dev), offered);
    let taken = accept(&dev, NET_F_MAC | NET_F_STATUS).expect("accepted");
    assert_eq!(taken, VIRTIO_F_VERSION_1 | VIRTIO_F_ACCESS_PLATFORM | NET_F_MAC);
    assert_eq!(dev.state().driver_features, taken, "both pages written");
    assert_ne!(dev.state().status & STATUS_FEATURES_OK, 0);
}

#[test]
fn a_device_without_version_1_is_marked_failed_before_any_feature_write() {
    let dev = ModelDevice::new(NET_F_MAC, &[256]);
    assert_eq!(accept(&dev, NET_F_MAC), Err(VirtioError::NoVersion1));
    let s = dev.state();
    assert_ne!(s.status & STATUS_FAILED, 0);
    assert_eq!(s.status & STATUS_FEATURES_OK, 0);
    assert_eq!(s.driver_features, 0, "nothing was written");
}

#[test]
fn a_device_that_withholds_features_ok_is_marked_failed() {
    let dev = ModelDevice::new(VIRTIO_F_VERSION_1 | NET_F_MAC, &[256]);
    dev.with(|s| s.refuse_features = true);
    assert_eq!(accept(&dev, NET_F_MAC), Err(VirtioError::FeaturesRejected));
    assert_ne!(dev.state().status & STATUS_FAILED, 0);
}
