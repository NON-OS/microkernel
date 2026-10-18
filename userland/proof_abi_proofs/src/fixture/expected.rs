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

//! The question a verifier of the fixture's listing brings: this listing,
//! its release's policy, the days around the proof, and its pins.

use nonos_proof_abi::trust::Trust;
use nonos_proof_abi::{DeliveryExpected, Expected};

use super::delivery::{CHANNEL, NONCE, ORDER, REQUEST, RESPONSE};
use super::{decode_policy, DAY, LISTING};

pub fn expected<'a>(policy: &'a [u8; 256], pins: &'a Trust<'a>) -> Expected<'a> {
    Expected {
        listing_id: LISTING,
        policy,
        days: (DAY - 1, DAY + 1),
        pins,
        read_policy: decode_policy,
    }
}

/// A buyer of the fixture's listing, with its own nonce and bytes.
pub fn delivery_expected<'a>(policy: &'a [u8; 256], pins: &'a Trust<'a>) -> DeliveryExpected<'a> {
    let (nonce, request, response, order_id, channel_id) =
        (NONCE, REQUEST, RESPONSE, ORDER, CHANNEL);
    let days = (DAY - 1, DAY + 1);
    DeliveryExpected {
        listing_id: LISTING,
        policy,
        nonce,
        request,
        response,
        order_id,
        channel_id,
        days,
        pins,
        read_policy: decode_policy,
    }
}
