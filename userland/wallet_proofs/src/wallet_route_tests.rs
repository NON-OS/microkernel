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

//! The wallet's route set: Nym, Anyone or none, and never a Direct kind.

use nonos_policy_proto::route::{ANYONE, DIRECT, NYM};

use crate::pick::{pick, private_only, Route, PRIVATE_DOWN};
use crate::wallet::net::private_route::{refused, WAITING};
use crate::wallet::net::wallet_route::WalletRoute;

/* Every kind the wallet can hold, with no wildcard: a Direct case added to
 * WalletRoute stops this file compiling. */
fn kind(route: WalletRoute) -> &'static str {
    match route {
        WalletRoute::Nym(_) => "nym",
        WalletRoute::Anon(_) => "anon",
        WalletRoute::Down(_) => "down",
    }
}

#[test]
fn the_wallet_route_set_holds_no_direct_kind() {
    let direct = WalletRoute::from_link(Route::Direct);
    assert_eq!(direct, WalletRoute::Down(WAITING), "a Direct handed in is no route");
    assert_eq!(kind(direct), "down");
    assert_eq!(direct.link(), None, "and gives route link nothing to open");
    assert_eq!(refused(direct), Some(WAITING));
    for r in [Route::Nym(7), Route::Anon(9)] {
        assert_eq!(WalletRoute::from_link(r).link(), Some(r), "{r:?} passes unchanged");
    }
}

// Route::for_wallet as the wallet holds it, over every default and network.
#[test]
fn for_wallet_holds_nym_or_anyone_and_never_a_direct() {
    let held = |d: Option<u8>, nym, anon| {
        let link = private_only(pick(d, nym, anon), nym, anon);
        assert_ne!(link, Route::Direct, "{d:?} {nym} {anon}: the gate is never the path");
        WalletRoute::from_link(link)
    };
    for default in [Some(DIRECT), Some(NYM), Some(ANYONE), None] {
        for (nym, anon) in [(0, 0), (7, 0), (0, 9), (7, 9)] {
            let r = held(default, nym, anon);
            assert_ne!(r.link(), Some(Route::Direct), "{default:?} {nym} {anon}");
            assert!(r.link().is_some() == refused(r).is_none(), "{r:?}");
        }
    }
    assert_eq!(held(Some(DIRECT), 7, 9), WalletRoute::Nym(7));
    assert_eq!(held(Some(DIRECT), 0, 9), WalletRoute::Anon(9));
    assert_eq!(held(Some(DIRECT), 0, 0), WalletRoute::Down(PRIVATE_DOWN));
}

#[test]
fn the_wallet_route_keeps_route_links_words() {
    for link in [Route::Nym(7), Route::Anon(9), Route::Down(PRIVATE_DOWN)] {
        let held = WalletRoute::from_link(link);
        assert_eq!(held.label(), link.label());
        assert_eq!(held.name(), link.name());
        assert_eq!(held.patience_ms(), link.patience_ms());
    }
}
