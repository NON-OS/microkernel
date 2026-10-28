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

//! The wire's server: each question from the capsule SERVE_ONE serves,
//! answered from the family's socket table. OPEN joins a door end to the
//! guest's listener (life.rs, sock/door.rs); WRITE, READ and CLOSE move
//! bytes through it. One connection at a time, keyed by its id and by the
//! pid that opened it. Answers wait in `out` for the pump to send.

use alloc::vec::Vec;

use nonos_serve_wire::{read_asked, written, Header, EBADF, EINVAL, OK, OPEN, READ, WRITE};

use super::answer::{answer, refused};
use super::conn::Conn;
use crate::linux::net::sock::{Addr, Socks};

pub struct Server {
    /// The listener the box serves: 127.0.0.1 at its field's port.
    pub at: Addr,
    pub conn: Option<Conn>,
    /// The id the next connection is given, never 0.
    pub next: u16,
    /// Answers ready, each with the request it answers.
    pub out: Vec<(u64, Vec<u8>)>,
}

impl Server {
    pub fn new(at: Addr) -> Self {
        Self { at, conn: None, next: 1, out: Vec::new() }
    }

    /// `frame` from pid `from`, which `request` answers. A READ with nothing
    /// to read yet is held (read.rs); a malformed frame delivers nothing.
    pub fn ask(&mut self, t: &mut Socks, from: u32, request: u64, frame: &[u8], now: u64) {
        let Ok((h, body)) = nonos_serve_wire::request(frame) else {
            return self.say(request, refused(frame), EINVAL, Vec::new());
        };
        if h.op == OPEN {
            return self.open(t, from, request, h);
        }
        let ours = self.conn.as_ref().is_some_and(|c| c.id == h.conn && c.pid == from);
        let Some(c) = self.conn.as_mut().filter(|_| ours) else {
            return self.say(request, h, EBADF, Vec::new());
        };
        let said = match h.op {
            WRITE => written(body).map(|(seq, bytes)| c.write(t, seq, bytes)),
            READ => read_asked(body).and_then(|ask| c.read(t, request, ask, now)),
            _ => {
                self.close(t);
                Some((OK, Vec::new()))
            }
        };
        if let Some((status, body)) = said {
            self.say(request, h, status, body);
        }
    }

    pub(super) fn say(&mut self, request: u64, h: Header, status: u8, body: Vec<u8>) {
        self.out.push((request, answer(h, status, &body)));
    }
}
