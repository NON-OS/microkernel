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

//! What a buyer asks a delivery about: the listing and the policy
//! of the release it bought, its own nonce and the bytes it sent and got,
//! the order and channel, a few days, and the pins it trusts.

use super::expected::PolicyDecode;
use crate::trust::Trust;

pub struct DeliveryExpected<'a> {
    pub listing_id: [u8; 32],
    pub policy: &'a [u8; 256],
    pub nonce: [u8; 32],
    pub request: &'a [u8],
    pub response: &'a [u8],
    pub order_id: [u8; 32],
    pub channel_id: [u8; 32],
    /// The first and last unix day accepted, inclusive.
    pub days: (u64, u64),
    pub pins: &'a Trust<'a>,
    pub read_policy: PolicyDecode,
}

/// What an accepted delivery says: on `window`, the measured supervisor
/// `service_meas`, on its sealed family so far, was handed the buyer's
/// request and gave the buyer's answer. Not that the answer is right.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DeliveryVerdict {
    pub window: u64,
    pub record_id: [u8; 32],
    pub service_meas: [u8; 32],
    /// The device's tag in the delivery scope e2 on `window`.
    pub tag: [u64; 4],
    pub from: u64,
    pub to: u64,
}
