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

//! What a box may hold when a terminal starts it. A terminal run has no
//! network in this release (its role holds no Network, and the BusyBox and
//! every unlisted program hold no NET_ANON), so a box naming a destination
//! would promise what no run can have; it is refused instead.

use super::preset::{parse, BoxPolicy};

pub const NO_NETWORK: &str = "a terminal run has no network in this release";

pub fn for_terminal(word: &[u8]) -> Result<BoxPolicy, &'static str> {
    let policy = parse(word)?;
    match policy.dests.is_empty() {
        true => Ok(policy),
        false => Err(NO_NETWORK),
    }
}
