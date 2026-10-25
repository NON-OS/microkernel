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

//! A composite device two class drivers share: the keyboard's driver
//! enabled and addressed the slot, the security key's driver claimed
//! another interface of it. The controller serves a slot only to the
//! client that enabled it, save one the multiplexer vouched for, so the
//! claimant is served and a stranger is not; and when the enabling driver
//! exits, the slot stays while the claimant runs, then goes with it.

use nonos_libc::pid_exits;

use super::bench::Bench;
use super::hc_device::lock;
use crate::protocol::{OP_CLAIM_INTERFACE, OP_GET_DEVICE_DESCRIPTOR};
use crate::server::reply::set_sender;

const HIGH_SPEED: u32 = 3;
const KEYBOARD: u32 = 0x4201;
const KEY: u32 = 0x4202;
const STRANGER: u32 = 0x4203;

#[test]
fn a_claimant_is_served_a_slot_it_did_not_enable_and_keeps_it_after_its_enabler_exits() {
    let mut b = Bench::new(HIGH_SPEED);
    set_sender(KEYBOARD);
    let slot = b.root_device();
    assert_eq!(b.call(OP_CLAIM_INTERFACE, &[slot, 1, 0]).status, 0, "the keyboard's");
    set_sender(KEY);
    assert_eq!(b.call(OP_CLAIM_INTERFACE, &[slot, 1, 1]).status, 0, "the key's");
    assert_eq!(b.call(OP_GET_DEVICE_DESCRIPTOR, &[slot]).status, 0, "a claimant is served");
    set_sender(STRANGER);
    let refused = b.call(OP_CLAIM_INTERFACE, &[slot, 1, 0]).status;
    assert_ne!(refused, 0, "a claimed interface is not a stranger's");
    pid_exits(KEYBOARD);
    b.mux.sweep();
    assert!(lock(&b.hc).enabled[slot as usize], "the key's driver still drives it");
    set_sender(KEY);
    assert_eq!(b.call(OP_GET_DEVICE_DESCRIPTOR, &[slot]).status, 0, "and is served");
    pid_exits(KEY);
    b.mux.sweep();
    assert!(!lock(&b.hc).enabled[slot as usize], "gone with its last claimant");
    set_sender(0);
}
