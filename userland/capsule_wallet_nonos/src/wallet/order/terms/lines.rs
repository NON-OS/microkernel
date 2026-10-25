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
 * The escrow consent's words (B8, the the maintainers' consent ruling): the wallet
 * says by its own name that it is asking, and to whom it pays, in full;
 * what is bought by the market's names; the price and the Shield's fee in
 * NOX; the escrow's terms with the order's own deadline; bond and package
 * on one detail line, with what the store called it (words.rs).
 */

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use nonos_marketplace_abi::{address_text, Keccak};
use nonos_payment_proto::Names;

use super::asked::Record;
use super::words::{detail, escrow, text, QUOTING, REFUSED};
use crate::wallet::order::buy::Buy;
use crate::wallet::order::fees_kind::{exact, fee_wei, Fees, FEE_UNREAD};

pub struct Lines {
    pub lead: String,
    /// Buy, when the Shield quoted a fee that reads as NOX.
    pub action: Option<&'static str>,
    pub escrow: String,
    pub rows: Vec<(&'static str, String)>,
    pub detail: String,
}

/// The consent for `b`, as `wallet` (its own name) says it.
pub fn lines(k: &impl Keccak, wallet: &str, n: &Names, r: &Record, b: &Buy, f: &Fees) -> Lines {
    let payee = text(&address_text(k, &b.payee));
    let fee = match f {
        Fees::Quoted(q) => fee_wei(q).ok_or(FEE_UNREAD),
        Fees::Refused(why) if why == FEE_UNREAD => Err(FEE_UNREAD),
        Fees::Refused(_) => Err(REFUSED),
        Fees::Unasked | Fees::Asking => Err(QUOTING),
    };
    let lead = match fee {
        Ok(_) => format!(
            "{wallet} is asking you, not the store: pay {payee} for what follows? These are \
             the market's own terms. Buy only if it is what you meant."
        ),
        Err(why) => String::from(why),
    };
    let mut rows = Vec::from([
        ("buying", format!("{}, release {}", text(&r.title), text(&r.release))),
        ("pay to", payee),
        ("price", format!("{} NOX, once", exact(b.amount))),
    ]);
    rows.push(match (fee, f) {
        (Ok(wei), _) => ("fee", format!("{} NOX, the Shield's, to pay it privately", exact(wei))),
        (_, Fees::Refused(why)) => ("shield", why.clone()),
        (_, Fees::Quoted(_)) => ("shield", String::from(FEE_UNREAD)),
        _ => ("fee", String::from("asking the Shield")),
    });
    let (action, escrow, detail) = (fee.ok().map(|_| "Buy"), escrow(), detail(n, b));
    Lines { lead, action, escrow, rows, detail }
}
