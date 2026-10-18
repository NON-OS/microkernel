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

//! A dispute's first steps (v1.4.51): orderOf, then ORDER_REFUND's refundTo.

use nonos_payment_proto::{DeliveredMade, OrderMade};

use super::dispute::dispute;
use super::order_of::order_of;
use super::send_step::SendStep;

pub const DISPUTED: u8 = 3;
pub(super) const NO_ORDER: &str = "settle did not read the escrow's order.";

/// The order, its bad chunk, and refundTo drawn at Buy.
pub struct Disputed<'a> {
    pub made: &'a OrderMade,
    pub delivered: &'a DeliveredMade,
    pub refund_to: [u8; 20],
}

impl Disputed<'_> {
    /// orderOf read: disputed already, or the window checked.
    pub fn after_read(&self, words: Option<&[[u8; 32]]>, now: u64) -> SendStep {
        let Some(of) = words.and_then(order_of) else { return SendStep::Failed(NO_ORDER) };
        if of.0 == DISPUTED {
            return SendStep::Done;
        }
        match dispute(self.made, self.delivered, self.refund_to, self.refund_to, of, now) {
            Ok(_) => SendStep::RefundTo(of),
            Err(why) => SendStep::Failed(why),
        }
    }

    /// ORDER_REFUND's refundTo, which must be the one drawn at Buy.
    pub fn after_refund_to(&self, answered: [u8; 20], of: (u8, u64), now: u64) -> SendStep {
        match dispute(self.made, self.delivered, self.refund_to, answered, of, now) {
            Ok(send) => SendStep::Quoting(send),
            Err(why) => SendStep::Failed(why),
        }
    }
}

/// After the broadcast, orderOf until it reads Disputed.
pub fn landed(words: Option<&[[u8; 32]]>) -> bool {
    words.and_then(order_of).is_some_and(|of| of.0 == DISPUTED)
}
