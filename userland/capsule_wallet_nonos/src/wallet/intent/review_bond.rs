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
 * Review rows for the NOX and Bond kinds: approve, bond, the delivery
 * agent, unbonding, and the Bond's withdrawals.
 */

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use super::call::Publisher;
use super::text::{contract_name, hex, id};

pub type Rows = Vec<(&'static str, String)>;

pub fn bond_rows(p: &Publisher, nox: fn(u128) -> String) -> Option<Rows> {
    use Publisher::*;
    let s = |t: &str| t.to_string();
    Some(match p {
        Approve { spender, amount } => Vec::from([
            ("action", s("let a contract take NOX from this account")),
            ("taker", s(contract_name(*spender))),
            ("amount", format!("{} NOX, exactly this, never unlimited", nox(*amount))),
        ]),
        Bond { publisher_key, amount } => Vec::from([
            ("action", s("bond NOX for a publisher key")),
            ("publisher key", id(publisher_key)),
            ("amount", format!("{} NOX", nox(*amount))),
            ("risk", s("slashed for a lost dispute, or two different attests of one release")),
        ]),
        SetDeliveryAgent { bond_id, agent } => Vec::from([
            ("action", s("name who delivers escrow orders")),
            ("bond", id(bond_id)),
            ("agent", hex(agent)),
        ]),
        RequestUnbond { bond_id } => Vec::from([
            ("action", s("start unbonding")),
            ("bond", id(bond_id)),
            ("after", s("no new orders; the bond can be withdrawn once the unbond delay passes")),
        ]),
        WithdrawBond { bond_id } => {
            Vec::from([("action", s("withdraw the bond")), ("bond", id(bond_id))])
        }
        WithdrawBondCredit { to } => {
            Vec::from([("action", s("withdraw the Bond's credit")), ("to", hex(to))])
        }
        _ => return None,
    })
}
