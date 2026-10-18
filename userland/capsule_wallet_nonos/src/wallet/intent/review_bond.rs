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
 * agent, unbonding, and the Bond's withdrawals; ids and addresses whole.
 */

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use nonos_marketplace_abi::Keccak;
use nonos_nox_config::Contract;

use super::call::Publisher;
use super::text::{addr_text, contract_name, hex, placed};

pub type Rows = Vec<(&'static str, String)>;

pub type At<'a> = &'a dyn Fn(Contract) -> Option<[u8; 20]>;

pub fn bond_rows(p: &Publisher, k: &impl Keccak, at: At, nox: fn(u128) -> String) -> Option<Rows> {
    use Publisher::*;
    let s = |t: &str| t.to_string();
    Some(match p {
        Approve { spender, amount } => {
            let (name, n) = (contract_name(*spender), nox(*amount));
            Vec::from([
                ("action", format!("set NOX allowance for {name} to {n} NOX")),
                ("taker", placed(k, *spender, at(*spender))),
                ("amount", format!("{n} NOX, exactly this, never unlimited")),
            ])
        }
        Bond { publisher_key, amount } => Vec::from([
            ("action", s("bond NOX for a publisher key")),
            ("publisher key", hex(publisher_key)),
            ("amount", format!("{} NOX", nox(*amount))),
            ("risk", s("slashed for a lost dispute, or two different attests of one release")),
        ]),
        SetDeliveryAgent { bond_id, agent } => Vec::from([
            ("action", s("name who delivers escrow orders")),
            ("bond", hex(bond_id)),
            ("agent", addr_text(k, agent)),
        ]),
        RequestUnbond { bond_id } => Vec::from([
            ("action", s("start unbonding")),
            ("bond", hex(bond_id)),
            ("after", s("no new orders; the bond can be withdrawn once the unbond delay passes")),
        ]),
        WithdrawBond { bond_id } => {
            Vec::from([("action", s("withdraw the bond")), ("bond", hex(bond_id))])
        }
        WithdrawBondCredit { to } => {
            Vec::from([("action", s("withdraw the Bond's credit")), ("to", addr_text(k, to))])
        }
        _ => return None,
    })
}
