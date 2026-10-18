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

//! What a private payment or withdrawal would cost.

use shield_wire::Values;

use super::args::{coin, flag};
use super::held::wallet;
use super::{refuse, Answer};

pub fn quote(f: &[&str]) -> Answer {
    let [c, amount, withdraw] = f else { return refuse("Name the coin, amount and kind.") };
    let q = wallet()?
        .quote_spend(coin(c)?, String::from(*amount), flag(withdraw))
        .map_err(|e| e.to_string())?;
    let mut v = Values::new();
    v.put("network_fee", &q.network_fee)
        .put("protocol_fee", &q.protocol_fee)
        .put("total_fee", &q.total_fee)
        .put("base_fee_gwei", &q.base_fee_gwei);
    if let Some(why) = &q.refusal {
        v.put("refusal", why);
    }
    Ok(v)
}
