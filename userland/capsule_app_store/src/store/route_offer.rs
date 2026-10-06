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
use crate::status_wire::{Status, ANYONE, ANYONE_WAIT, BROKEN, CHECKING, DIRECT, NYM, STARTING};

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
        Route::Down(_) => {
            String::from("Downloads over Anyone, which is still starting: Install waits up to 3 minutes for it")
        }
        Route::Nym(_) => String::from("Downloads through the Nym mixnet"),
        Route::Direct => String::from("Downloads over a direct connection"),
    }
}

/// Where a running download stands, as the fetcher answered it.
pub fn progress_line(s: &Status) -> String {
    let (done, total) = (size(s.done), size(s.total));
    let via = match s.route {
        NYM => "through the Nym mixnet",
        ANYONE => "over Anyone",
        DIRECT => "over a direct connection",
        _ => "",
    };
    match s.stage {
        ANYONE_WAIT if s.try_n > 0 => {
            format!("Anyone is building its circuit, step {} of {}", s.try_n, s.tries)
        }
        ANYONE_WAIT => String::from("Anyone is starting; the download waits for it"),
        STARTING => String::from("The download is starting: finding its route and the files' sizes"),
        CHECKING => format!("{total} here; checking its SHA-256 against the signed pin"),
        /*
         * Through Nym, an exit that went silent: net.socks5 rotates to the
         * next, and the fetcher tries again on a new connection.
         */
        BROKEN if s.try_n > 0 && s.route == NYM => format!(
            "Nym exit did not answer; trying the next one ({} of {}), from {done} of {total}",
            s.try_n, s.tries
        ),
        BROKEN if s.try_n > 0 => format!(
            "The connection dropped; trying again ({} of {}), from {done} of {total} {via}",
            s.try_n, s.tries
        ),
        BROKEN => format!("The connection dropped at {done} of {total}; it resumes from there"),
        _ if s.rate == 0 => format!("{done} of {total} {via}, measuring the rate"),
        _ => format!(
            "{done} of {total} {via}, {}/s, {} left",
            size(s.rate),
            duration(s.total.saturating_sub(s.done) / s.rate)
        ),
    }
}

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

/// The reasons an install stops with when the anonymity network or its
/// exits did not answer, or Anyone did not come up in time
/// (nonos_market_proto::reason, 28, 29, 31, 32 and 33).
pub const ROUTE_REASONS: [u8; 5] = [28, 29, 31, 32, 33];
