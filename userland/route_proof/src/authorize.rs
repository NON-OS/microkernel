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

//! Who may put a report on the board.

use crate::report::Network;

/// The Network capability bit (abi/caps.toml).
pub const CAP_NETWORK: u64 = 1 << 2;

/// Whether the process named `name`, holding `caps`, may report on `network`.
///
/// Only the transport for that network, and only while it holds Network: a
/// process without Network cannot be carrying anybody's traffic, so its
/// account of a route would be a claim about something it cannot do. The name
/// is the one the kernel gave the process at spawn and shows in its process
/// table, not one the sender writes into the message.
pub fn may_report(name: &[u8], caps: u64, network: Network) -> bool {
    if caps & CAP_NETWORK == 0 {
        return false;
    }
    match network {
        Network::Nym => name == b"net.nym",
        Network::Anyone => name == b"net.anon",
    }
}
