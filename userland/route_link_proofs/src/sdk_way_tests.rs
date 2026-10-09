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

//! An app built on the SDK connects over the network the person chose, and
//! looks a name up, sends a datagram or takes outside connections only on
//! Direct. Before, every app connection was a direct socket and every name a
//! lookup in the clear, whatever was chosen.

use nonos_policy_proto::route;

use crate::pick::{pick, Route};
use crate::sdk_net::way::{direct_only, way, Way, DATAGRAMS_OFF_DIRECT, LOOKUP_OFF_DIRECT};

fn every_choice() -> impl Iterator<Item = (Option<u8>, u32, u32)> {
    let defaults = core::iter::once(None).chain((0..=255u8).map(Some));
    defaults.flat_map(|d| {
        [(0u32, 0u32), (7, 0), (0, 9), (7, 9)].into_iter().map(move |(nym, anon)| (d, nym, anon))
    })
}

#[test]
fn an_app_connection_takes_the_chosen_network_and_no_other() {
    for (default, nym, anon) in every_choice() {
        let route = pick(default, nym, anon);
        let got = way(route);
        match default {
            Some(route::DIRECT) => assert_eq!(got, Way::Direct),
            Some(route::ANYONE) if anon != 0 => assert_eq!(got, Way::Anyone(Route::Anon(anon))),
            Some(route::ANYONE) => assert!(matches!(got, Way::Down(_)), "{default:?} {nym} {anon}"),
            _ if nym != 0 => assert_eq!(got, Way::Mixnet, "{default:?} {nym} {anon}"),
            _ => assert!(matches!(got, Way::Down(_)), "{default:?} {nym} {anon}"),
        }
        assert_eq!(
            got == Way::Direct,
            default == Some(route::DIRECT),
            "a direct socket only when Direct is chosen: {default:?} {nym} {anon}"
        );
    }
}

#[test]
fn a_lookup_or_a_datagram_leaves_only_on_direct() {
    for (default, nym, anon) in every_choice() {
        let route = pick(default, nym, anon);
        for off in [LOOKUP_OFF_DIRECT, DATAGRAMS_OFF_DIRECT] {
            let r = direct_only(route, off);
            if default == Some(route::DIRECT) {
                assert_eq!(r, Ok(()));
            } else {
                assert!(r.is_err(), "{default:?} {nym} {anon}");
            }
        }
    }
}

#[test]
fn a_refusal_says_why() {
    assert_eq!(direct_only(Route::Nym(7), LOOKUP_OFF_DIRECT), Err(LOOKUP_OFF_DIRECT));
    assert_eq!(direct_only(Route::Anon(9), DATAGRAMS_OFF_DIRECT), Err(DATAGRAMS_OFF_DIRECT));
    assert_eq!(direct_only(Route::Down("down"), LOOKUP_OFF_DIRECT), Err("down"));
}
