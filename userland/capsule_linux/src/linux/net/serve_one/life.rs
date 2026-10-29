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

//! A connection's life: OPEN queues its door end on the guest's listener,
//! the held READ is answered as the guest writes or closes, and CLOSE, or
//! its opener gone, lets the door end go so the guest reads end of file.

use alloc::vec::Vec;

use nonos_serve_wire::{Header, CLOSED, EBUSY, EINVAL, OK, READ};

use super::conn::Conn;
use super::server::Server;
use crate::linux::net::sock::Socks;

impl Server {
    /// OPEN: OK and the new conn once its door end is queued; EBUSY while
    /// one is open, ECONNREFUSED with no listener, EAGAIN with a full queue.
    pub(super) fn open(&mut self, t: &mut Socks, from: u32, request: u64, h: Header) {
        if self.conn.is_some() {
            return self.say(request, h, EBUSY, Vec::new());
        }
        match t.open_door(self.at) {
            Ok(end) => {
                let id = self.next;
                self.next = id.checked_add(1).unwrap_or(1);
                self.conn = Some(Conn::new(id, from, end));
                self.say(request, Header { conn: id, ..h }, OK, Vec::new());
            }
            Err(e) => self.say(request, h, u8::try_from(e).unwrap_or(EINVAL), Vec::new()),
        }
    }

    /// CLOSE, or its opener gone: the guest reads end of file, and a READ
    /// still held is answered CLOSED.
    pub fn close(&mut self, t: &mut Socks) {
        let Some(c) = self.conn.take() else { return };
        t.door_close(c.end);
        if let Some(held) = c.held {
            self.say(held.request, read_on(c.id), CLOSED, Vec::new());
        }
    }

    /// The held READ, answered once the guest wrote or closed or its wait ran out.
    pub fn settle(&mut self, t: &mut Socks, now: u64) {
        let Some(c) = self.conn.as_mut() else { return };
        let id = c.id;
        if let Some((request, status, body)) = c.settle(t, now) {
            self.say(request, read_on(id), status, body);
        }
    }
}

fn read_on(conn: u16) -> Header {
    Header { op: READ, status: 0, conn, len: 0 }
}
