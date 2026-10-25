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

//! The raw input ring carries every key typed into every program. Drivers post
//! to it; the input router alone drains it. When draining took `InputSource`,
//! the bit every input driver holds to post, a keyboard or touchpad driver
//! could read every key; these pin the split on the real token and cap table.
//! Posting takes `InputSource` alone and draining `InputConsumer` alone: Irq,
//! which every driver holds, and Admin stand in for neither.

use crate::authorization_tests::token;
use crate::capabilities::Capability;
use crate::syscall::contract::is_allowed;
use crate::syscall::numbers::SyscallNumber::{
    MkInputEventDrain, MkInputEventPost, MkInputEventWait,
};

extern crate alloc;
use alloc::vec;

#[test]
fn an_input_driver_posts_but_cannot_drain_or_wait() {
    let driver = token(vec![Capability::InputSource]);
    assert!(is_allowed(&driver, MkInputEventPost));
    assert!(!is_allowed(&driver, MkInputEventDrain));
    assert!(!is_allowed(&driver, MkInputEventWait));
    // Irq alone neither posts nor drains: a driver could forge keys or read them.
    let irq = token(vec![Capability::Irq]);
    assert!(![MkInputEventPost, MkInputEventDrain, MkInputEventWait]
        .into_iter()
        .any(|n| is_allowed(&irq, n)));
}

#[test]
fn the_ps2_driver_holding_both_post_bits_still_cannot_drain() {
    let ps2 = token(vec![Capability::Irq, Capability::InputSource, Capability::Pio]);
    assert!(is_allowed(&ps2, MkInputEventPost));
    assert!(!is_allowed(&ps2, MkInputEventDrain));
    assert!(!is_allowed(&ps2, MkInputEventWait));
}

#[test]
fn the_router_drains_and_waits_but_does_not_post() {
    let router = token(vec![Capability::IPC, Capability::Memory, Capability::InputConsumer]);
    assert!(is_allowed(&router, MkInputEventDrain));
    assert!(is_allowed(&router, MkInputEventWait));
    assert!(!is_allowed(&router, MkInputEventPost));
}

#[test]
fn admin_stands_in_for_neither() {
    let admin = token(vec![Capability::Admin]);
    assert!(!is_allowed(&admin, MkInputEventPost));
    assert!(!is_allowed(&admin, MkInputEventDrain));
    assert!(!is_allowed(&admin, MkInputEventWait));
}
