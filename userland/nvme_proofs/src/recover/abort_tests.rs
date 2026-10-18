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

//! What the controller says of the Abort decides what follows. DW0 bit 0 of
//! its completion is set when the command was not aborted (NVMe 1.4, 5.1):
//! no Command Abort Requested completion is coming, so there is no grace,
//! only one look at the ring for a command done before the Abort reached
//! it, and the reset takes back one still out. Only bit 0 counts; the rest
//! of DW0 is reserved, whatever the controller writes there.

use super::{Recovered, ABORT_GRACE_MS};
use crate::fake::{disables, up};

#[test]
fn an_abort_the_controller_did_not_carry_out_is_reset_after_one_look() {
    for dw0 in [1, 0x8000_0001, u32::MAX] {
        let mut up = up();
        up.stall_a_read(1);
        up.fake.borrow_mut().abort_dw0 = dw0;
        let (from, ticks, resets) = (up.admin_count(), up.fake.borrow().ticks, disables());
        assert_eq!(up.recover(None), Recovered::Reset, "dw0 {dw0:#x}");
        assert_eq!(up.ops_since(from), [0x08, 0x05, 0x01], "dw0 {dw0:#x}");
        let took = up.fake.borrow().ticks - ticks;
        assert!(took < ABORT_GRACE_MS / 100, "a grace of {took} reads for nothing");
        assert_eq!((up.io.stuck(), disables() - resets), (None, 1));
        let mut buf = [0u8; 512];
        assert!(up.io.read_into(up.regs, 0, 1, &mut buf).is_ok(), "the queue serves again");
    }
}

#[test]
fn bit_0_alone_says_not_aborted() {
    // Reserved bits set, bit 0 clear: aborted, and the command comes back
    // within its grace, half a second after the Abort is answered.
    let mut up = up();
    (up.fake.borrow_mut().abort_frees, up.fake.borrow_mut().abort_dw0) = (true, 0xffff_fffe);
    up.stall_a_read(1);
    up.fake.borrow_mut().hold = 5;
    assert_eq!(up.recover(None), Recovered::Aborted);
    assert_eq!((up.io.stuck(), disables()), (None, 0));
}

#[test]
fn a_command_done_before_its_abort_is_retired_without_a_reset() {
    let mut up = up();
    up.stall_a_read(1);
    // It completes once its wait has run out; the Abort then finds nothing
    // to abort, and says so with bit 0.
    up.fake.borrow_mut().finish(1);
    up.fake.borrow_mut().abort_dw0 = 1;
    let (from, ticks) = (up.admin_count(), up.fake.borrow().ticks);
    assert_eq!(up.recover(None), Recovered::Aborted);
    assert_eq!((up.ops_since(from), up.io.stuck(), disables()), (vec![0x08], None, 0));
    assert!(up.fake.borrow().ticks - ticks < ABORT_GRACE_MS / 100, "no grace waited out");
}
