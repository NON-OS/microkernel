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
 * What the review tile says for each shielded action. Every figure is the
 * one the service read from the pool for this review; nothing is a default.
 */

use alloc::format;
use alloc::string::String;

pub use super::review_list::rows;
use crate::wallet::state::shield_ui::*;

/* Said before a self-settlement is confirmed, as the apps say it. */
pub const LINKS: &str = "Settling it yourself sends the payment from this wallet's public \
     account. Anyone watching the chain can then link this account to the payment. A lander \
     keeps that link hidden.";

pub fn lead(ui: &ShieldUi) -> &'static str {
    match ui.from {
        SHIELD_DEPOSIT if ui.review.as_ref().is_some_and(|r| r.approval) => {
            "The pool needs your approval to take NOX first. This sends that approval; \
             review the deposit again once it lands."
        }
        SHIELD_DEPOSIT => "Check it before it goes. A deposit is public and cannot be undone.",
        SHIELD_SETTLE => LINKS,
        SHIELD_SEND => {
            "The proof is built on this machine. A lander puts it on chain over a private \
             circuit and cannot see who pays whom or how much."
        }
        _ if ui.payee.to_meter => METER,
        _ if ui.payee.to_deposit.is_some() => DEPOSIT,
        _ => {
            "The proof is built on this machine. The amount and the receiving address \
             become public when it settles."
        }
    }
}

pub(super) fn with(text: &str, coin: &str) -> String {
    if text.is_empty() {
        String::from("not reported")
    } else if text.contains(' ') {
        String::from(text)
    } else {
        format!("{text} {coin}")
    }
}

const DEPOSIT: &str = "The proof is built on this machine. The amount and the order's deposit \
     address become public when it settles; the escrow holds it until the seller delivers.";
const METER: &str = "The proof is built on this machine. The amount and the meter's address \
     become public when it settles; nothing ties them to this wallet's own address.";
