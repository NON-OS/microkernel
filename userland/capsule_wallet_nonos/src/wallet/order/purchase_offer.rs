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

/*
 * ORDER_OFFER's job result (B8): a checked offer goes on to ORDER_NEW under
 * the fresh refundTo and a refundSalt the wallet draws itself, so the meter
 * never chooses the salt that hides where a refund goes (plan/27).
 */

use alloc::boxed::Box;

use nonos_payment_proto::{OfferMade, EAGAIN};

use super::{failed, After, Buy, Stage, UNREADABLE};

pub const NO_SALT: &str = "The wallet could not draw a refund salt; nothing was ordered.";

/// `refund` is the fresh refundTo and the draw of its refundSalt, made only
/// once the offer is in: None from it orders nothing.
pub fn after_offer(
    buy: &Buy,
    r: Result<&[u8], i32>,
    refund: ([u8; 20], impl FnOnce() -> Option<[u8; 32]>),
    now: u64,
) -> After {
    let (refund_to, draw) = refund;
    match r {
        Err(EAGAIN) => After::Wait,
        Err(s) => After::Failed(failed(Stage::Offer, s)),
        Ok(b) => match OfferMade::parse(b) {
            None => After::Failed(UNREADABLE),
            Some(o) => draw().map_or(After::Failed(NO_SALT), |salt| {
                After::Order(Box::new(buy.order_new(&o, (refund_to, salt), now)))
            }),
        },
    }
}
