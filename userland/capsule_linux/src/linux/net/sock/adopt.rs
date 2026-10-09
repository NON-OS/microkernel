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

//! A stream net.anon opened, taken on by the socket that asked for it.

use crate::linux::abi::errno::{EBADF, EIO};

use super::backend::{Anon, Backend, Close};
use super::table::Socks;
use super::types::Addr;

impl Socks {
    /// True when a socket here already holds net.anon stream `sid`.
    pub fn holds_anon(&self, port: u32, sid: u16) -> bool {
        self.iter().any(|(_, s)| {
            s.svc
                .as_ref()
                .and_then(Backend::anon)
                .is_some_and(|a| a.port == port && a.id == Some(sid))
        })
    }

    /// Socket `id` is connected to `to` over net.anon stream `sid`.
    ///
    /// net.anon keys a stream by this capsule's pid and its id, so an id one
    /// socket here already holds, answered again for a new open, would join
    /// two guests' streams. It is refused (EIO) and not closed: closing it
    /// would end the stream its holder still reads. For a socket that is
    /// gone the new stream is closed again.
    pub fn adopt_anon(&mut self, id: u32, port: u32, sid: u16, to: Addr) -> Result<(), i64> {
        if self.holds_anon(port, sid) {
            return Err(EIO);
        }
        let Some(s) = self.get_mut(id) else {
            super::super::anon_stream::close(Close { port, id: sid });
            return Err(EBADF);
        };
        s.svc = Some(Backend::Anon(Anon::new(port, sid)));
        s.remote = Some(to);
        s.connected = true;
        Ok(())
    }
}
