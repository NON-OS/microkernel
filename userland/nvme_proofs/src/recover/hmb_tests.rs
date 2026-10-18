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

//! A reset takes the host memory buffer back, and the rebuild gives it again
//! before the I/O queues, as bring-up does. A controller that refuses it now
//! is served without it, as the spec allows; one that never answers cannot
//! be trusted with the queues, and the reset fails.

use super::Recovered;
use crate::fake::up;

/// SET FEATURES dwords 11 to 15 of a buffer: enabled, 64 pages, its
/// descriptor list at 0x1000, two descriptors.
const DW: [u32; 5] = [1, 64, 0x1000, 0, 2];

#[test]
fn a_reset_gives_the_host_memory_buffer_back_before_the_queues() {
    let mut up = up();
    up.stall_a_read(1);
    let from = up.admin_count();
    assert_eq!(up.recover(Some(DW)), Recovered::Reset);
    assert_eq!(up.ops_since(from), [0x08, 0x09, 0x05, 0x01]);
    let hmb = up.admin_since(from)[1];
    assert_eq!((hmb.cdw10, [hmb.cdw11, hmb.cdw12, hmb.cdw13, hmb.cdw14, hmb.cdw15]), (0x0d, DW));
}

#[test]
fn a_buffer_refused_after_a_reset_leaves_the_drive_served_without_it() {
    let mut up = up();
    up.stall_a_read(1);
    up.fake.borrow_mut().refuse_hmb = true;
    let from = up.admin_count();
    assert_eq!(up.recover(Some(DW)), Recovered::Reset);
    assert_eq!(up.ops_since(from), [0x08, 0x09, 0x05, 0x01], "the queues after the refusal");
    let mut buf = [0u8; 512];
    assert!(up.io.read_into(up.regs, 0, 1, &mut buf).is_ok(), "the queue serves");
}

#[test]
fn a_buffer_never_answered_after_a_reset_fails_it() {
    let mut up = up();
    up.stall_a_read(1);
    up.fake.borrow_mut().ignore_hmb = true;
    let from = up.admin_count();
    assert_eq!(up.recover(Some(DW)), Recovered::Failed);
    assert_eq!(up.ops_since(from), [0x08, 0x09], "no queue on a controller that went quiet");
}
