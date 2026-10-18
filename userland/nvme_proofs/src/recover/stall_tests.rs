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

//! An I/O command gets no completion in its 30 s, on the driver's own queues
//! against the fake controller, and recovery ends in a named state with no
//! stuck slot (C1). The late completion is injected by the Abort.

use super::Recovered;
use crate::fake::{disables, up};

#[test]
fn an_abort_the_controller_honours_retires_the_stuck_command() {
    let mut up = up();
    up.fake.borrow_mut().abort_frees = true;
    let mut buf = [0u8; 512];
    assert!(up.io.read_into(up.regs, 0, 1, &mut buf).is_ok(), "command 1 completes");
    up.stall_a_read(2);
    let (from, ticks) = (up.admin_count(), up.fake.borrow().ticks);
    assert_eq!(up.recover(None), Recovered::Aborted);
    let took = up.fake.borrow().ticks - ticks;
    assert!(took < 5, "an Abort answered at once took {took} clock reads");
    assert_eq!(up.ops_since(from), [0x08], "one Abort, nothing else");
    assert_eq!(up.admin_since(from)[0].cdw10, (2 << 16) | 1, "command 2, queue 1");
    assert_eq!((up.io.stuck(), disables()), (None, 0), "retired without a reset");
    up.fake.borrow_mut().stall.clear();
    assert!(up.io.read_into(up.regs, 0, 1, &mut buf).is_ok(), "the queue serves again");
}

#[test]
fn a_controller_gone_from_the_bus_is_left_alone() {
    let mut up = up();
    up.fake.borrow_mut().gone = true;
    up.stall_a_read(1);
    let from = up.admin_count();
    assert_eq!(up.recover(None), Recovered::Gone);
    assert_eq!((up.ops_since(from).len(), disables()), (0, 0), "nothing sent to it");
    assert_eq!(up.io.stuck(), Some(1), "its buffer is never reused");
}

#[test]
fn a_fatal_controller_is_reset_without_an_abort() {
    let mut up = up();
    up.fake.borrow_mut().fatal = true;
    up.stall_a_read(1);
    let (from, ticks) = (up.admin_count(), up.fake.borrow().ticks);
    assert_eq!(up.recover(None), Recovered::Reset);
    let took = up.fake.borrow().ticks - ticks;
    assert!(took < 10, "a reset alone took {took} clock reads");
    assert_eq!(up.ops_since(from), [0x05, 0x01], "only the queues created again");
    assert_eq!((up.io.stuck(), disables()), (None, 1));
}

#[test]
fn a_reset_that_fails_keeps_the_buffer_the_device_may_still_write() {
    let mut up = up();
    up.fake.borrow_mut().wedged = true;
    up.stall_a_read(1);
    assert_eq!(up.recover(None), Recovered::Failed);
    assert_eq!(up.io.stuck(), Some(1), "command 1 may still be moving data");
}
