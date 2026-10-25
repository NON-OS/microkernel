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

//! The review tile's rows for each shielded action, as the service read them.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use super::consts::ticker;
use super::review_rows::with;
use super::review_text::short;
use crate::wallet::shield::actions::amount;
use crate::wallet::state::shield_ui::*;

pub fn rows(ui: &ShieldUi) -> Vec<(&'static str, String)> {
    let net = ("Network", String::from(crate::wallet::chain::current().name));
    let coin = ticker(ui.asset);
    if let (SHIELD_DEPOSIT | SHIELD_SETTLE, Some(r)) = (ui.from, &ui.review) {
        let mut out = Vec::from([
            (if r.approval { "Approve" } else { "Send" }, with(&r.amount, coin)),
            ("Pool fee", with(&r.pool_fee, coin)),
            ("Into the shield", with(&r.shielded, coin)),
            ("Network fee, at most", with(&r.max_network_fee, "ETH")),
        ]);
        if !r.valid_for.is_empty() {
            out.push(("Valid for", format!("{} s", r.valid_for)));
        }
        out.push(net);
        return out;
    }
    let q = ui.quote.clone().unwrap_or_default();
    let fees = [
        ("Lander fee", with(&q.network_fee, coin)),
        ("Protocol fee", with(&q.protocol_fee, coin)),
        ("Total fee", with(&q.total_fee, coin)),
    ];
    let mut out = match ui.from {
        SHIELD_SEND => {
            Vec::from([("Pay privately", format!("{} {coin}", amount(ui))), ("To", short(&ui.to))])
        }
        _ => Vec::from([("Withdraw", format!("{} {coin}", amount(ui))), ("To", to_text(ui))]),
    };
    out.extend(fees);
    out.push(("Proof", String::from("built on this machine")));
    out.push(net);
    out
}

fn to_text(ui: &ShieldUi) -> String {
    let to = ui.payee.withdraw_to.as_deref().map(short).unwrap_or_default();
    if ui.payee.to_meter {
        format!("the meter, {to}")
    } else if ui.payee.to_deposit.is_some() {
        format!("the order's deposit address, {to}")
    } else {
        to
    }
}
