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

/* What the review tile says for each shielded action. */

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use super::consts::{fee, ticker};
use super::review_text::{short, standard};
use crate::wallet::state::shield_ui::{ShieldUi, SHIELD_DEPOSIT, SHIELD_SEND};

pub fn lead(ui: &ShieldUi) -> &'static str {
    match ui.from {
        SHIELD_DEPOSIT => "Check it before it goes. A deposit is public and cannot be undone.",
        SHIELD_SEND => {
            "The proof is built on this machine. A relayer submits it over an \
             onion circuit and cannot see who pays whom."
        }
        _ => {
            "The proof is built on this machine. The amount and the receiving address \
             become public when it settles."
        }
    }
}

pub fn rows(ui: &ShieldUi) -> Vec<(&'static str, String)> {
    let net = ("Network", String::from("Sepolia"));
    match ui.from {
        SHIELD_DEPOSIT => Vec::from([
            ("Deposit", standard(ui)),
            ("Into", String::from("the shield pool")),
            ("Fee", String::from("network gas, from this account")),
            ("Proof", String::from("none, a deposit is public")),
            net,
        ]),
        SHIELD_SEND => Vec::from([
            ("Pay privately", format!("{} {}", ui.amount, ticker(ui.asset))),
            ("To", short(&ui.to)),
            ("Relay fee", String::from(fee(ui.asset))),
            ("Change", String::from("stays shielded")),
            net,
        ]),
        _ => Vec::from([
            ("Withdraw", standard(ui)),
            ("To", String::from("a fresh address of this wallet")),
            ("Relay fee", String::from(fee(ui.asset))),
            ("Proof", String::from("built on this machine")),
            net,
        ]),
    }
}
