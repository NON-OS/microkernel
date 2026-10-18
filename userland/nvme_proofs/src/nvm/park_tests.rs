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

//! An I/O wait gives the processor away between rounds of polling: it parks
//! on the controller's MSI-X interrupt, which the I/O completion queue is
//! created to raise, or yields when none is bound. It used to spin a core
//! for a command's whole latency.

use nonos_libc::{calls, REFUSED_GRANT};

use crate::fake::up;

/// The grants the driver parked on, in order.
fn parks() -> Vec<u64> {
    calls().iter().filter(|c| c.0 == "irq_wait").map(|c| c.1).collect()
}

#[test]
fn the_io_completion_queue_raises_vector_0() {
    let up = up();
    let create_cq = up.admin_since(0).into_iter().find(|c| c.cdw0 & 0xff == 0x05);
    // PC and IEN set, interrupt vector (bits 31:16) 0.
    assert_eq!(create_cq.map(|c| c.cdw11), Some(0b11));
}

#[test]
fn a_command_not_done_within_a_round_parks_on_the_interrupt() {
    let mut up = up();
    up.fake.borrow_mut().hold = 3;
    let mut buf = [0u8; 512];
    assert!(up.io.read_into(up.regs, 0, 1, &mut buf).is_ok(), "no vector: it yields");
    assert!(parks().is_empty());
    up.io.park_on(5);
    assert!(up.io.read_into(up.regs, 0, 1, &mut buf).is_ok());
    assert!(!parks().is_empty() && parks().iter().all(|&g| g == 5), "{:?}", parks());
}

#[test]
fn a_grant_the_kernel_refuses_is_dropped_and_the_wait_yields() {
    let mut up = up();
    up.fake.borrow_mut().hold = 3;
    up.io.park_on(REFUSED_GRANT);
    let mut buf = [0u8; 512];
    assert!(up.io.read_into(up.regs, 0, 1, &mut buf).is_ok());
    assert!(up.io.read_into(up.regs, 0, 1, &mut buf).is_ok());
    assert_eq!(parks(), [REFUSED_GRANT], "asked once, then never again");
}

#[test]
fn a_stuck_command_parks_out_its_whole_budget_on_its_grant_2_ms_a_park() {
    let mut up = up();
    up.io.park_on(5);
    up.stall_a_read(1);
    // Nothing is delivered, so every park times out (MK_IRQ_WAIT_TIMED_OUT),
    // and a timed-out park keeps its grant. 30 s on a clock read after each
    // park, 100 ms a read.
    let n = parks().len();
    assert!((295..=305).contains(&n), "parked {n} times");
    assert!(parks().iter().all(|&g| g == 5), "the grant was dropped: {:?}", parks());
    let ms: Vec<u64> = calls().iter().filter(|c| c.0 == "irq_wait_ms").map(|c| c.1).collect();
    assert!(ms.len() == n && ms.iter().all(|&t| t == 2), "{ms:?}");
}
