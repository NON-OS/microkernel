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

//! A stream out of the family on the Nym mixnet, which net.sockets holds
//! (connect_out.rs chose it).

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::super::ops::{NET_E_NAME_REFUSED, NET_E_NO_TRANSPORT};
use super::super::sock::{self, Addr, Backend};
use super::dial::{dial, open};

pub fn mixnet(guest: &Guest, id: u32, to: Addr) -> u64 {
    let handle = match open() {
        Ok(h) => h,
        Err(e) => return e,
    };
    let status = match dial(guest, handle, to) {
        Ok(s) => s,
        Err(e) => {
            super::super::stream::close(handle);
            return e;
        }
    };
    /*
     * No transport is a mixnet holding no gateway: there is no route out, which
     * a tool has to read as unreachable, not as a peer that answered and
     * refused.
     */
    /*
     * A name the mixnet cannot carry is no route out either: net.sockets
     * refused to resolve it in the clear, so nothing left the machine.
     */
    let answer = match status {
        Some((0, _)) => errno::ok(0),
        Some((NET_E_NO_TRANSPORT | NET_E_NAME_REFUSED, _)) => errno::fail(errno::ENETUNREACH),
        Some(_) => errno::fail(errno::ECONNREFUSED),
        None => errno::fail(errno::EIO),
    };
    if answer != 0 {
        super::super::stream::close(handle);
        return answer;
    }
    sock::with(|t| {
        if let Some(s) = t.get_mut(id) {
            s.svc = Some(Backend::Sockets(handle));
            s.remote = Some(to);
            s.connected = true;
        }
    });
    answer
}
