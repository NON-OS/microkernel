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

//! What the virtio specification requires of a driver bringing a device up.
//!
//! The window is memory, so a register reads back what was written. That is
//! the behaviour a conforming device has for the status byte, and it is why
//! these can assert on the state the driver leaves behind.

use nonos_devmodel::FakeBar;

use crate::constants::{
    LEG_GUEST_FEATURES, LEG_QUEUE_NUM, LEG_QUEUE_PFN, LEG_QUEUE_SEL, LEG_STATUS,
    STATUS_ACKNOWLEDGE, STATUS_DRIVER, STATUS_DRIVER_OK, STATUS_FAILED, STATUS_FEATURES_OK,
};
use crate::init::bring_up;
use crate::regs::Regs;

const WINDOW: usize = 256;

/// A device advertising a queue of `qmax` entries and nothing else.
fn device(qmax: u16) -> FakeBar {
    let bar = FakeBar::new(WINDOW);
    bar.present16(LEG_QUEUE_NUM, qmax);
    bar
}

#[test]
fn a_brought_up_device_ends_with_every_status_bit_the_spec_requires() {
    let bar = device(64);
    let size = bring_up(Regs::mmio(bar.base()), 0x1234_5000, 64).expect("bring-up");
    assert_eq!(size, 64);

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
fn the_queue_address_is_written_as_a_page_number_not_a_byte_address() {
    /*
     * The legacy queue register takes a page frame number. Writing the byte
     * address instead is the classic way to point a device at memory 4096
     * times further up than intended, and it is invisible until the device
     * does a DMA. `0x1234_5000 >> 12` is the number that belongs there.
     */
    let bar = device(64);
    bring_up(Regs::mmio(bar.base()), 0x1234_5000, 64).expect("bring-up");
    assert_eq!(bar.wrote32(LEG_QUEUE_PFN), 0x0001_2345);
}

#[test]
fn the_queue_is_selected_before_its_size_is_believed() {
    // Queue 0 is the requestq. Reading the size without selecting first reads
    // whichever queue the device happened to have selected.
    let bar = device(64);
    bring_up(Regs::mmio(bar.base()), 0x1000, 64).expect("bring-up");
    assert_eq!(bar.wrote16(LEG_QUEUE_SEL), 0);
}

#[test]
fn no_feature_bits_are_claimed() {
    // virtio-rng needs none, and claiming one the device does not offer is a
    // protocol violation rather than a wish.
    let bar = device(64);
    bring_up(Regs::mmio(bar.base()), 0x1000, 64).expect("bring-up");
    assert_eq!(bar.wrote32(LEG_GUEST_FEATURES), 0);
}

#[test]
fn a_device_with_no_queue_is_refused_and_told_so() {
    /*
     * The failure that matters: a device that advertises no requestq cannot
     * serve entropy. Driving on regardless would leave the system taking
     * randomness from a queue that was never configured.
     */
    let bar = device(0);
    let err = bring_up(Regs::mmio(bar.base()), 0x1000, 64).expect_err("must refuse");
    assert!(err.contains("requestq"), "unexpected error: {err}");

    let status = bar.wrote8(LEG_STATUS);
    assert!(status & STATUS_FAILED != 0, "the device must be told the driver gave up");
    assert_eq!(status & STATUS_DRIVER_OK, 0, "DRIVER_OK must never follow a refusal");
    assert_eq!(bar.wrote32(LEG_QUEUE_PFN), 0, "no queue address on a refused device");
}

#[test]
fn the_queue_size_is_clamped_to_what_the_device_offers() {
    // Asking for more than the device has is the driver's error to absorb, not
    // the device's to police.
    let bar = device(16);
    let size = bring_up(Regs::mmio(bar.base()), 0x1000, 64).expect("bring-up");
    assert_eq!(size, 16);
}

#[test]
fn a_generous_device_does_not_widen_the_queue_past_the_request() {
    let bar = device(1024);
    let size = bring_up(Regs::mmio(bar.base()), 0x1000, 64).expect("bring-up");
    assert_eq!(size, 64);
}
