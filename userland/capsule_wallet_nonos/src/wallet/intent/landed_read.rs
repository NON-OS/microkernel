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
 * The READ (settle op 7, plan/28) that says a sent publisher call landed:
 * the view of settle_proto's table and its argument words. Approve reads
 * NOX allowance (2) of `from` for the spender; the Bond's calls bondOf
 * (30), deliveryAgentOf (32) and credit (35); Publish and Withdraw the
 * registry's releaseOf (40); Anchor anchoredAt (50). landed.rs reads the
 * answer.
 */

use alloc::vec::Vec;

use nonos_marketplace_abi::{bond_id as derived_bond, Keccak};
use nonos_nox_config::LISTING_TAG;

use super::abi::{addr, uint};
use super::call::Publisher;
use super::review_bond::At;

/// listingId as the registry recomputes it from publish's words (plan/24).
pub fn listing_of(k: &impl Keccak, publisher_key: &[u8; 32], app_id_hash: &[u8; 32]) -> [u8; 32] {
    k.keccak256(&[LISTING_TAG, *publisher_key, *app_id_hash].concat())
}

/// None only for an Approve whose spender is not deployed.
pub fn landed_read(
    k: &impl Keccak,
    p: &Publisher,
    from: &[u8; 20],
    at: At,
) -> Option<(u16, Vec<[u8; 32]>)> {
    use Publisher::*;
    Some(match p {
        Approve { spender, .. } => (2, Vec::from([addr(from), addr(&at(*spender)?)])),
        Bond { publisher_key, .. } => (30, Vec::from([derived_bond(k, from, publisher_key)])),
        SetDeliveryAgent { bond_id, .. } => (32, Vec::from([*bond_id])),
        RequestUnbond { bond_id } | WithdrawBond { bond_id } => (30, Vec::from([*bond_id])),
        Publish { publisher_key, app_id_hash, release_id, .. } => {
            (40, Vec::from([listing_of(k, publisher_key, app_id_hash), *release_id]))
        }
        WithdrawRelease { listing_id, release_id } => (40, Vec::from([*listing_id, *release_id])),
        Anchor { anchor, kind, listing_id } => {
            (50, Vec::from([*anchor, uint(u128::from(*kind)), *listing_id]))
        }
        WithdrawBondCredit { to } => (35, Vec::from([addr(to)])),
    })
}
