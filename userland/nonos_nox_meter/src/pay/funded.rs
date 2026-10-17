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

//! FUNDED (op 21, the plan): whether one more install of a listing would be
//! charged, as one bit for app.store. The bit is PAY's own decision on the
//! PAY an install makes: `priced` on the record, then `landing` on the
//! meter's state, in the order PAY makes them (capsule_payment's
//! handlers/pay.rs). The caller names a listing and a chain and nothing
//! else; payee, unit and price are the record's, so the bit cannot bisect
//! a deposit.

use nonos_marketplace_abi::commerce_from_bytes;
use nonos_payment_proto::PayRequest;

use super::landing::landing;
use super::offer::priced;
use super::record::Offer;
use crate::state::MeterState;

/// The PAY an install of `o` makes on `chain_id`: one unit of the record's
/// unit at its unitPrice, to its payee, for the release it names. None
/// when the commerce block does not read. Neither check reads the market
/// strings or the call id, so they are left empty.
pub fn install(o: &Offer, chain_id: u64) -> Option<PayRequest<'static>> {
    let c = commerce_from_bytes(&o.block).ok()?;
    Some(PayRequest {
        listing_id: o.listing_id,
        release_hash: o.release_hash,
        payee: c.payee,
        unit: c.unit,
        qty: 1,
        unit_price: c.unit_price,
        call_id: [0; 32],
        chain_id,
        listing: b"",
        release: b"",
    })
}

/// The bit: `o` is the market's record for the listing asked about, `s`
/// the meter's state. Yes exactly when PAY would charge `install`.
pub fn funded(s: &MeterState, o: &Offer, chain_id: u64, nox: &[u8; 20], now: u64) -> bool {
    let Some(r) = install(o, chain_id) else {
        return false;
    };
    priced(&r, o, nox).and_then(|amount| landing(s, &r, amount, now)).is_ok()
}
