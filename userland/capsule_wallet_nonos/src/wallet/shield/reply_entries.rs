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

//! The history a state reply carries.

use alloc::string::String;
use alloc::vec::Vec;

use shield_wire::{field, fields};

use crate::wallet::state::shield_log::{Entry, Kind, Stage};

/* The history a state reply carries, oldest first; None when it carries
 * none, as when the store would not give it (`history_why`): the list
 * shown before stays. */
pub fn entries(body: &str) -> Option<Vec<Entry>> {
    /* An entry the wallet cannot read is skipped, never shown as another. */
    (field(body, "history") == Some("1")).then(|| fields(body, "entry").filter_map(entry).collect())
}

fn entry(line: &str) -> Option<Entry> {
    let mut p = line.split('|');
    let kind = match p.next()? {
        "deposit" => Kind::Deposit,
        "sent" => Kind::Send,
        "withdrawn" => Kind::Withdraw,
        "received" => Kind::Received,
        _ => return None,
    };
    /* A coin this wallet does not know is not listed as another. */
    let asset = match p.next()? {
        "ETH" => 0,
        "NOX" => 1,
        _ => return None,
    };
    let amount = String::from(p.next()?);
    let stage = match p.next()? {
        "on its way" => Stage::Sent,
        "settling" => Stage::Settling,
        "done" => Stage::Settled,
        "taken back" => Stage::Failed,
        _ => return None,
    };
    let tx = p.next().filter(|t| !t.is_empty()).map(String::from);
    Some(Entry { kind, asset, amount, stage, tx })
}
