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

//! A write fills the data buffer only once no command given up on can still
//! be moving data through it: a read still out could land its blocks over
//! the write's, and a write still out could carry them to its own sectors.

use super::model::{io_queue, post};
use super::IoQueue;
use crate::completion_tests::entry;
use crate::error::NvmeError;
use crate::fake::registers;

/// The first block of the data buffer.
fn first_block(io: &IoQueue) -> Vec<u8> {
    // SAFETY: the queue's own data buffer, host memory of 64 blocks.
    unsafe { core::slice::from_raw_parts(io.data.user_va() as *const u8, 512) }.to_vec()
}

#[test]
fn a_write_leaves_the_buffer_alone_while_a_command_is_out() {
    nonos_libc::set_clock(1_000);
    let (regs, mut io) = (registers(), io_queue());
    let first = io.transfer(regs, 0, 1, false);
    assert!(matches!(first, Err(NvmeError::ControllerTimeout)), "{first:?}");
    let r = io.write_from(regs, 0, 1, &[0x5a; 512]);
    assert!(matches!(r, Err(NvmeError::ControllerTimeout)), "{r:?}");
    assert!(first_block(&io).iter().all(|&b| b == 0), "filled under command 1");
    // Command 1 comes back: the write goes ahead, with its own bytes.
    post(&io, 0, entry(1, 1, 0, 1));
    post(&io, 1, entry(2, 1, 0, 1));
    assert!(io.write_from(regs, 0, 1, &[0x5a; 512]).is_ok());
    assert!(first_block(&io).iter().all(|&b| b == 0x5a));
}
