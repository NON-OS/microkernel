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
use crate::browser::paint::tokens::palette;
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

/* What `route` is called, with its badge fill and ink: an anonymising
 * network in teal, the direct one plain. */
pub fn look(route: Route) -> (&'static str, u32, u32) {
    match route {
        Route::Mixnet | Route::Anyone => (route.name(), palette().teal_soft, palette().teal),
        Route::Socks | Route::Direct => (route.name(), palette().field, palette().muted),
    }
}

/* The network the next request leaves through, as the badge shows it. */
pub fn label(state: &State) -> (&'static str, u32, u32) {
    look(route_of(state))
}
