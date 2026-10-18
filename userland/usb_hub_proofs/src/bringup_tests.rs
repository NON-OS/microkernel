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

//! A hub port's device brought up one step per look, every wait a deadline:
//! 100 ms of steady connection, a reset read no sooner than 10 ms after it
//! was asked for, 10 ms of recovery, then the address, once until it
//! leaves; and a port tried at most three times a minute however often it
//! reconnects.

use crate::bringup_steps::{next, next_within, port, HIGH};
use nonos_usb_hub::Bringup;
use nonos_usb_hub::{Step, TRIES_PER_MINUTE};

#[test]
fn a_device_is_debounced_reset_recovered_and_addressed_on_the_clock() {
    let mut b = Bringup::new();
    let up = port(true, false, false);
    assert_eq!(next(&mut b, up, 1000), (1100, Step::Reset), "after 100 ms of connection");
    assert!(b.busy());
    assert_eq!(b.step(port(true, false, true), 1101, true), Step::Wait, "resetting");
    let done = port(true, true, false);
    assert_eq!(b.step(done, 1150, true), Step::ResetOver(None));
    assert_eq!(next(&mut b, done, 1151), (1160, Step::Address(HIGH)), "after 10 ms");
    b.decide();
    assert_eq!(next_within(&mut b, done, 1161, 120_000), None, "addressed once while it stays");
}

#[test]
fn a_port_read_at_once_after_its_reset_is_asked_for_is_not_yet_reset() {
    // Enabled before the reset, which the hub has not begun to show: read
    // at once, it looked reset, and the device was addressed mid reset.
    let mut b = Bringup::new();
    let enabled = port(true, true, false);
    assert_eq!(next(&mut b, enabled, 0), (100, Step::Reset));
    assert_eq!(next(&mut b, enabled, 101), (110, Step::ResetOver(None)));
}

#[test]
fn a_flapping_port_is_tried_three_times_a_minute() {
    let mut b = Bringup::new();
    let (mut resets, mut t) = (0, 0u64);
    // Connected 150 ms, then gone 50 ms, for two minutes.
    while t < 120_000 {
        let st = port(t % 200 < 150, false, false);
        resets += u32::from(b.step(st, t, true) == Step::Reset);
        t += 1;
    }
    assert_eq!((resets, TRIES_PER_MINUTE), (6, 3), "three a minute for two minutes");
}
