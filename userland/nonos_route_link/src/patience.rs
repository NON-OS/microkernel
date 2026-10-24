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

//! How long a read waits for the far end's first bytes, by the network it
//! crosses, for a route and for a stream's proxy alike: one number per
//! network. A Nym or Anyone answer crosses several relays each way, the
//! mixnet delaying every packet on purpose, so it can be many seconds out.

use crate::pick::Route;
use crate::refusal::Proxy;

/// Through the Nym mixnet.
pub const NYM_PATIENCE_MS: u64 = 60_000;
/// Through the Anyone onion network, to a host or an onion service.
pub const ANYONE_PATIENCE_MS: u64 = 30_000;

impl Proxy {
    /// The patience of a stream through this proxy.
    pub fn patience_ms(self) -> u64 {
        match self {
            Proxy::Nym => NYM_PATIENCE_MS,
            Proxy::Anyone | Proxy::AnyoneService | Proxy::AnyoneName => ANYONE_PATIENCE_MS,
        }
    }
}

impl Route {
    /// A socket read already waits on the wire, so Direct adds nothing and
    /// its callers keep the windows they had.
    pub fn patience_ms(self) -> u64 {
        match self {
            Route::Nym(_) => NYM_PATIENCE_MS,
            Route::Anon(_) => ANYONE_PATIENCE_MS,
            Route::Direct | Route::Down(_) => 0,
        }
    }
}
