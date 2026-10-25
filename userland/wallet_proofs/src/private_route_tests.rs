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
 * The wallet's network: Nym or Anyone through route link, never Direct.
 * A Direct default reaches the chain over Nym, or Anyone when Nym is down;
 * with both down nothing opens and the wallet says why.
 */

use crate::pick::{pick, private_only, Route};
use crate::wallet::net::private_route::{refused, WAITING};
use crate::wallet::net::wallet_route::WalletRoute;
use nonos_policy_proto::route::DIRECT;

/* The route a wallet's exchange is given, as Route::for_wallet makes it. */
fn for_wallet(nym: u32, anon: u32) -> WalletRoute {
    WalletRoute::from_link(private_only(pick(Some(DIRECT), nym, anon), nym, anon))
}

#[test]
fn a_chosen_direct_route_reaches_the_chain_over_nym_or_anyone() {
    assert_eq!(for_wallet(7, 9), WalletRoute::Nym(7));
    assert_eq!(for_wallet(0, 9), WalletRoute::Anon(9), "Anyone when Nym is down");
    assert_eq!(refused(for_wallet(7, 9)), None);
    assert_eq!(refused(for_wallet(0, 9)), None);
}

#[test]
fn with_both_down_nothing_opens_and_the_wait_is_named() {
    assert!(matches!(for_wallet(0, 0), WalletRoute::Down(_)));
    assert_eq!(refused(for_wallet(0, 0)), Some(WAITING));
    assert!(WAITING.contains("Nym or Anyone"));
    /* Even a Direct handed in by mistake is no route, and opens nothing. */
    assert_eq!(refused(WalletRoute::from_link(Route::Direct)), Some(WAITING));
}
