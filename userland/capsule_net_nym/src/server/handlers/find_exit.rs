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

//! Finding an exit that answers.

use alloc::vec::Vec;

use crate::directory_sync::{cached_requesters, refresh_requesters, ExitAddress};
use crate::setup;
use crate::topology::{self, Role};
use crate::trace;

/// A requester on a known exit gateway, starting from `index`.
///
/// Callers pass an index so they can ask for a different one than last time;
/// net.socks5 moves on when an exit stays silent. Requesters come from the
/// validator's described nodes over TLS, and one is offered only while the
/// gateway its address names is an exit gateway in the current node list, so
/// every packet this returns an address for leaves by a gateway the directory
/// vouched for.
pub fn find_exit(index: usize) -> Option<ExitAddress> {
    let nodes = topology::snapshot().ok()?;
    let exits: Vec<[u8; 32]> =
        nodes.iter().filter(|n| n.role == Role::ExitGateway).map(|n| n.identity).collect();
    if exits.is_empty() {
        trace::say(b"exit lookup: the directory lists none");
        return None;
    }
    let is_exit = |id: &[u8; 32]| exits.iter().any(|e| e == id);
    let mut usable: Vec<ExitAddress> =
        cached_requesters().into_iter().filter(|r| is_exit(&r.gateway)).collect();
    if usable.is_empty() {
        match refresh_requesters(setup::tcp_port(), is_exit) {
            Ok(n) => trace::say_num(b"exit lookup: requesters on known exits", n as u64),
            Err(e) => {
                trace::say_num(b"exit lookup: described nodes failed, code", e as u64);
                return None;
            }
        }
        usable = cached_requesters().into_iter().filter(|r| is_exit(&r.gateway)).collect();
    }
    if usable.is_empty() {
        trace::say(b"exit lookup: no requester on a known exit");
        return None;
    }
    trace::say_num(b"exit lookup: candidates", usable.len() as u64);
    Some(usable[index % usable.len()])
}
