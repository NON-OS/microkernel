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

//! What a box may hold when a terminal starts it. A box naming destinations
//! runs only in the terminal role that holds Network (app.linux.term.net),
//! and that role runs only such a box: the plain terminal role never reaches
//! the network, and the network role is never a box without a list.

use super::preset::{parse, BoxPolicy};

/// A box with destinations, started without the network role.
pub const NO_NETWORK: &str = "a box naming destinations runs as `linux` with the network role";
/// The network role, started for a box with no destination.
pub const NO_LIST: &str = "the network role runs only a box that names destinations";

pub fn for_terminal(word: &[u8], net_role: bool) -> Result<BoxPolicy, &'static str> {
    let policy = parse(word)?;
    match (policy.dests.is_empty(), net_role) {
        (true, false) | (false, true) => Ok(policy),
        (false, false) => Err(NO_NETWORK),
        (true, true) => Err(NO_LIST),
    }
}
