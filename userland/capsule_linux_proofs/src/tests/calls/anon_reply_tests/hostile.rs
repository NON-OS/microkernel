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

//! Hostile replies end in an errno, never a panic.

use super::super::super::random::Regs;
use super::super::mutation::damage;
use super::server::server_reply;
use crate::linux::abi::errno::{EAGAIN, ECONNREFUSED, EIO, ENETUNREACH, ENOBUFS, EPIPE};
use crate::linux::net::anon_answer::{got, opened, sent};
use crate::linux::net::anon_ops::*;
use crate::linux::net::anon_wire::{reply, REQUEST_ID};

#[test]
fn hostile_replies_end_in_an_errno_and_never_a_panic() {
    let mut r = Regs::new(0x0a40_41e5);
    let mut seed = 0x0dd5_eed0_u64;
    let statuses = [
        E_OK,
        E_NO_LINK,
        E_TABLE_FULL,
        E_NO_STREAM,
        E_RX_EMPTY,
        E_STREAM_CLOSED,
        E_WOULD_BLOCK,
        77,
    ];
    for round in 0..20_000 {
        let op = [OP_OPEN_STREAM, OP_SEND, OP_RECV][round % 3];
        let status = statuses[r.small(statuses.len() as u64) as usize];
        let body: Vec<u8> = (0..r.small(9)).map(|_| r.any() as u8).collect();
        let mut raw = server_reply(op, status, REQUEST_ID, &body);
        if r.small(2) == 0 {
            damage(&mut seed, &mut raw);
        }
        let read = reply(&raw, op);
        if let Some((_, b)) = read {
            assert!(b.len() <= PAYLOAD_MAX);
        }
        match opened(read) {
            Ok(_) => assert!(matches!(read, Some((E_OK, b)) if b.len() == 2)),
            Err(e) => assert!([ENETUNREACH, ECONNREFUSED, ENOBUFS, EIO].contains(&e), "open: {e}"),
        }
        let asked = r.small(64) as usize;
        match sent(read, asked) {
            Ok(n) => assert!(n <= asked),
            Err(e) => assert!([EAGAIN, EPIPE, EIO].contains(&e), "send: {e}"),
        }
        let _ = got(read);
    }
}
