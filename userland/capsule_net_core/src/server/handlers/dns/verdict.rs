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

//! What a lookup asked of every server the lease named comes to. Kept free
//! of smoltcp so the host proofs include it unchanged.

use crate::protocol::dns::{E_SERVFAIL, E_TIMEOUT};

/// What one server's query holds, or the lookup as a whole.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Found {
    Pending,
    Address([u8; 4]),
    Failed,
}

/// The first address any server gave. Failed once every server asked has
/// failed, so one server down or refusing does not fail the lookup while
/// another may still answer; pending otherwise.
pub fn combine(each: impl IntoIterator<Item = Found>) -> Found {
    let mut pending = false;
    for found in each {
        match found {
            Found::Address(ip) => return Found::Address(ip),
            Found::Pending => pending = true,
            Found::Failed => {}
        }
    }
    if pending {
        Found::Pending
    } else {
        Found::Failed
    }
}

/// What a lookup that ended without an address is answered with: still
/// pending at its deadline means no server answered at all (E_TIMEOUT);
/// anything else is a server that answered without one (E_SERVFAIL).
pub fn failure_errno(found: Found) -> u16 {
    match found {
        Found::Pending => E_TIMEOUT,
        _ => E_SERVFAIL,
    }
}
