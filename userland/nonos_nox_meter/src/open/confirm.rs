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

//! OPEN's CONFIRM: the channel the chain says was opened, recorded only when
//! its event is the one this meter asked for (plan/21, v1.4.1).

use nonos_marketplace_abi::Keccak;
use nonos_payment_proto::OpenStart;
use nonos_settle_proto::Opened;

use crate::channel::{Channel, NO_SIG};
use crate::error::MeterError;
use crate::voucher::ChannelKey;

/// The new channel, if `ev`'s payer is `meter`, its payee, listingId,
/// amount, expiry and reclaimTo equal the request's, and its channelId is the
/// one its openNonce derives on this chain's `settlement`.
pub fn confirmed(
    k: &impl Keccak,
    s: &OpenStart,
    meter: &[u8; 20],
    settlement: &[u8; 20],
    ev: &Opened,
) -> Result<Channel, MeterError> {
    let same = ev.payer == *meter
        && ev.payee == s.payee
        && ev.listing_id == s.listing_id
        && ev.amount == s.amount
        && ev.expiry == s.expiry
        && ev.reclaim_to == s.reclaim_to;
    let key = ChannelKey {
        chain_id: s.chain_id,
        settlement: *settlement,
        payer: *meter,
        payee: s.payee,
        listing_id: s.listing_id,
        open_nonce: ev.open_nonce,
    };
    if !same || key.id(k) != ev.channel_id {
        return Err(MeterError::Mismatch);
    }
    Ok(Channel {
        id: ev.channel_id,
        payee: s.payee,
        listing_id: s.listing_id,
        chain_id: s.chain_id,
        open_nonce: ev.open_nonce,
        unit: s.unit,
        deposit: s.amount,
        settled: 0,
        cumulative: 0,
        seq: 0,
        expiry: s.expiry,
        usage_root: [0; 32],
        sig: NO_SIG,
        frozen: false,
        carried: 0,
        reclaim_to: s.reclaim_to,
    })
}
