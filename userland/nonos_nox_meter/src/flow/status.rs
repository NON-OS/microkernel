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

//! ORDER_STATUS's answer: orderOf's five words
//! (state, funded, extra, fundedAt, deliveredAt), and the NOX waiting for the
//! order, the deposit address's balance plus what the pool credits it. The
//! order leaves the table only when it is Refunded, or Released with no
//! extra, and nothing waits, and ORDER_REFUND has answered for it once:
//! extra above the amount is still the buyer's, and needs refundSalt.

use nonos_payment_proto::OrderStatus;

use super::after::Got;
use super::ask::{Next, Step};
use super::words::{first_u128, u128_of};
use crate::error::MeterError;
use crate::orders::Order;

pub const RELEASED: u8 = 4;
pub const REFUNDED: u8 = 5;

pub(super) fn status(o: &Order, got: &Got) -> Step {
    let of = got.first().filter(|w| w.len() == 5).ok_or(MeterError::Mismatch)?;
    let state = u128_of(&of[0]).and_then(|s| u8::try_from(s).ok()).filter(|s| *s <= REFUNDED);
    let (funded, extra) = (u128_of(&of[1]), u128_of(&of[2]));
    let balance = first_u128(got.get(1));
    let credit = got.get(2).map_or(Some(0), |r| first_u128(Some(r)));
    let (Some(state), Some(funded), Some(extra), Some(balance), Some(credit)) =
        (state, funded, extra, balance, credit)
    else {
        return Err(MeterError::Mismatch);
    };
    let pending = balance.checked_add(credit).ok_or(MeterError::Overflow)?;
    let body = OrderStatus { state, funded, pending, extra, plain: o.plain }.encode();
    let ended = state == REFUNDED || (state == RELEASED && extra == 0);
    Ok(match ended && pending == 0 && o.refund_taken {
        true => Next::Drop(o.order_id, body),
        false => Next::Done(body),
    })
}
