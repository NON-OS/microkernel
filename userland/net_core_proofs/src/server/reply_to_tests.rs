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

//! A lookup held past its turn is answered by the kernel's id for the call
//! it came in, so the answer reaches that call and no later one from the
//! same client; the frame still names the client's own request id.

use nonos_libc::{serial, take_replies};

use super::parse_req::{HDR_LEN, IPC_BUF_MAX};
use super::respond::{reply, reply_to};
use crate::protocol::dns::{E_OK, MAGIC_NDNS, OP_RESOLVE_A};

fn le32(b: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([b[off], b[off + 1], b[off + 2], b[off + 3]])
}

#[test]
fn a_held_answer_goes_by_the_kernel_id_it_was_received_under() {
    let _guard = serial();
    let _ = take_replies();
    let mut tx = vec![0u8; IPC_BUF_MAX];
    let ip = [10, 0, 2, 3];
    let n = reply_to(0x1_0000_0007, (MAGIC_NDNS, OP_RESOLVE_A), E_OK, 41, &ip, &mut tx);
    assert_eq!(n, (HDR_LEN + ip.len()) as i64);
    let replies = take_replies();
    assert_eq!(replies.len(), 1);
    let r = &replies[0];
    assert_eq!((r.pid, r.request), (0, 0x1_0000_0007), "by the id, not a pid");
    assert_eq!((le32(&r.bytes, 0), le32(&r.bytes, 12)), (MAGIC_NDNS, 41));
    assert_eq!(u16::from_le_bytes([r.bytes[6], r.bytes[7]]), OP_RESOLVE_A);
    assert_eq!(le32(&r.bytes, 16), ip.len() as u32);
    assert_eq!(&r.bytes[HDR_LEN..], &ip);
}

#[test]
fn an_answer_in_turn_goes_to_the_sender_with_no_id() {
    let _guard = serial();
    let _ = take_replies();
    let mut tx = vec![0u8; IPC_BUF_MAX];
    let _ = reply(31, MAGIC_NDNS, OP_RESOLVE_A, E_OK, 41, &[], &mut tx);
    let replies = take_replies();
    assert_eq!((replies.len(), replies[0].pid, replies[0].request), (1, 31, 0));
    assert_eq!(le32(&replies[0].bytes, 12), 41);
}
