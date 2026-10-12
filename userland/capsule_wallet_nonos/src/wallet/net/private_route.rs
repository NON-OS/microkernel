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
//! never Direct. `Route::for_wallet` already turns a Direct default into Nym,
//! or Anyone when Nym is down; this is the last gate before a stream opens,
//! so even a Direct handed in by mistake opens nothing.

use super::Route;

/// What the wallet says while neither private network is running. The tick
/// asks again on its own schedule; nothing ever falls back to Direct.
pub const WAITING: &str =
    "Waiting for Nym or Anyone: the wallet reaches the chain over nothing else.";

/// None when `route` may carry the wallet's traffic; else why nothing opens.
pub fn refused(route: Route) -> Option<&'static str> {
    match route {
        Route::Nym(_) | Route::Anon(_) => None,
        Route::Direct | Route::Down(_) => Some(WAITING),
    }
}
