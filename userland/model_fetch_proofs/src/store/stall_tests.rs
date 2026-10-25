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
 * An install over Anyone that stopped on the network may offer a direct
 * download, and only `d` takes it: the route the store gives the install,
 * the release word Enter sends, init's reading of it, the fetcher's route
 * and its retries all stay off Direct until the person asks.
 */

use crate::init_direct::split;
use crate::net::pick::{install_route, Route};
use crate::offer::route_for;
use crate::retry::{End, Kind, Next, Retry};
use crate::store::route_offer::{direct_offered, installs, ROUTE_REASONS};

const GB: u64 = 1_000_000_000;
/* What Enter installs with (capsule_app_store store/install.rs `ask`): no release word. */
const ENTER_RELEASE: &str = "";

/* The fetcher's retries through Anyone, to their end: they carry no route. */
fn given_up() -> End {
    let (mut r, mut now) = (Retry::new(0), 0);
    loop {
        match r.after(true, 2, false, false, Kind::Other, now) {
            Next::Wait { ms, .. } => now += ms,
            Next::GiveUp(end) => return end,
        }
    }
}

#[test]
fn a_stalled_anyone_install_with_no_click_opens_nothing_direct() {
    for store_route in [install_route(9150), install_route(0)] {
        for why in ROUTE_REASONS {
            for (left, uninstalled) in [(GB / 2, false), (20 * GB, false), (20 * GB, true)] {
                assert!(direct_offered(store_route, left, uninstalled, Some(why)), "{why}");
                assert_ne!(installs(store_route), Route::Direct, "{why}");
            }
        }
        assert_eq!(installs(store_route), Route::Anon(store_route_port(store_route)));
    }
    /* Enter again, with no `d`: init reads no direct ask, the fetcher keeps Anyone. */
    let (_, asked) = split("linux.qwen-qwen3-8b", ENTER_RELEASE);
    assert!(!asked);
    assert_eq!(route_for(install_route(9150), asked), Route::Anon(9150));
    assert_ne!(route_for(install_route(0), asked), Route::Direct);
    /* Its retries end as no exit answering, which offers Direct and takes nothing. */
    assert_eq!(given_up(), End::NoExit);
    assert!(direct_offered(Route::Anon(9150), GB / 2, false, Some(32)));
}

/* The port `installs` keeps, and 0 for Anyone still starting. */
fn store_route_port(r: Route) -> u32 {
    match r {
        Route::Anon(p) => p,
        _ => 0,
    }
}
