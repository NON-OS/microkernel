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

//! A STALLed control request is answered E_PIPE, a request the device does
//! not support, only once EP0 is back; when the recovery fails the pipe is
//! not clear and the answer is E_IO (`quarantine_tests`, `dead_stall_tests`).

use super::bench::Bench;
use super::hc_device::lock;
use crate::protocol::{E_IO, E_PIPE, OP_CONTROL_TRANSFER};

const HIGH_SPEED: u32 = 3;
const GET_STATUS: [u8; 2] = [0x80, 0];

fn get_status(b: &mut Bench, slot: u8) -> i32 {
    let body = [slot, 0, GET_STATUS[0], GET_STATUS[1], 0, 0, 0, 0, 2, 0];
    b.call(OP_CONTROL_TRANSFER, &body).status
}

#[test]
fn a_stall_whose_ep0_comes_back_is_e_pipe() {
    let mut b = Bench::new(HIGH_SPEED);
    let slot = b.root_device();
    lock(&b.hc).stall_in = true;
    assert_eq!(get_status(&mut b, slot), E_PIPE, "the device's answer, EP0 recovered");
}

#[test]
fn a_stall_whose_recovery_fails_is_e_io() {
    let mut b = Bench::new(HIGH_SPEED);
    let slot = b.root_device();
    (lock(&b.hc).stall_in, lock(&b.hc).refuse_recovery) = (true, true);
    assert_eq!(get_status(&mut b, slot), E_IO, "EP0 is still halted");
}
