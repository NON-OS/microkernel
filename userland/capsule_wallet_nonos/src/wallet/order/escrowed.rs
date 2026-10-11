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
 * An escrow listing's order terms (plan/27, v1.4.40): the block's price for
 * one "install", and the release's package.
 */

use nonos_market_proto::{Commerce, ReleaseEscrow};
use nonos_marketplace_abi::Commerce as Block;
use nonos_nox_config::UNIT_INSTALL;

use super::buy::Buy;

pub(super) fn escrowed(
    chain_id: u64,
    c: &Commerce,
    block: &Block,
    e: &ReleaseEscrow,
) -> Result<Buy, &'static str> {
    if block.unit != UNIT_INSTALL {
        return Err("An escrow listing is sold by the install, and this one names another unit.");
    }
    if e.chunk_root == [0; 32] || e.content_len == 0 {
        return Err("The release names no escrowed package.");
    }
    Ok(Buy {
        chain_id,
        listing_id: c.listing_id,
        release_id: c.release_id,
        bond_id: block.bond_id,
        payee: block.payee,
        amount: block.unit_price,
        chunk_root: e.chunk_root,
        content_len: e.content_len,
        url: e.package_url.clone(),
    })
}
