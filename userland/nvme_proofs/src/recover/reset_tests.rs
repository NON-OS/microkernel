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

//! A command an Abort does not settle is reset away: the controller is
//! disabled, which drops it, and its queues come back in the same memory.

use super::{Recovered, ABORT_GRACE_MS};
use crate::admin::Submission;
use crate::constants::REG_INTMS;
use crate::fake::{register, up};

/// The queue addresses Create I/O CQ (05h) and SQ (01h) named.
fn created(cmds: &[Submission]) -> Vec<(u32, u64)> {
    let queues = cmds.iter().filter(|c| matches!(c.cdw0 & 0xff, 0x05 | 0x01));
    queues.map(|c| (c.cdw0 & 0xff, c.prp1)).collect()
}

#[test]
fn a_command_the_abort_leaves_out_is_reset_away_and_the_queues_rebuilt_in_place() {
    let mut up = up();
    let first = created(&up.admin_since(0));
    let mut buf = [0u8; 512];
    assert!(up.io.read_into(up.regs, 0, 1, &mut buf).is_ok(), "command 1 completes");
    up.stall_a_read(2);
    // From here the controller answers two clock reads after a command: the
    // driver reads its rings first, so an old entry would pass for new.
    up.fake.borrow_mut().hold = 2;
    let (from, ticks) = (up.admin_count(), up.fake.borrow().ticks);
    assert_eq!(up.recover(None), Recovered::Reset);
    assert_eq!(up.ops_since(from), [0x08, 0x05, 0x01], "Abort, then the queues again");
    assert_eq!(created(&up.admin_since(from)), first, "in the same memory");
    // The stuck command had its grace after the Abort's answer, and no more.
    let (grace, took) = (ABORT_GRACE_MS / 100, up.fake.borrow().ticks - ticks);
    assert!((grace..grace + 15).contains(&took), "recovery took {took} clock reads");
    assert_eq!(up.io.stuck(), None);
    assert_eq!(register(up.regs, REG_INTMS), u32::MAX, "INTx masked again after the reset");
    assert!(up.io.read_into(up.regs, 0, 1, &mut buf).is_ok(), "the queue serves again");
}

#[test]
fn an_abort_with_no_answer_resets() {
    let mut up = up();
    up.stall_a_read(1);
    up.fake.borrow_mut().mute = true;
    let (from, ticks) = (up.admin_count(), up.fake.borrow().ticks);
    assert_eq!(up.recover(None), Recovered::Reset);
    assert_eq!(up.ops_since(from), [0x08, 0x05, 0x01]);
    // The Abort's 5 s admin deadline, one look at the ring, and the reset.
    let took = up.fake.borrow().ticks - ticks;
    assert!((50..60).contains(&took), "recovery took {took} clock reads");
    assert_eq!(up.io.stuck(), None);
}
