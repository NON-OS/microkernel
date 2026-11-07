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

//! One fuzz input run against a broker, shared by the host test and the
//! cargo-fuzz target: the bytes as one request, then as a stream of
//! requests cut at each length prefix, then as a manifest body, a journal record and a slot.
//! Nothing may panic, and every reply must be a whole reply to its request.

use nonos_user_files::manifest::{apply, decode, first_body, parse, Slot};
use nonos_user_files::wire::{Header, HEADER};
use nonos_user_files::{serve, Broker};

use crate::doubles::{Desk, Volume};

/// The pid fuzzed requests come from.
pub const FUZZ_PID: u32 = 7;

pub fn run(b: &mut Broker<Volume>, desk: &Desk, input: &[u8]) {
    whole(input, &serve(b, desk, FUZZ_PID, input));
    let mut rest = input;
    while rest.len() >= 2 {
        let len = (u16::from_le_bytes([rest[0], rest[1]]) as usize).min(rest.len() - 2);
        let body = &rest[2..2 + len];
        let mut msg = vec![0u8; HEADER];
        let op = u16::from(body.first().copied().unwrap_or(0) % 18);
        let mut head = [0u8; HEADER];
        Header { op, id: len as u32, body_len: body.len() as u32 }.write(&mut head);
        msg.copy_from_slice(&head);
        msg.extend_from_slice(body);
        whole(&msg, &serve(b, desk, FUZZ_PID, &msg));
        rest = &rest[2 + len..];
    }
    let _ = decode(input);
    if let Some(mut p) = parse(&first_body()) {
        let _ = apply(&mut p, input);
    }
    let _ = Slot::decode(input);
    let _ = b.tick(u64::MAX);
}

fn whole(request: &[u8], reply: &[u8]) {
    let (h, body) = Header::read(reply).expect("a reply with a whole header");
    assert!(body.len() >= 4, "a reply with no status");
    if let Some((asked, _)) = Header::read(request) {
        assert_eq!((h.op, h.id), (asked.op, asked.id), "a reply to another request");
    }
}
