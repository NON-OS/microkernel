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

//! The terms a purchase shows: the market's, never the store's; at
//! consent, terms::lines's.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::wallet::order::delivered::delivered_line;
use crate::wallet::order::fees::{exact, fee_wei, Fees};
use crate::wallet::order::intake::{Pending, Step};
use crate::wallet::order::read::Named;
use crate::wallet::order::rows::status_rows;
use crate::wallet::order::terms::words::{text, DELIVERY_DAYS};
use crate::wallet::screen::receive_address::checksummed_text;

pub fn rows(p: &Pending) -> Vec<(&'static str, String)> {
    let r = p.record.as_ref();
    let mut out = Vec::from_iter(
        r.map(|r| ("buying", format!("{}, release {}", text(&r.title), text(&r.release)))),
    );
    let b = match &p.read {
        Ok(Named::Escrow(b)) => b,
        Ok(Named::Voucher(v)) => {
            return [out, super::voucher_rows::rows(v, &p.step, &p.form, p.held)].concat()
        }
        Err(_) => return Vec::new(),
    };
    if let Some(consent) = super::consent::lines(p) {
        return consent.rows;
    }
    let bond: String = b.bond_id.iter().take(6).map(|x| format!("{x:02x}")).collect();
    out.extend([
        ("pay to", checksummed_text(&b.payee)),
        ("price", format!("{} NOX, once", exact(b.amount))),
        ("bond", format!("0x{bond}…, checked by the market")),
        ("package", format!("{} bytes, checked chunk by chunk", b.content_len)),
        ("delivery", format!("within {DELIVERY_DAYS} days of the order")),
        ("refund", String::from("to a fresh address of this wallet")),
    ]);
    if let Fees::Quoted(fee) = &p.fees {
        let fee = fee_wei(fee).map_or(String::from("not read"), |w| format!("{} NOX", exact(w)));
        out.push(("shield fee", fee));
    }
    if let Step::Fund(m) = &p.step {
        out.push(("deposit", checksummed_text(&m.deposit)));
        p.watch.status.iter().for_each(|s| out.extend(status_rows(s, exact)));
        p.delivery.made.iter().for_each(|d| out.push(("delivery", delivered_line(d))));
        out.extend(super::escrow_rows::rows(p));
    }
    out
}
