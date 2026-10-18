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

//! A control request that STALLs on a controller that failed as its EP0
//! doorbell rang. The STALL alone would have EP0 reset and moved on, but a
//! failed controller is sent no command: the buffer stays with the slot, as
//! it would had the wait ended on the failure itself (`dead_buffer_tests`).
//! EP0 is not back, so the STALL is answered E_IO: E_PIPE would tell the
//! class driver the pipe is clear.

use nonos_libc::{start_log, take_log, Call};

use super::bench::Bench;
use super::hc_device::lock;
use super::kept_bound_tests::{get_status, HIGH_SPEED};
use crate::protocol::E_IO;

#[test]
fn a_stall_on_a_controller_that_failed_meanwhile_is_not_recovered() {
    let mut b = Bench::new(HIGH_SPEED);
    let slot = b.root_device();
    (lock(&b.hc).stall_in, lock(&b.hc).race.fail_at_ep0) = (true, true);
    let commands = b.ran().len();
    start_log(|| 0);
    assert_eq!(get_status(&mut b, slot), E_IO, "a STALL on an EP0 not brought back");
    let freed = take_log().iter().filter(|l| matches!(l.call, Call::DmaUnmap(_))).count();
    assert_eq!(b.ran().len(), commands, "a recovery command went to a failed controller");
    assert_eq!(freed, 0, "freed while the failed controller may still write it");
    let d = &mut b.mux.ctxs[0].driver;
    let res = d.slots.resources_mut(slot, d.layout.max_slots).expect("the slot");
    assert_eq!(res.quarantine.len(), 1, "kept with the slot until the reset");
}
