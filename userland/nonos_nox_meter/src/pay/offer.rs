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

//! Whether a PAY request asks for exactly what the release sells.

use nonos_marketplace_abi::{commerce_from_bytes, SettleMode};

use super::record::Offer;
use crate::error::MeterError;
use nonos_payment_proto::PayRequest;

/// The amount to charge, qty times unitPrice, once every word of the
/// request equals the signed block: the caller names nothing the publisher
/// and the payee did not sign. `nox` is the token on the request's chain.
pub fn priced(r: &PayRequest<'_>, o: &Offer, nox: &[u8; 20]) -> Result<u128, MeterError> {
    if !o.trusted {
        return Err(MeterError::Untrusted);
    }
    let same = o.listing_id == r.listing_id
        && o.release_hash == r.release_hash
        && o.chain_id == r.chain_id
        && o.token.as_slice() == nox;
    let c = commerce_from_bytes(&o.block).map_err(|_| MeterError::Mismatch)?;
    if !same || c.payee != r.payee || c.unit != r.unit || c.unit_price != r.unit_price {
        return Err(MeterError::Mismatch);
    }
    if c.settle_mode != SettleMode::Voucher {
        return Err(MeterError::NotMetered);
    }
    let amount = (r.qty as u128).checked_mul(r.unit_price).ok_or(MeterError::Overflow)?;
    if amount == 0 || amount < c.min_charge {
        return Err(MeterError::BelowMinimum);
    }
    Ok(amount)
}
