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

//! A stream to an address outside the family. It goes over the mixnet,
//! never the open network, and the guest holds no capability that could
//! name a socket: there is no second route to disable and no firewall rule
//! to remove. net.sockets holds the stream; the family's entry names it.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::connect_dial::{dial, open};
use super::sock::{self, Addr};

pub fn connect(guest: &Guest, id: u32, to: Addr) -> u64 {
    if sock::with(|t| t.get(id).is_some_and(|s| s.connected || s.listening || s.svc.is_some())) {
        return errno::fail(errno::EISCONN);
    }
    let handle = match open() {
        Ok(h) => h,
        Err(e) => return e,
    };
    let status = match dial(guest, handle, to) {
        Ok(s) => s,
        Err(e) => {
            super::stream::close(handle);
            return e;
        }
    };
    let answer = match status {
        Some((0, _)) => errno::ok(0),
        Some(_) => errno::fail(errno::ECONNREFUSED),
        None => errno::fail(errno::EIO),
    };
    if answer != 0 {
        super::stream::close(handle);
        return answer;
    }
    sock::with(|t| {
        if let Some(s) = t.get_mut(id) {
            s.svc = Some(handle);
            s.remote = Some(to);
            s.connected = true;
        }
    });
    answer
}
