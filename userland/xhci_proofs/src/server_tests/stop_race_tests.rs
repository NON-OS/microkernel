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

//! A control request its device NAKs until the 5 s wait runs out leaves
//! EP0 Running: Reset Endpoint answers Context State Error, and Stop
//! Endpoint takes its place. A STALL in between halts EP0, and Stop answers
//! Context State Error too. EP0 is then reset after all, as its context
//! says it is Halted, so its dequeue can move and the request's buffer is
//! freed; Set TR Dequeue on the Halted EP0 failed, and the buffer stayed.

use nonos_libc::{start_log, take_log, Call};

use super::bench::Bench;
use super::hc_device::lock;
use crate::protocol::{E_IO, OP_CONTROL_TRANSFER};

const HIGH_SPEED: u32 = 3;
const RESET_ENDPOINT: u32 = 14;
const STOP_ENDPOINT: u32 = 15;
const SET_TR_DEQUEUE: u32 = 16;
/// 32-byte contexts: EP0's follows the Slot Context.
const EP0_CONTEXT: u64 = 32;

#[test]
fn an_endpoint_that_halts_while_being_stopped_is_reset_and_its_buffer_freed() {
    let mut b = Bench::new(HIGH_SPEED);
    let slot = b.root_device();
    let d = &mut b.mux.ctxs[0].driver;
    let res = d.slots.resources_mut(slot, d.layout.max_slots).expect("the slot");
    let at = res.output_context.phys() + EP0_CONTEXT;
    (lock(&b.hc).race.nak_in, lock(&b.hc).race.halt_at) = (true, Some(at));
    let before = b.ran().len();
    start_log(|| 0);
    let get_status = [slot, 0, 0x80, 0, 0, 0, 0, 0, 2, 0];
    assert_eq!(b.call(OP_CONTROL_TRANSFER, &get_status).status, E_IO, "timed out");
    let freed = take_log().iter().filter(|l| matches!(l.call, Call::DmaUnmap(_))).count();
    let ran: Vec<u32> = b.ran()[before..].iter().map(|r| r.trb.get_type()).collect();
    assert_eq!(ran, [RESET_ENDPOINT, STOP_ENDPOINT, RESET_ENDPOINT, SET_TR_DEQUEUE]);
    assert_eq!(freed, 1, "the request's buffer, once EP0 was reset and moved past it");
    assert!(lock(&b.hc).race.halted.is_none(), "EP0 left Halted");
}
