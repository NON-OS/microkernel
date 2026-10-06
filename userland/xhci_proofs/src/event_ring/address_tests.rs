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

//! Address Device, against the producer written from the specification:
//! the device is given its SetAddress recovery interval before anything
//! else is sent to it.

use super::events::{command, CC_SUCCESS};
use super::fixture::{Fixture, SLOT};
use crate::controller::{issue_address_device, SET_ADDRESS_SETTLE_MS};
use crate::error::XhciError;

const INPUT_CONTEXT: u64 = 0x10_0000;

#[test]
fn an_addressed_device_is_left_to_settle_before_the_reply() {
    // USB 2.0 section 9.2.6.3 gives the device 2 ms after SET_ADDRESS in
    // which it may ignore Setup packets; Linux waits 10 ms.
    let mut fx = Fixture::new();
    let mut cmd = fx.command_ring();
    let base = cmd.crcr_value() & !0x3F;
    fx.hc.post(command(base, CC_SUCCESS, SLOT));
    let before = nonos_libc::slept_ms();
    issue_address_device(fx.db(), fx.intr(), &mut cmd, &mut fx.ring, INPUT_CONTEXT, SLOT)
        .expect("addressed");
    assert!(SET_ADDRESS_SETTLE_MS >= 2, "at least the specification's recovery interval");
    assert!(nonos_libc::slept_ms() - before >= SET_ADDRESS_SETTLE_MS, "the device was given its time");
}

#[test]
fn a_refused_address_is_reported_at_once() {
    let mut fx = Fixture::new();
    let mut cmd = fx.command_ring();
    let base = cmd.crcr_value() & !0x3F;
    fx.hc.post(command(base, 4, SLOT));
    let before = nonos_libc::slept_ms();
    let got = issue_address_device(fx.db(), fx.intr(), &mut cmd, &mut fx.ring, INPUT_CONTEXT, SLOT);
    assert_eq!(got, Err(XhciError::CommandCompletionFailed(4)));
    assert!(nonos_libc::slept_ms() - before < SET_ADDRESS_SETTLE_MS, "no settle for a device not addressed");
}
