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
//! The store against the meter's own role table and predicate: app.store
//! may ask FUNDED and not ACCOUNT, and the bit the meter works out over its
//! channel table is what sends Enter to install or to the wallet.

use nonos_nox_config::SEPOLIA;
use nonos_payment_proto::{FundedAsk, Names, OP_ACCOUNT, OP_FUNDED};

use super::held_fixture::{row, sold};
use super::meter_fixture::{meter, record};
use crate::meter::caller::{op_role, Role};
use crate::store::channel_held::{answer, reads_channels, route, Route};
use crate::store::price::VOUCHER;

const STORE: u32 = 31;

fn registry(name: &[u8]) -> Option<u32> {
    match name {
        b"app.store" => Some(STORE),
        b"app.nonos_wallet" => Some(10),
        _ => None,
    }
}

/// The ask the store sends: the names it would post, less the release.
fn ask(n: &Names) -> Vec<u8> {
    FundedAsk { chain_id: n.chain_id, listing: n.listing.clone() }.encode().expect("encodes")
}

#[test]
fn the_store_asks_funded_as_app_store_and_installs_on_the_meters_yes() {
    let l = sold(VOUCHER);
    assert!(reads_channels(&l));
    let names = Names { chain_id: SEPOLIA, listing: l.id.clone(), release: Vec::new() };
    let body = ask(&names);
    let funded = op_role(OP_FUNDED, &body).expect("FUNDED has a role");
    assert!(funded.admits(STORE, registry), "the meter admits app.store to FUNDED");
    let account = op_role(OP_ACCOUNT, &[0]).expect("ACCOUNT has a role");
    assert_eq!(account, Role::Wallet);
    assert!(!account.admits(STORE, registry), "and refuses it ACCOUNT's rows");
    assert_eq!(FundedAsk::parse(&body).map(|a| a.listing), Some(l.id.clone()));
    let yes = meter(&[row()], &record(3));
    assert_eq!(yes, [1], "a funded row for the listing is a one");
    assert_eq!(route(&l, answer(Ok(yes))), Route::Install);
    let no = meter(&[], &record(3));
    assert_eq!(no, [0], "no row is a zero");
    assert_eq!(route(&l, answer(Ok(no))), Route::Post);
}
