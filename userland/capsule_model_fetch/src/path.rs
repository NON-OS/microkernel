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
 * Which path a download takes, and what is said about it before and while
 * it comes. The person chose Nym or Anyone for privacy, so a download never
 * leaves by another network on its own: the chosen route is used unless the
 * person picked a direct download for this one download (`qwen get
 * --direct`, or `d` on the store's card). A download of more than
 * `ASK_OVER` through an anonymity network can take hours, so before it the
 * path and an estimate are said, with the offer of a direct download and
 * what it costs: the mirror sees this machine's address. The estimate
 * before any byte has come rests on a rate assumed for the network and is
 * said as assumed; once bytes come, the time left is worked from the rate
 * they came at, and only when enough has come for that to mean anything
 * (get/eta.rs).
 * Pure, so model_fetch_proofs holds every rule here. The fetcher's words
 * are get/offer.rs, the store's its own route_offer.rs, both on these.
 */

use alloc::format;
use alloc::string::String;

use crate::net::Route;

/* Above this, a download through an anonymity network is said with an estimate and the offer. */
pub const ASK_OVER: u64 = 1_000_000_000;
/*
 * Rates assumed for an estimate before a byte has come: what a download
 * through the mixnet and through the onion network commonly manages. Said
 * as assumed wherever an estimate rests on them.
 */
pub const NYM_RATE: u64 = 250_000;
pub const ANYONE_RATE: u64 = 1_000_000;

/* The network named in a sentence, and the rate assumed on it, if any. */
pub fn network(route: Route) -> Option<(&'static str, u64)> {
    match route {
        Route::Nym(_) => Some(("the Nym mixnet", NYM_RATE)),
        Route::Anon(_) => Some(("the Anyone onion network", ANYONE_RATE)),
        Route::Direct | Route::Down(_) => None,
    }
}

/* Whether a download of `left` bytes on `route` is said with an estimate and the offer. */
pub fn offers_direct(route: Route, left: u64) -> bool {
    network(route).is_some() && left > ASK_OVER
}

/* "about 5 h 33 min", "about 14 min", "under a minute". */
pub fn duration(secs: u64) -> String {
    match secs {
        s if s < 60 => String::from("under a minute"),
        s if s < 3_600 => format!("about {} min", s.div_ceil(60)),
        s => format!("about {} h {} min", s / 3_600, s % 3_600 / 60),
    }
}
