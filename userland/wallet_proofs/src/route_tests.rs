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

//! The network the wallet's requests go over, as its screens say it, and
//! the reader for answers through an anonymity network: whole flights,
//! early stops, and every wait bounded by time rather than by a count.

use nonos_policy_proto::route::{ANYONE, DIRECT, NYM};

use crate::pick::{pick, Route, NYM_DOWN};
use crate::wallet::net::route_text::{route_part, route_value};

#[test]
fn the_status_line_names_the_route_the_requests_took() {
    assert_eq!(route_part(None), None, "nothing is claimed before a check");
    assert_eq!(route_part(Some(pick(Some(NYM), 7, 9))), Some("Nym"));
    assert_eq!(route_part(Some(pick(Some(ANYONE), 7, 9))), Some("Anyone"));
    /* Direct reads as Direct, even with the mixnet running: it used to say
     * "Nym" whenever net.nym answered a health check. */
    assert_eq!(route_part(Some(pick(Some(DIRECT), 7, 9))), Some("Direct"));
    assert_eq!(route_part(Some(pick(Some(NYM), 0, 9))), Some("no route"));
}

#[test]
fn the_route_row_says_how_the_rpc_host_is_reached() {
    assert_eq!(route_value(Some(Route::Nym(7))), "public RPC via Nym");
    assert_eq!(route_value(Some(Route::Anon(9))), "public RPC via Anyone");
    assert_eq!(route_value(Some(Route::Direct)), "public RPC, direct");
    assert_eq!(route_value(Some(Route::Down(NYM_DOWN))), "no route");
    assert_eq!(route_value(None), "public RPC");
}

/* The wallet's RPC never leaves Direct: a Direct default goes over Nym,
 * else Anyone, and with neither there is no route at all. */
#[test]
fn the_wallet_reads_the_chain_only_over_an_anonymity_network() {
    use crate::pick::{private_only, PRIVATE_DOWN};
    assert_eq!(private_only(Route::Direct, 7, 9), Route::Nym(7));
    assert_eq!(private_only(Route::Direct, 0, 9), Route::Anon(9));
    assert_eq!(private_only(Route::Direct, 0, 0), Route::Down(PRIVATE_DOWN));
    assert_eq!(private_only(Route::Anon(9), 7, 9), Route::Anon(9));
    assert_eq!(private_only(Route::Nym(7), 0, 9), Route::Nym(7));
    assert_eq!(private_only(Route::Down(NYM_DOWN), 0, 0), Route::Down(NYM_DOWN));
    for default in [Some(DIRECT), Some(NYM), Some(ANYONE), None] {
        for (nym, anon) in [(0, 0), (7, 0), (0, 9), (7, 9)] {
            let r = private_only(pick(default, nym, anon), nym, anon);
            assert_ne!(r, Route::Direct, "{default:?} {nym} {anon}");
        }
    }
}
