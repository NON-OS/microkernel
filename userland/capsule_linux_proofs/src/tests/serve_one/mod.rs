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

//! SERVE_ONE as the personality ships it (docs/design/serve-one.md): the
//! one listener a box may take (P.4, P.5), the door end its capsule's
//! connection comes through (P.6 to P.8), and the wire's server (P.9, P.10).

mod close_tests;
mod door_tests;
mod gate_tests;
mod retry_tests;
mod room_tests;
mod wire_tests;

use crate::linux::net::serve_one::server::Server;
use crate::linux::net::sock::{Addr, Domain, Proto, Socks};
use nonos_serve_wire::{decode, put, Header, HEADER, OK, OPEN};

/// The listener a box serving app.simplex on 5225 binds.
const AT: Addr = Addr { ip: [127, 0, 0, 1], port: 5225 };
const GUEST: u32 = 40;
/// The pid owning the capsule's endpoint, which opens the connection.
const OPENER: u32 = 70;
const NOW: u64 = 1_000;

/// The guest's listener at AT, as its bind and listen leave it.
fn listening(backlog: usize) -> (Socks, u32) {
    let mut t = Socks::new();
    let l = t.open(Domain::Inet, Proto::Stream, Some(GUEST));
    let s = t.get_mut(l).unwrap();
    (s.local, s.listening, s.backlog) = (Some(AT), true, backlog);
    (t, l)
}

/// A question, as the capsule asks it.
fn frame(op: u8, conn: u16, body: &[u8]) -> Vec<u8> {
    let mut out = vec![0u8; HEADER + body.len()];
    let n = put(&mut out, Header { op, status: 0, conn, len: 0 }, body).unwrap();
    out.truncate(n);
    out
}

/// The last answer to `frame` from `from`, None while it is held.
fn ask(s: &mut Server, t: &mut Socks, from: u32, frame: &[u8]) -> Option<(Header, Vec<u8>)> {
    s.ask(t, from, 1, frame, NOW);
    s.out.pop().map(|(_, f)| answered(&f))
}

fn answered(f: &[u8]) -> (Header, Vec<u8>) {
    let (h, body) = decode(f).unwrap();
    (h, body.to_vec())
}

/// A connection OPENER opened and the guest accepted: its id, the guest's end.
fn opened(s: &mut Server, t: &mut Socks, l: u32) -> (u16, u32) {
    let (h, _) = ask(s, t, OPENER, &frame(OPEN, 0, &[])).unwrap();
    assert_eq!(h.status, OK);
    let (end, _) = t.accept(l, GUEST).unwrap();
    (h.conn, end)
}
