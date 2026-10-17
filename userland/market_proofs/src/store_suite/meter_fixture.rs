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
//! The market's record for the sold listing and the meter's FUNDED reply
//! over a channel table, worked out by the meter's own predicate.

use nonos_marketplace_abi::{commerce_bytes, Commerce, SettleMode};
use nonos_nox_config::{address, Contract, SEPOLIA};
use nonos_payment_proto::{AccountRow, FUNDED_NO, FUNDED_YES};

use super::held_fixture::{KEY, NOW};
use crate::meter::pay::{funded, Offer};

/// What OP_GET_COMMERCE says of the sold listing: KEY on Sepolia, 3 NOX an
/// install by voucher, signed and bound.
pub fn record(unit_price: u128) -> Offer {
    let (payee, bond_id, unit, binding_sig) = ([0; 20], [9; 32], [3; 32], [0; 65]);
    let (min_charge, settle_mode, proof_reqs) = (unit_price, SettleMode::Voucher, Vec::new());
    let c = Commerce {
        payee,
        bond_id,
        unit,
        unit_price,
        min_charge,
        settle_mode,
        proof_reqs,
        binding_sig,
    };
    let token = address(SEPOLIA, Contract::NoxToken).expect("NOX on Sepolia").to_vec();
    let (listing_id, release_hash, block) = (KEY, [2; 32], commerce_bytes(&c));
    Offer { trusted: true, listing_id, release_hash, chain_id: SEPOLIA, token, block }
}

/// The meter's FUNDED reply body for the sold listing over `rows`.
pub fn meter(rows: &[AccountRow], o: &Offer) -> Vec<u8> {
    let nox = address(SEPOLIA, Contract::NoxToken).expect("NOX on Sepolia");
    let yes = funded(rows.iter().copied(), o, SEPOLIA, &nox, NOW);
    Vec::from([if yes { FUNDED_YES } else { FUNDED_NO }])
}
