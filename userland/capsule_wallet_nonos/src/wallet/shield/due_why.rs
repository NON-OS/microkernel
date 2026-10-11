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

//! Why a purchase's refundTo is due (B7, v1.4.56), decided from what the
//! wallet already knows: a refund or dispute sent from it, or an escrow that
//! owes a refund now. Nothing else makes an address due. Pure.

use nonos_payment_proto::{DeliveredMade, OrderStatus};

use super::super::order::escrow_offer::{offer, Offer};
use super::super::order::send_step::{SendStep, DISPUTE};
use super::due_entry::Why;

/// `send` is the dispute or refund under way, by kind and step, if any.
pub fn due_why(
    send: Option<(u8, &SendStep)>,
    status: Option<&OrderStatus>,
    delivered: Option<&DeliveredMade>,
    deadline: u64,
    now_s: u64,
) -> Option<Why> {
    if let Some((kind, SendStep::Sent(_) | SendStep::Done)) = send {
        return Some(if kind == DISPUTE { Why::DisputeSent } else { Why::RefundSent });
    }
    let owed = offer(send.map(|s| s.1), status, delivered, deadline, now_s);
    (owed == Some(Offer::Refund)).then_some(Why::RefundOwed)
}
