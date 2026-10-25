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

//! The schema 3 gates and the escrow pin, as the market computes them.

use nonos_marketplace_abi::SettleMode;

use crate::abi_v3::fixtures::{free_commerce, sold};
use crate::chain_hosts::Host;
use crate::install_ready::market::{escrow, market_gates};
use crate::install_ready::pin::pinned;

#[test]
fn an_escrow_package_is_pinned_by_its_chunk_tree_not_its_hash() {
    let e = sold().entries[0].clone();
    let mut rel = e.releases[0].clone();
    assert!(escrow(&e));
    rel.package_hash = [0; 32];
    assert!(pinned(&rel, true) && !pinned(&rel, false));
    rel.package_hash = [2; 32];
    rel.ext.as_mut().unwrap().content_len = 0;
    assert!(!pinned(&rel, true) && pinned(&rel, false));
}

#[test]
fn a_free_listing_passes_and_a_sale_waits_for_its_bond_domain() {
    let mut e = sold().entries[0].clone();
    e.listing_id = "nonos.pkg".into();
    let rel = e.releases[0].clone();
    // A sale: no bond contract is deployed, so neither binding nor attest holds.
    let g = market_gates(&Host, &e, &rel, true);
    assert!(!g.commerce && !g.attest && g.policy && !g.all());
    e.commerce = Some(free_commerce());
    assert!(market_gates(&Host, &e, &rel, false).all());
    e.commerce.as_mut().unwrap().settle_mode = SettleMode::Voucher;
    assert!(!market_gates(&Host, &e, &rel, true).all());
}

#[test]
fn a_schema_2_release_keeps_todays_checks() {
    let mut e = sold().entries[0].clone();
    e.commerce = None;
    e.listing_id = "nonos.pkg".into();
    let mut rel = e.releases[0].clone();
    rel.ext = None;
    assert!(market_gates(&Host, &e, &rel, true).all());
}
