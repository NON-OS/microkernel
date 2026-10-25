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

//! The staking review's rows, read off the draft Confirm signs.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::wallet::act::Purpose;
use crate::wallet::chain;
use crate::wallet::send::{fee_text, Draft};

fn hex(bytes: &[u8]) -> String {
    let mut s = String::from("0x");
    for b in bytes {
        s.push_str(&format!("{b:02x}"));
    }
    s
}

fn what(d: &Draft) -> String {
    let nox = super::super::nox_amount(d.amount);
    match d.purpose {
        Purpose::Approve => format!("Let staking take {nox} NOX"),
        Purpose::Unstake => String::from("Close a position"),
        _ => format!("Stake {nox} NOX"),
    }
}

pub(super) fn rows(d: &Draft) -> Vec<(&'static str, String)> {
    Vec::from([
        ("Transaction", what(d)),
        ("Contract", crate::wallet::etna::groups::shortened(&hex(&d.to))),
        ("Network", String::from(chain::named(d.chain_id))),
        ("Network fee, at most", fee_text(d.fee_cap_wei())),
        ("Gas limit", format!("{}", d.gas)),
        ("Nonce", format!("{}", d.nonce)),
    ])
}
