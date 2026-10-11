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

//! A voucher listing's terms on the purchase screen: the market's.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use nonos_nox_config::UNITS;

use crate::wallet::etna::groups::shortened;
use crate::wallet::order::fees::exact;
use crate::wallet::order::held::Held;
use crate::wallet::order::intake::Step;
use crate::wallet::order::voucher::Voucher;
use crate::wallet::order::voucher_form::{Budgeted, VoucherForm};
use crate::wallet::screen::receive_address::checksummed_text;

pub fn rows(v: &Voucher, step: &Step, f: &VoucherForm, held: Held) -> Vec<(&'static str, String)> {
    let unit = UNITS.iter().find(|(_, w)| *w == v.unit).map_or("unit", |(n, _)| n);
    let mut out = Vec::from([
        ("payee", shortened(&checksummed_text(&v.payee))),
        ("price", format!("{} NOX per {unit}", exact(v.unit_price))),
        ("least charge", format!("{} NOX", exact(v.min_charge))),
        ("unspent", String::from("back to a fresh address of this wallet")),
    ]);
    if let (Step::Consent, Held::Open(r)) = (step, held) {
        let left = exact(r.deposit.saturating_sub(r.cumulative));
        out.push(("open channel", format!("{}, {left} NOX unspent", short(&r.channel_id))));
    }
    if let Step::Kept(id) = step {
        out.push(("channel", short(id)));
    }
    if let Step::Opened(id) = step {
        out.push(("channel", short(id)));
        out.push(match f.budgeted {
            Budgeted::Set => ("cap", format!("this listing may spend {} NOX", exact(f.cap_wei))),
            Budgeted::Failed(why) => ("cap", String::from(why)),
            _ => ("cap", String::from("being set")),
        });
    }
    out
}

fn short(id: &[u8; 32]) -> String {
    let id: String = id.iter().take(8).map(|b| format!("{b:02x}")).collect();
    format!("0x{id}…")
}
