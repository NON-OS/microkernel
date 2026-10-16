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

//! The multiplexer's ids across a revival of one of its controllers: the
//! ids bound there are freed (`forget_revived`) and the next Enable Slot
//! reserves one again. A client's stale id then named another client's
//! reservation, and that client's Disable Slot or Address Device took it.

use super::bench::Bench;
use super::hc_device::lock;
use super::hc_reset::Resets;
use super::stale_slot_tests::{fail_and_wait, A, B, C, HIGH_SPEED};
use crate::protocol::{E_INVAL, OP_ADDRESS_DEVICE, OP_DISABLE_SLOT, OP_GET_DEVICE_DESCRIPTOR};
use crate::server::reply::set_sender;

#[test]
fn a_multiplexer_id_held_across_a_revival_reaches_no_other_clients_slot() {
    let mut b = Bench::new(HIGH_SPEED);
    let _second = b.second_controller();
    lock(&b.hc).resets = Resets::Followed;
    set_sender(A);
    let stale = b.root_device();
    fail_and_wait(&b);
    set_sender(B);
    let fresh = b.root_device();
    assert_eq!(b.mux.ctxs[0].driver.generation, 1, "revived");
    set_sender(C);
    let reserved = b.enable_slot();
    assert_eq!(reserved, stale, "C reserved the id A held");
    let commands = b.ran().len();
    set_sender(A);
    assert_eq!(b.call(OP_GET_DEVICE_DESCRIPTOR, &[stale]).status, E_INVAL);
    assert_eq!(b.call(OP_ADDRESS_DEVICE, &[stale, 1]).status, E_INVAL, "bound for A");
    assert_eq!(b.call(OP_DISABLE_SLOT, &[stale]).status, E_INVAL, "C's reservation given up");
    assert_eq!(b.ran().len(), commands, "no slot was enabled for A");
    set_sender(B);
    assert_eq!(b.call(OP_GET_DEVICE_DESCRIPTOR, &[fresh]).status, 0, "B's device is B's");
    set_sender(C);
    assert_eq!(b.call(OP_DISABLE_SLOT, &[reserved]).status, 0, "C's reservation is C's");
    set_sender(0);
}
