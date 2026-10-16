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

//! The engine's own copy of a transfer's bytes, the buffer the receive FIFO
//! is drained into before the reply is built from it, is wiped wherever it
//! is let go: after the reply, on an error, and in setup's probes (C8).

use std::mem::MaybeUninit;

use crate::transaction::TransferResult;

#[test]
fn a_transfer_result_is_wiped_where_it_is_dropped() {
    let mut slot = MaybeUninit::new(TransferResult::empty());
    // SAFETY: `slot` was initialised on the line above.
    let result = unsafe { slot.assume_init_mut() };
    result.read[..4].copy_from_slice(&[0xA5, 0x5A, 0x12, 0x34]);
    result.read_len = 4;
    // SAFETY: `slot` holds an initialised TransferResult, dropped once here
    // and never used as one again.
    unsafe { slot.assume_init_drop() };
    // SAFETY: dropping a TransferResult frees nothing, so the slot's memory
    // is still live, and its `read` field is plain bytes, initialised.
    let read = unsafe { std::ptr::addr_of!((*slot.as_ptr()).read).read() };
    assert_eq!(read[..4], [0; 4], "the dropped result still holds the device's bytes");
    assert!(read.iter().all(|&b| b == 0));
}
