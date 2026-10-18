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

//! What a Qwen tier's card says of the path its download takes, before
//! Install and while it comes: the network the system chose, and before a
//! download through an anonymity network larger than the fetcher's
//! threshold, the estimate and the offer of a direct download, which `d`
//! asks for (`crate::path`, the fetcher's own rule). Pure, so
//! model_fetch_proofs holds the words.

use alloc::format;
use alloc::string::String;

use crate::net::Route;
use crate::path::{duration, network, offers_direct};
use crate::size::size;

#[path = "route_direct.rs"]
mod route_direct;
#[path = "route_progress.rs"]
mod route_progress;

pub use route_direct::{direct_offered, installs};
pub use route_progress::progress_line;

/// The offer for `left` bytes still to come on `route`, where `d` asks for
/// the direct download; None when there is none to make.
pub fn store_offer(route: Route, left: u64) -> Option<String> {
    let (net, rate) = network(route).filter(|_| offers_direct(route, left))?;
    Some(format!(
        "Through {net}: {} at an assumed {}/s. Press d to download direct: faster, \
         but the mirror sees this machine's address",
        duration(left / rate),
        size(rate)
    ))
}

/// The path a download takes, as the card says it before Install: over
/// Anyone, which installs use whatever network the browser uses, waited
/// for when it is still starting.
pub fn path_line(route: Route) -> String {
    match route {
        Route::Anon(_) => String::from("Downloads over Anyone"),
        Route::Down(_) => String::from(
            "Downloads over Anyone, which is still starting: Install waits up to 3 minutes for it",
        ),
        Route::Nym(_) => String::from("Downloads through the Nym mixnet"),
        Route::Direct => String::from("Downloads over a direct connection"),
    }
}

/// The reasons an install stops with when the anonymity network or its
/// exits did not answer, or Anyone did not come up in time
/// (nonos_market_proto::reason, 28, 29, 31, 32 and 33).
pub const ROUTE_REASONS: [u8; 5] = [28, 29, 31, 32, 33];
