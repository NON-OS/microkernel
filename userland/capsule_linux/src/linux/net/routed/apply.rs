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

//! What a step came to, applied to the socket. A stream open through
//! net.socks5 stays in the table (one net.anon opened is adopt.rs's); one
//! that did not open is said in the run's window and told to the guest: by
//! its connect, by SO_ERROR, or as the SOCKS endpoint's reply.

use super::super::sock;
use super::drive::Drove;
use super::entry::Phase;
use super::socks::reply::{code_of, reply, SUCCEEDED};
use super::table;

/// Opened through net.socks5. A connect that has returned already learns
/// it from the next one, which answers 0 once, as Linux's does.
pub fn opened(id: u32, h: u32, asked: bool) -> Drove {
    match table::with(h, |e| e.socks).flatten() {
        Some(ver) => {
            table::with(h, |e| e.out.extend_from_slice(&reply(ver, SUCCEEDED)));
        }
        None => sock::with(|t| {
            if let Some(s) = t.get_mut(id) {
                (s.connected, s.opened_late) = (true, !asked);
            }
        }),
    }
    Drove::Opened
}

/// Not opened, with `errno`: said in the window, then told to the guest.
pub fn failed(id: u32, h: u32, errno: i64, asked: bool) -> Drove {
    let said = table::with(h, |e| {
        match e.socks {
            Some(ver) => {
                e.out.extend_from_slice(&reply(ver, code_of(errno)));
                e.phase = Phase::Closed;
            }
            None => e.phase = Phase::Failed(errno),
        }
        (e.why, e.socks.is_none())
    });
    let Some(((net, why), plain)) = said else {
        return Drove::Failed(errno);
    };
    super::said::say(net, why);
    if plain && !asked {
        sock::with(|t| t.get_mut(id).map(|s| s.error = errno));
    }
    Drove::Failed(errno)
}
