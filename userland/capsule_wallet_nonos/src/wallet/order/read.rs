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
 * What the wallet makes of a purchase the store names (B8, v1.4.36-38):
 * an escrow order or a voucher channel. The store gives names only; the
 * terms are the market's: OP_GET_COMMERCE's block, trusted by publisher
 * and payee, and the release's escrow fields. INSTALL_READY must pass
 * every gate but the package, the bond's included.
 */

use nonos_market_proto::{Commerce, Readiness, ReleaseEscrow};
use nonos_marketplace_abi::{commerce_from_bytes, SettleMode};

use super::buy::Buy;
use super::escrowed::escrowed;
use super::voucher::Voucher;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Named {
    Escrow(Buy),
    Voucher(Voucher),
}

/// INSTALL_READY's package gate: the one an escrowed package may fail
/// until it is bought.
const PACKAGE_GATE: usize = 1;

pub fn read_named(
    chain_id: u64,
    c: &Commerce,
    e: &ReleaseEscrow,
    r: &Readiness,
) -> Result<Named, &'static str> {
    if !c.trusted() {
        return Err("Its publisher and payee do not both stand behind this price.");
    }
    if c.chain_id != chain_id {
        return Err("The market sells this release on another chain.");
    }
    let others = r.gates.iter().enumerate().all(|(i, ok)| *ok || i == PACKAGE_GATE);
    if !others || !r.bond.passes() {
        return Err("The market does not pass this release here; the store says why.");
    }
    let block = commerce_from_bytes(&c.block).map_err(|_| "Its commerce block does not read.")?;
    let (listing_id, payee) = (c.listing_id, block.payee);
    match block.settle_mode {
        SettleMode::Free => Err("This release is free: there is nothing to buy."),
        SettleMode::Voucher => {
            let (unit, unit_price, min_charge) = (block.unit, block.unit_price, block.min_charge);
            let v = Voucher { chain_id, payee, listing_id, unit, unit_price, min_charge };
            Ok(Named::Voucher(v))
        }
        SettleMode::Escrow => escrowed(chain_id, c, &block, e).map(Named::Escrow),
    }
}
