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

//! A stream net.anon opened, taken on by the socket that asked for it, as
//! a connect that opened at once takes it on (sock/adopt.rs): the socket's
//! own net.anon stream from then on, with the SOCKS endpoint's replies, for
//! its conversation, in front of the stream's first bytes.

use super::super::sock::{self, Backend};
use super::drive::Drove;
use super::socks::reply::{reply, SUCCEEDED};
use super::table;

/// Opened by net.anon as its stream `sid`: the socket takes it on, with
/// the endpoint's replies in front of the stream's first bytes.
pub fn adopt(id: u32, h: u32, port: u32, sid: u16, asked: bool) -> Drove {
    let socks = table::with(h, |e| e.socks.map(|ver| (ver, e.out.clone()))).flatten();
    let took = sock::with(|t| {
        let to = t.get(id).and_then(|s| s.remote).unwrap_or_default();
        t.adopt_anon(id, port, sid, to)?;
        if let Some(s) = t.get_mut(id) {
            s.opened_late = socks.is_none() && !asked;
            if let (Some((ver, said)), Some(a)) =
                (&socks, s.svc.as_mut().and_then(Backend::anon_mut))
            {
                a.rx = [&said[..], &reply(*ver, SUCCEEDED)].concat();
            }
        }
        Ok(())
    });
    match took {
        Ok(()) => {
            drop(table::remove(h));
            Drove::Opened
        }
        Err(errno) => {
            table::with(h, |e| e.why.1 = TAKEN);
            super::apply::failed(id, h, errno, asked)
        }
    }
}

/// Why a stream net.anon opened was not taken on: its id is one a socket
/// here holds already, which would join two guests' streams (sock/adopt.rs).
const TAKEN: &str = "net.anon gave a stream an id this capsule already holds";
