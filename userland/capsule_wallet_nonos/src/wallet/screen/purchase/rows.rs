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

//! The terms a purchase shows: the market's, never the store's.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::wallet::etna::groups::shortened;
use crate::wallet::order::buy::DELIVERY_ASKED;
use crate::wallet::order::delivered::delivered_line;
use crate::wallet::order::fees::{exact, Fees};
use crate::wallet::order::intake::{Pending, Step};
use crate::wallet::order::read::Named;
use crate::wallet::order::rows::status_rows;
use crate::wallet::screen::receive_address::checksummed_text;

fn text(b: &[u8]) -> String {
    String::from_utf8_lossy(b).into_owned()
}

pub fn rows(p: &Pending) -> Vec<(&'static str, String)> {
    let release = if p.names.release.is_empty() {
        String::from("its default")
    } else {
        text(&p.names.release)
    };
    let mut out = Vec::from([("listing", text(&p.names.listing)), ("release", release)]);
    let b = match &p.read {
        Ok(Named::Escrow(b)) => b,
        Ok(Named::Voucher(v)) => {
            return [out, super::voucher_rows::rows(v, &p.step, &p.form, p.held)].concat()
        }
        Err(_) => return out,
    };
    let bond: String = b.bond_id.iter().take(6).map(|x| format!("{x:02x}")).collect();
    out.extend([
        ("payee", shortened(&checksummed_text(&b.payee))),
        ("price", format!("{} NOX, once", exact(b.amount))),
        ("bond", format!("0x{bond}…, checked by the market")),
        ("package", format!("{} bytes, checked chunk by chunk", b.content_len)),
        ("delivery", format!("within {} days of the order", DELIVERY_ASKED / 86_400)),
        ("refund", String::from("to a fresh address of this wallet")),
    ]);
    out.push(match &p.fees {
        Fees::Quoted(fee) => {
            ("you pay", format!("{} NOX and the Shield's fee, {fee}", exact(b.amount)))
        }
        Fees::Refused(why) => ("shield", why.clone()),
        _ => ("fees", String::from("asking the Shield")),
    });
    if let Step::Fund(m) = &p.step {
        out.push(("deposit", checksummed_text(&m.deposit)));
        p.watch.status.iter().for_each(|s| out.extend(status_rows(s, exact)));
        p.delivery.made.iter().for_each(|d| out.push(("delivery", delivered_line(d))));
    }
    out
}
