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

//! One connection to the RPC host, over Nym or Anyone through route link.
//!
//! Every balance, nonce, fee, broadcast and receipt goes over one of these.
//! The host name goes unresolved to the exit through net.socks5 or net.anon,
//! so the RPC provider never sees this machine's address beside the accounts
//! it is asked about. A route that is down refuses with its reason, and
//! nothing falls back: there is no Direct path in the wallet. A link is
//! opened and carried a step at a time (`step::open`, `step::io`), so the
//! window never waits on one.

use nonos_route_link::RouteStream;

/// A stream through net.socks5 or net.anon, waiting up to `patience_ms` for
/// the far end's first byte. Dropping it resets its tunnel.
pub struct Link {
    pub stream: RouteStream,
    pub patience_ms: u64,
}
