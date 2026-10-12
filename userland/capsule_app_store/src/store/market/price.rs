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

//! OP_GET_COMMERCE for a listing's default release, as its price (B9). An
//! empty commerce block is a free listing; a block that does not decode is
//! Malformed, never free.

use nonos_market_proto::{pair_body, parse_commerce, OP_GET_COMMERCE};
use nonos_marketplace_abi::commerce_from_bytes;
use nonos_nox_config::UNITS;

use super::failure::Failure;
use super::wire::exchange;
use crate::store::price::{Price, FREE};

pub fn fetch(port: u32, request_id: u32, listing: &[u8]) -> Result<Price, Failure> {
    let body = exchange(
        port,
        OP_GET_COMMERCE,
        request_id,
        &pair_body(listing, b"").ok_or(Failure::Malformed)?,
    )?;
    let got = parse_commerce(&body).ok_or(Failure::Malformed)?;
    if got.block.is_empty() {
        return Ok(Price { mode: FREE, unit: b"unit", unit_price: 0, trusted: true });
    }
    let c = commerce_from_bytes(&got.block).map_err(|_| Failure::Malformed)?;
    let unit = UNITS.iter().find(|(_, w)| *w == c.unit).map_or(&b"unit"[..], |(n, _)| n.as_bytes());
    let mode = c.settle_mode as u8;
    Ok(Price { mode, unit, unit_price: c.unit_price, trusted: got.trusted() })
}
