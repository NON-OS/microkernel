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

//! Reviews to confirm or settle: a deposit, and a payment settled from the public account.

use nox_shield_core::ffi::PublicShield;
use shield_wire::Values;

use super::args::coin;
use super::held::wallet;
use super::{refuse, Answer};

pub(super) fn shield_view(v: &mut Values, s: &PublicShield) {
    v.put("id", &s.id.to_string())
        .put("approval", if s.approval { "1" } else { "0" })
        .put("amount", &s.amount)
        .put("pool_fee", &s.pool_fee)
        .put("shielded", &s.shielded)
        .put("max_network_fee", &s.max_network_fee)
        .put("valid_for", &s.valid_for_seconds.to_string());
    if let Some(why) = &s.refusal {
        v.put("refusal", why);
    }
}

pub fn review_shield(f: &[&str]) -> Answer {
    let [c, amount] = f else { return refuse("Name the coin and the amount.") };
    let review =
        wallet()?.review_shield(coin(c)?, String::from(*amount)).map_err(|e| e.to_string())?;
    let mut v = Values::new();
    shield_view(&mut v, &review);
    Ok(v)
}

/// The spend proved last, or with a field, the one kept under that number.
pub fn review_self_settle(f: &[&str]) -> Answer {
    let w = wallet()?;
    let review = match f {
        [] | [""] => w.review_self_settle(),
        [id] => w.review_self_settle_earlier(String::from(*id)),
        _ => return refuse("Name at most the payment to settle."),
    }
    .map_err(|e| e.to_string())?;
    let mut v = Values::new();
    shield_view(&mut v, &review);
    Ok(v)
}
