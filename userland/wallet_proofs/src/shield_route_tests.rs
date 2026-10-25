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

//! Shield's route set (shield_core's net::tor): Nym, Anyone or none, never a Direct kind.

use nonos_policy_proto::route::{ANYONE, DIRECT, NYM};

use crate::pick::{pick, private_only, Route, PRIVATE_DOWN};

#[path = "../../shield_core/src/net/tor/shield_route.rs"]
mod shield_route;
use shield_route::{ShieldRoute, NYM_OR_ANYONE};

/* Every kind Shield can hold, with no wildcard: a Direct case added to
 * ShieldRoute stops this file compiling. */
fn kind(route: ShieldRoute) -> &'static str {
    match route {
        ShieldRoute::Nym(_) => "nym",
        ShieldRoute::Anon(_) => "anon",
        ShieldRoute::Down(_) => "down",
    }
}

#[test]
fn the_shield_route_set_holds_no_direct_kind() {
    let direct = ShieldRoute::from_link(Route::Direct);
    assert_eq!(direct, ShieldRoute::Down(NYM_OR_ANYONE), "a Direct handed in is no route");
    assert_eq!(kind(direct), "down");
    assert_eq!(direct.link(), None, "and gives route link nothing to open");
    for r in [Route::Nym(7), Route::Anon(9)] {
        assert_eq!(ShieldRoute::from_link(r).link(), Some(r), "{r:?} passes unchanged");
    }
    let down = ShieldRoute::from_link(Route::Down(PRIVATE_DOWN));
    assert_eq!((down, down.link()), (ShieldRoute::Down(PRIVATE_DOWN), None));
}

// Route::for_wallet, as Shield takes it, over every default and network.
#[test]
fn for_shield_holds_nym_or_anyone_and_never_a_direct() {
    let held = |d: Option<u8>, nym, anon| {
        let link = private_only(pick(d, nym, anon), nym, anon);
        assert_ne!(link, Route::Direct, "{d:?} {nym} {anon}: the gate is never the path");
        ShieldRoute::from_link(link)
    };
    /* None is a store that could not be asked; 0xEE a value this build does not know. */
    for default in [Some(DIRECT), Some(NYM), Some(ANYONE), Some(0xEE), None] {
        for (nym, anon) in [(0, 0), (7, 0), (0, 9), (7, 9)] {
            let r = held(default, nym, anon);
            assert_ne!(r.link(), Some(Route::Direct), "{default:?} {nym} {anon}");
            assert_eq!(r.link().is_some(), kind(r) != "down", "{r:?}");
        }
    }
    assert_eq!(held(Some(DIRECT), 7, 9), ShieldRoute::Nym(7));
    assert_eq!(held(Some(DIRECT), 0, 9), ShieldRoute::Anon(9));
    assert_eq!(held(Some(DIRECT), 0, 0), ShieldRoute::Down(PRIVATE_DOWN));
    assert_eq!(held(Some(ANYONE), 7, 9), ShieldRoute::Anon(9));
}
