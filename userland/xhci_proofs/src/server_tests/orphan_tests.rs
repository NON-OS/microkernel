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

//! A class driver that exits or is restarted leaves its slots behind; the
//! server gives them back once its pid is gone, and its ports with them,
//! so the restarted driver, a new pid, can address them again. They used
//! to stay held until reboot, Address Device answering E_BUSY. A slot the
//! controller no longer has answers Disable Slot with Slot Not Enabled: it
//! is gone, so it is given back, not tried again on every sweep.

use nonos_libc::pid_exits;

use super::bench::Bench;
use super::hc_device::lock;
use crate::constants::TRB_TYPE_DISABLE_SLOT_CMD;
use crate::protocol::{E_BUSY, OP_ADDRESS_DEVICE};
use crate::server::reply::set_sender;

const HIGH_SPEED: u32 = 3;
const GONE: u32 = 0x4101;
const STAYS: u32 = 0x4102;
const RESTARTED: u32 = 0x4103;
const LOST: u32 = 0x4104;

#[test]
fn the_slots_of_a_client_that_exited_are_disabled_and_its_port_freed() {
    let mut b = Bench::new(HIGH_SPEED);
    set_sender(GONE);
    let held = b.root_device();
    set_sender(STAYS);
    let kept = b.enable_slot();
    set_sender(RESTARTED);
    let fresh = b.enable_slot();
    assert_eq!(b.call(OP_ADDRESS_DEVICE, &[fresh, 1]).status, E_BUSY, "the port is held");
    pid_exits(GONE);
    b.mux.sweep();
    assert!(!lock(&b.hc).enabled[held as usize], "the controller disabled the slot");
    assert!(lock(&b.hc).enabled[kept as usize], "a live client's slot stays");
    assert_eq!(b.call(OP_ADDRESS_DEVICE, &[fresh, 1]).status, 0, "the port came back");
    set_sender(0);
}

#[test]
fn a_slot_the_controller_no_longer_has_is_given_back_once() {
    let mut b = Bench::new(HIGH_SPEED);
    set_sender(LOST);
    let slot = b.enable_slot();
    set_sender(0);
    lock(&b.hc).enabled[slot as usize] = false;
    pid_exits(LOST);
    b.mux.sweep();
    b.mux.sweep();
    let disables = b.ran().iter().filter(|r| r.trb.get_type() == TRB_TYPE_DISABLE_SLOT_CMD).count();
    assert_eq!(disables, 1, "answered Slot Not Enabled, it was not tried again");
    let d = &b.mux.ctxs[0].driver;
    assert!(!d.slots.is_allocated(slot, d.layout.max_slots), "the table gave it back");
}
