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

//! A socket's readiness in poll's bits: the family's own sockets answer from
//! the table, a stream outside the family from net.sockets.

use super::call::call;
use super::ops::{OP_POLL, POLL_READABLE, POLL_WRITABLE};
use super::sock;

const POLLIN: u16 = 0x001;
const POLLOUT: u16 = 0x004;
const POLLNVAL: u16 = 0x020;

pub(super) fn socket_bits(id: u32) -> u16 {
    if let Some(bits) = sock::bits(id) {
        return bits;
    }
    match sock::with(|t| t.get(id).and_then(|s| s.svc)) {
        Some(handle) => service_bits(handle),
        None => POLLNVAL,
    }
}

/// True when `id` is a stream net.sockets holds: nothing tells the family
/// when it changes, so a wait on it is looked at again on a tick.
pub fn outside(id: u32) -> bool {
    sock::with(|t| t.get(id).is_some_and(|s| s.svc.is_some()))
}

fn service_bits(handle: u32) -> u16 {
    let Some((0, out)) = call(OP_POLL, &handle.to_le_bytes(), 1) else {
        return 0;
    };
    let Some(bits) = out.first() else {
        return 0;
    };
    let mut set = 0;
    if bits & POLL_READABLE != 0 {
        set |= POLLIN;
    }
    if bits & POLL_WRITABLE != 0 {
        set |= POLLOUT;
    }
    set
}
