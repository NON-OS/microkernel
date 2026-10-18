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

//! The server loop at the IPC boundary, against hostile requests: anything
//! that is not one whole request for a known op is refused by status, and a
//! message with no sender, or no bytes, is not answered at all.

use super::memory::Memory;
use super::model::window;
use super::serve::{ask, request, serve, status};
use crate::protocol::{E_INVAL, E_MSGSIZE, HDR_LEN, OP_HEALTHCHECK, OP_LINK_STATUS};

#[test]
fn hostile_requests_are_refused_by_status() {
    let bar = window();
    let mut mem = Memory::new();
    let mut d = mem.driver(&bar);
    let mut bad_magic = request(OP_HEALTHCHECK, &[], 0);
    bad_magic[0] ^= 0xFF;
    let cases = [
        ((7, vec![0u8; HDR_LEN - 1]), E_INVAL),
        ((7, bad_magic), E_INVAL),
        ((7, request(OP_HEALTHCHECK, &[], 1)), E_MSGSIZE),
        ((7, request(OP_HEALTHCHECK, &[1, 2], 1)), E_MSGSIZE),
        ((7, request(OP_LINK_STATUS, &[], u32::MAX)), E_MSGSIZE),
        (ask(0, &[]), E_INVAL),
        (ask(u16::MAX, &[]), E_INVAL),
    ];
    for (req, want) in cases {
        let got = serve(&mut d, std::slice::from_ref(&req));
        assert_eq!(got.len(), 1, "one reply to {req:?}");
        assert_eq!((got[0].0, status(&got[0].1)), (7, want), "{req:?}");
    }
}

#[test]
fn no_sender_or_no_bytes_gets_no_reply_and_the_next_one_is_served() {
    let bar = window();
    let mut mem = Memory::new();
    let mut d = mem.driver(&bar);
    let quiet = [(0, request(OP_HEALTHCHECK, &[], 0)), (7, vec![]), ask(OP_HEALTHCHECK, &[])];
    let got = serve(&mut d, &quiet);
    assert_eq!(got.len(), 1, "only the last was answered");
    assert_eq!((got[0].0, status(&got[0].1)), (7, 0));
}
