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

//! The data buffer between a request and the controller, at and past its
//! bounds: no blocks, one block past the buffer, or a client buffer of the
//! wrong size is refused before anything is submitted, and a read hands back
//! exactly its blocks, only once its own completion came back clean.

use super::model::{io_queue, post};
use super::{sq_tail_doorbell, IoQueue};
use crate::completion_tests::entry;
use crate::error::{NvmeError, NvmeResult};
use crate::fake::{register, registers};

/// The data buffer: 64 blocks of 512 bytes.
const BUFFER: usize = 64 * 512;

/// What the data buffer holds.
fn data(io: &IoQueue) -> Vec<u8> {
    // SAFETY: the queue's own data buffer, BUFFER bytes of host memory.
    unsafe { core::slice::from_raw_parts(io.data.user_va() as *const u8, BUFFER) }.to_vec()
}

#[test]
fn nothing_of_no_blocks_past_the_buffer_or_the_wrong_size_goes_out() {
    nonos_libc::set_clock(1_000);
    let (regs, mut io) = (registers(), io_queue());
    let refused = |r: NvmeResult<()>| matches!(r, Err(NvmeError::InvalidTransfer));
    let mut client = vec![0x5a; BUFFER + 1];
    for n in [0, 65, 1 << 23, u32::MAX] {
        assert!(refused(io.read_into(regs, 0, n, &mut client)), "read of {n}");
        assert!(refused(io.write_from(regs, 0, n, &client[..BUFFER])), "write of {n}");
    }
    for (n, len) in [(1, 0), (1, 511), (64, BUFFER - 1)] {
        assert!(refused(io.read_into(regs, 0, n, &mut client[..len])), "{n} into {len}");
    }
    for (n, len) in [(1, 0), (1, 511), (1, 513), (64, BUFFER + 1)] {
        assert!(refused(io.write_from(regs, 0, n, &client[..len])), "{len} bytes as {n}");
    }
    assert_eq!(register(regs, sq_tail_doorbell(1, 0)), 0, "nothing was submitted");
    assert!(data(&io).iter().all(|&b| b == 0), "nor the buffer filled");
}

#[test]
fn a_read_hands_back_its_own_blocks_once_its_completion_is_clean() {
    nonos_libc::set_clock(1_000);
    let (regs, mut io) = (registers(), io_queue());
    // SAFETY: as for `data`; the controller's DMA of the blocks it read.
    unsafe { core::ptr::write_bytes(io.data.user_va() as *mut u8, 0x3c, BUFFER) };
    post(&io, 0, entry(1, 1, 0, 1));
    // Command 2: Unrecovered Read Error (SCT 2, SC 81h) with Do Not Retry.
    post(&io, 1, entry(2, 1, 0, 0x8000 | (2 << 9) | (0x81 << 1) | 1));
    let mut client = vec![0xa5; BUFFER];
    assert!(io.read_into(regs, 0, 2, &mut client).is_ok());
    let (read, rest) = client.split_at(1024);
    assert!(read.iter().all(|&b| b == 0x3c) && rest.iter().all(|&b| b == 0xa5));
    let mut client = vec![0xa5; BUFFER];
    let r = io.read_into(regs, 0, 64, &mut client);
    assert!(matches!(r, Err(NvmeError::AdminCommandFailed)), "{r:?}");
    assert!(client.iter().all(|&b| b == 0xa5), "a failed read handed bytes back");
}
