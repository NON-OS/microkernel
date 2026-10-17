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
 * Where a purchase goes on each job result (B8): a checked offer on to
 * ORDER_NEW, a made order on to funding; a refusal is said in words.
 */

use alloc::boxed::Box;

use nonos_marketplace_abi::Keccak;
use nonos_payment_proto::{OrderMade, OrderNew, EAGAIN};

use super::buy::Buy;
use super::confirm::confirms;
use super::deposit_check::{deposit_check, Pinned};
use super::said::{failed, Stage};

/* ORDER_OFFER's result, and the wallet's own refundSalt drawn for it. */
#[path = "purchase_offer.rs"]
pub mod offer;
pub use offer::after_offer;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum After {
    /// The job runs on: ask JOB again on a later tick.
    Wait,
    /// Send ORDER_NEW with these terms.
    Order(Box<OrderNew>),
    /// The order is made: fund its deposit address with the amount.
    Fund(Box<OrderMade>),
    Failed(&'static str),
}

const UNREADABLE: &str = "The meter's answer could not be read; nothing was ordered.";
const NOT_CONFIRMED: &str = "The order's terms are not the ones confirmed; do not fund it.";

/// ORDER_NEW's job result for `sent`: funded only on the terms confirmed, with
/// the id and deposit address the wallet itself makes of them on `at`.
pub fn after_order(
    k: &impl Keccak,
    sent: &OrderNew,
    at: Option<Pinned>,
    r: Result<&[u8], i32>,
) -> After {
    match r {
        Err(EAGAIN) => After::Wait,
        Err(s) => After::Failed(failed(Stage::Order, s)),
        Ok(b) => match OrderMade::parse(b) {
            Some(m) if !confirms(sent, &m.terms) => After::Failed(NOT_CONFIRMED),
            Some(m) => deposit_check(k, sent, &m, at)
                .map_or_else(After::Failed, |()| After::Fund(Box::new(m))),
            None => After::Failed(UNREADABLE),
        },
    }
}
