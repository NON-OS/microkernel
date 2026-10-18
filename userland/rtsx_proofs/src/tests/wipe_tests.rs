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

//! C8: no request's data stays behind once it is answered: not in the DMA
//! buffer a write went through, with no read after it to overwrite it, and
//! not in the IPC buffers the service loop receives into and replies from.

use nonos_libc::{push_request, take_sent};

use super::rig::{sdhc, serving};
use super::service_rig::{call, request, span, KERNEL};
use crate::protocol::*;
use crate::serve::run::serve_one;

const DATA: usize = MAX_RW_BLOCKS as usize * BLOCK_BYTES;

#[test]
fn a_write_leaves_nothing_in_the_data_buffer() {
    let (mut drv, mut medium) = serving(sdhc(1));
    let w = call(&mut drv, &mut medium, KERNEL, OP_WRITE, &[span(3, 8), vec![0x5A; 4096]].concat());
    assert_eq!(w.status, E_OK);
    assert!((0..4096).all(|i| drv.data.read_u8(i) == 0), "the write's data left in the buffer");
}

#[test]
fn both_ipc_buffers_are_wiped_once_a_request_is_answered() {
    let (mut drv, mut medium) = serving(sdhc(1));
    let mut rx = vec![0u8; HDR_LEN + RW_HEADER_LEN + DATA];
    let mut tx = vec![0u8; HDR_LEN + STATUS_LEN + DATA];
    let write = request(OP_WRITE, 1, &[span(3, 8), vec![0x5A; 4096]].concat());
    for (op, body) in [(write, 0), (request(OP_READ, 2, &span(3, 8)), 4096)] {
        push_request(KERNEL, op);
        serve_one(&mut drv, &mut medium, &mut rx, &mut tx);
        let sent = take_sent();
        assert_eq!(sent.len(), 1);
        assert_eq!(sent[0].1.len(), HDR_LEN + STATUS_LEN + body, "the request was answered");
        assert!(rx.iter().all(|&b| b == 0), "the request left in the receive buffer");
        assert!(tx.iter().all(|&b| b == 0), "the reply left in the send buffer");
    }
}
