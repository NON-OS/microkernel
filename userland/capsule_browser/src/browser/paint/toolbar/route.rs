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

use crate::browser::net::mixnet::{self, Network};
use crate::browser::shell::route::Route;
use crate::browser::state::State;

/* The route the next request leaves by. A manual proxy only carries
 * traffic on the direct network, so it is named there. */
pub fn route_of(state: &State) -> Route {
    match (mixnet::chosen(), state.proxy.is_some()) {
        (Network::Nym, _) => Route::Mixnet,
        (Network::Anyone, _) => Route::Anyone,
        (Network::Direct, true) => Route::Socks,
        (Network::Direct, false) => Route::Direct,
    }
}
