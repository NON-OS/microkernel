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

//! Escrow's time constants (plan/27, O12 proposed; the owner may change
//! them). The OS compares the contract's MAX_DELIVERY(), DISPUTE_WINDOW()
//! and UNBOND_DELAY() with these at start.

const HOUR: u64 = 3_600;
const DAY: u64 = 24 * HOUR;

/// The latest a deliveryDeadline may be after the order is written.
pub const MAX_DELIVERY: u64 = 7 * DAY;
/// How long after delivery a buyer may dispute.
pub const DISPUTE_WINDOW: u64 = 3 * DAY;
/// The seller's agent delivers only with at least this much time left.
pub const DELIVER_MARGIN: u64 = HOUR;
/// The least deliveryDeadline the buyer writes, after ORDER_NEW: a fresh
/// Shield note ripens in about 6 hours, and the seller still needs its time
/// (v1.4.17).
pub const ORDER_MIN_LEAD: u64 = 24 * HOUR;
/// The least UNBOND_DELAY: every order funded before an unbond request
/// ends inside it.
pub const UNBOND_DELAY_MIN: u64 = MAX_DELIVERY + DISPUTE_WINDOW + DAY;

/// keccak256 of the DepositForwarder clone's creation code on `chain_id`, as
/// depositAddress(orderId) needs it (plan/27). Pinned with a test vector when
/// the owner deploys the escrow; None until then, and ORDER_NEW refuses.
pub const fn forwarder_initcode_hash(_chain_id: u64) -> Option<[u8; 32]> {
    None
}
