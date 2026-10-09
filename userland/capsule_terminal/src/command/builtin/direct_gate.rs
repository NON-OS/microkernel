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

//! Commands that can only leave directly, and the line they print instead
//! when the person chose an anonymous network.
//!
//! An ICMP echo, a name looked up in the clear and the plain TCP `pull` and
//! `push` open to an address they resolved themselves have no anonymity
//! network to cross. Each one
//! named this machine to the host and to the resolver whatever network had
//! been chosen. They now run only when Direct is the default, the rule
//! nonos_route_link holds for every capsule, and otherwise say why nothing
//! was sent.

use alloc::vec::Vec;

/// What `ping` would have done.
pub const PING: &[u8] = b"ICMP cannot cross it and would leave directly";

/// What `nslookup` would have done.
pub const NSLOOKUP: &[u8] = b"a lookup would leave in the clear";

/// What `pull` would have done, and what goes the chosen way instead.
pub const PULL: &[u8] =
    b"pull reaches the host directly (curl and git go through the chosen network)";

/// What `push` would have done, and what goes the chosen way instead.
pub const PUSH: &[u8] =
    b"push reaches the host directly (curl and git go through the chosen network)";

/// The line `command` prints instead of doing `what`, given the route's
/// reason for refusing a direct contact, or None when it may go ahead.
pub fn refusal_line(command: &[u8], what: &[u8], refused: Option<&str>) -> Option<Vec<u8>> {
    let why = refused?;
    let mut line = Vec::with_capacity(command.len() + why.len() + what.len() + 28);
    line.extend_from_slice(command);
    line.extend_from_slice(b": ");
    line.extend_from_slice(why.as_bytes());
    line.extend_from_slice(b"; ");
    line.extend_from_slice(what);
    line.extend_from_slice(b", so nothing was sent");
    Some(line)
}
