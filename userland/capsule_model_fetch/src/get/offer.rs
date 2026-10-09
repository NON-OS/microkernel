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

/*
 * The path `qwen get` takes and what it says of it first (`crate::path`):
 * the chosen network's route unless the person asked for a direct download
 * for this one, and before a large download through an anonymity network,
 * the estimate and the offer of a direct one. Pure, so model_fetch_proofs
 * holds the decision.
 */

use alloc::format;
use alloc::string::String;

use crate::net::Route;
use crate::path::{duration, network, offers_direct};
use crate::size::size;

/*
 * The route a download takes: direct only when the person asked for it
 * for this download, else the one the system's default network gives,
 * which may be no route at all.
 */
pub fn route_for(chosen: Route, direct_asked: bool) -> Route {
    match direct_asked {
        true => Route::Direct,
        false => chosen,
    }
}

/* The route as `qwen get` says it: "downloading over Anyone". */
pub fn route_said(route: Route) -> &'static str {
    match route {
        Route::Anon(_) => "downloading over Anyone",
        Route::Nym(_) => "downloading through the Nym mixnet",
        Route::Direct => "downloading over a direct connection, as asked",
        Route::Down(why) => why,
    }
}

/*
 * Before a large download through an anonymity network: the path, the
 * estimate at the rate assumed for it, and the offer. None for any other.
 */
pub fn offer_line(route: Route, left: u64, tier: &str) -> Option<String> {
    let (net, rate) = network(route).filter(|_| offers_direct(route, left))?;
    Some(format!(
        "{} through {net} takes {} at the {}/s assumed for it; `qwen get --direct {tier}` \
         downloads it direct instead: faster, but the mirror sees this machine's address",
        size(left),
        duration(left / rate),
        size(rate)
    ))
}
