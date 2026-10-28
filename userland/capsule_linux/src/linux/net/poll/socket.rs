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
//! the table, a stream outside the family from routed/, or from what
//! net.anon has brought into the socket's buffer (sock/anon_rx.rs).

use super::super::sock::{self, Backend, Via};

const POLLNVAL: u16 = 0x020;

pub(super) fn socket_bits(id: u32) -> u16 {
    if let Some(bits) = sock::bits(id) {
        return bits;
    }
    match sock::with(|t| t.get(id).and_then(|s| s.svc.as_ref().map(Backend::via))) {
        /* A stream net.anon opened as it was looked at is answered as one. */
        Some(Via::Routed(_)) => super::super::routed::bits(id)
            .or_else(|| super::super::anon_stream::bits(id))
            .unwrap_or(POLLNVAL),
        Some(Via::Anon) => super::super::anon_stream::bits(id).unwrap_or(POLLNVAL),
        None => POLLNVAL,
    }
}

/// True when `id` is a stream routed/ or net.anon holds: nothing tells the
/// family when it changes, so a wait on it is looked at again on a tick.
pub fn outside(id: u32) -> bool {
    sock::with(|t| t.get(id).is_some_and(|s| s.svc.is_some()))
}
