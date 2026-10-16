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

//! The PRP entries a transfer names: one page in PRP1, two in PRP1 and PRP2,
//! more through the list page, whose entries walk the data buffer page by
//! page. Every byte count the queue accepts stays inside the buffer, and a
//! write's buffer holds exactly the client's bytes.

use super::constants::{DATA_BYTES, PAGE};
use super::model::{io_queue, post};
use super::prp::build_prp;
use crate::admin::Submission;
use crate::completion_tests::entry;

/// The list page's entry `i`.
fn listed(list_va: u64, i: u64) -> u64 {
    // SAFETY: `i` is below the entries a transfer of DATA_BYTES writes,
    // inside the queue's one-page list.
    unsafe { core::ptr::read_volatile((list_va + i * 8) as *const u64) }
}

#[test]
fn one_two_and_eight_pages() {
    let io = io_queue();
    let data = io.data.device_addr();
    for bytes in [1, 512, PAGE] {
        assert_eq!(build_prp(&io, bytes), (data, 0), "{bytes} bytes");
    }
    for bytes in [PAGE + 1, 2 * PAGE] {
        assert_eq!(build_prp(&io, bytes), (data, data + PAGE), "{bytes} bytes");
    }
    for bytes in [2 * PAGE + 1, DATA_BYTES] {
        assert_eq!(build_prp(&io, bytes), (data, io.prp_list.device_addr()), "{bytes} bytes");
        for i in 0..bytes.div_ceil(PAGE) - 1 {
            assert_eq!(listed(io.prp_list.user_va(), i), data + PAGE * (i + 1), "entry {i}");
        }
    }
    assert_eq!(DATA_BYTES, 8 * PAGE, "eight pages at most");
}

#[test]
fn a_write_names_the_buffer_holding_exactly_the_clients_bytes() {
    nonos_libc::set_clock(1_000);
    let (regs, mut io) = (crate::fake::registers(), io_queue());
    post(&io, 0, entry(1, 1, 0, 1));
    let src: Vec<u8> = (0..1024u32).map(|i| (i % 251) as u8).collect();
    assert!(io.write_from(regs, 9, 2, &src).is_ok());
    // SAFETY: the queue's own submission ring, whose slot 0 the write took.
    let cmd = unsafe { core::ptr::read_volatile(io.sq.user_va() as *const Submission) };
    let named = (cmd.cdw0 & 0xff, cmd.prp1, cmd.cdw10, cmd.cdw12);
    assert_eq!(named, (0x01, io.data.device_addr(), 9, 1), "write 2 blocks at 9");
    let at = io.data.user_va() as *const u8;
    // SAFETY: the queue's own data buffer, DATA_BYTES of host memory.
    let held = unsafe { core::slice::from_raw_parts(at, DATA_BYTES as usize) };
    assert_eq!(&held[..1024], &src[..]);
    assert!(held[1024..].iter().all(|&b| b == 0), "and nothing past them");
}
