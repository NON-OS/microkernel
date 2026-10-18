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

//! When `d` asks for a direct download, and the route an install takes.

use crate::net::Route;
use crate::path::offers_direct;

use super::ROUTE_REASONS;

/// Whether `d` asks for a direct download for this tier: before Install,
/// when there is an offer to make; or after an install through Nym or
/// Anyone stopped because the network or its exits did not answer (`why`,
/// the reason it stopped with), whatever its size.
pub fn direct_offered(route: Route, left: u64, uninstalled: bool, why: Option<u8>) -> bool {
    /* Installs go over Anyone; still starting counts as Anyone. */
    let anonymous = !matches!(route, Route::Direct);
    let stuck = why.is_some_and(|w| ROUTE_REASONS.contains(&w));
    (uninstalled && offers_direct(installs(route), left)) || (anonymous && stuck)
}

/// The route an install's download takes: Anyone, also while it is still
/// starting (`Route::for_installs` is no route until net.anon registers).
pub fn installs(route: Route) -> Route {
    match route {
        Route::Down(_) => Route::Anon(0),
        r => r,
    }
}
