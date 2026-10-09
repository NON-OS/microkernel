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

//! Which route a destination takes: the longest prefix that covers it, and
//! the first default route only when nothing longer does.

use crate::link::fresh;
use crate::route::{Route, ROUTES};

const GW_A: [u8; 4] = [10, 0, 2, 2];
const GW_B: [u8; 4] = [10, 0, 2, 3];
const GW_C: [u8; 4] = [10, 0, 2, 4];

fn route(network: [u8; 4], prefix: u8, gateway: Option<[u8; 4]>) -> Route {
    Route { network, prefix, gateway }
}

fn taken(dst: [u8; 4]) -> Option<(u8, Option<[u8; 4]>)> {
    ROUTES.lookup(&dst).map(|r| (r.prefix, r.gateway))
}

#[test]
fn the_longest_covering_prefix_wins_in_either_install_order() {
    let _g = fresh();
    let narrow = route([10, 0, 2, 0], 24, None);
    let wide = route([10, 0, 0, 0], 8, Some(GW_B));
    for order in [[narrow, wide], [wide, narrow]] {
        ROUTES.clear();
        let _ = ROUTES.install(route([0; 4], 0, Some(GW_A)));
        for r in order {
            let _ = ROUTES.install(r);
        }
        assert_eq!(taken([10, 0, 2, 7]), Some((24, None)), "on link, not by the /8");
        assert_eq!(taken([10, 9, 9, 9]), Some((8, Some(GW_B))), "the /8, not the default");
        assert_eq!(taken([192, 0, 2, 1]), Some((0, Some(GW_A))), "only the default covers it");
    }
}

#[test]
fn the_first_default_route_is_kept() {
    let _g = fresh();
    ROUTES.clear();
    let _ = ROUTES.install(route([0; 4], 0, Some(GW_A)));
    let _ = ROUTES.install(route([0; 4], 0, Some(GW_C)));
    assert_eq!(taken([192, 0, 2, 1]), Some((0, Some(GW_A))));
}

#[test]
fn with_no_default_an_uncovered_destination_has_no_route() {
    let _g = fresh();
    ROUTES.clear();
    let _ = ROUTES.install(route([10, 0, 2, 0], 24, None));
    assert_eq!(taken([192, 0, 2, 1]), None);
    assert_eq!(taken([10, 0, 2, 9]), Some((24, None)));
}
