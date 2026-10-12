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

//! The status handshake, as the virtio specification requires it.

use super::model::{device, handshake, regs};
use crate::constants::{
    LEG_GUEST_FEATURES, LEG_QUEUE_PFN, LEG_STATUS, STATUS_ACKNOWLEDGE, STATUS_DRIVER,
    STATUS_DRIVER_OK, STATUS_FAILED, STATUS_FEATURES_OK,
};

#[test]
fn a_brought_up_device_ends_with_every_status_bit_the_spec_requires() {
    let bar = device(8);
    assert_eq!(handshake(regs(&bar), 0x1234_5000), Ok(8));
    let status = bar.wrote8(LEG_STATUS);
    for (bit, name) in [
        (STATUS_ACKNOWLEDGE, "ACKNOWLEDGE"),
        (STATUS_DRIVER, "DRIVER"),
        (STATUS_FEATURES_OK, "FEATURES_OK"),
        (STATUS_DRIVER_OK, "DRIVER_OK"),
    ] {
        assert!(status & bit != 0, "{name} missing from the final status");
    }
    assert_eq!(status & STATUS_FAILED, 0, "a healthy bring-up must not set FAILED");
}

#[test]
fn a_device_with_no_queue_is_refused_and_told_so() {
    // A device with no requestq cannot serve entropy; driving on regardless
    // would take randomness from a queue that was never configured.
    let bar = device(0);
    let err = handshake(regs(&bar), 0x1000).expect_err("must refuse");
    assert!(err.contains("requestq"), "unexpected error: {err}");
    let status = bar.wrote8(LEG_STATUS);
    assert!(status & STATUS_FAILED != 0, "the device must be told the driver gave up");
    assert_eq!(status & STATUS_DRIVER_OK, 0, "DRIVER_OK must never follow a refusal");
    assert_eq!(bar.wrote32(LEG_QUEUE_PFN), 0, "no queue address on a refused device");
}

#[test]
fn no_feature_bits_are_claimed() {
    let bar = device(8);
    bar.present32(LEG_GUEST_FEATURES, u32::MAX);
    handshake(regs(&bar), 0x1000).expect("bring-up");
    assert_eq!(bar.wrote32(LEG_GUEST_FEATURES), 0);
}

#[test]
fn the_queue_address_is_written_as_a_page_number_not_a_byte_address() {
    let bar = device(8);
    handshake(regs(&bar), 0x1234_5000).expect("bring-up");
    assert_eq!(bar.wrote32(LEG_QUEUE_PFN), 0x0001_2345);
}
