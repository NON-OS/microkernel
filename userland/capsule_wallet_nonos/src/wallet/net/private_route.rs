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

//! The one rule every connection the wallet opens is held to: Nym or Anyone,
//! never Direct. `WalletRoute` has no Direct to hold (`from_link` turns one
//! into no route); this is the last gate before a stream opens, so a route
//! that is down opens nothing.

use super::wallet_route::WalletRoute;

/// What the wallet says while neither private network is running. The tick
/// asks again on its own schedule; nothing ever falls back to Direct.
pub const WAITING: &str =
    "Waiting for Nym or Anyone: the wallet reaches the chain over nothing else.";

/// None when `route` may carry the wallet's traffic; else why nothing opens.
pub fn refused(route: WalletRoute) -> Option<&'static str> {
    match route {
        WalletRoute::Nym(_) | WalletRoute::Anon(_) => None,
        WalletRoute::Down(_) => Some(WAITING),
    }
}
