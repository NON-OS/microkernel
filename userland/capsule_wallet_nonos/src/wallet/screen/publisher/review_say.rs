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

//! What a publisher transaction's review says at each step: the project
//! line, the rows to sign once reviewed (read back from the calldata, so
//! decode None shows no rows and no Sign), what the send ledger says of a
//! send (intent::said), and the landed line in neutral words: a bond no
//! longer held may have been withdrawn or slashed.

use alloc::format;
use alloc::string::String;

use nonos_nox_config::address;

use crate::wallet::intent::{fate_of, rows, said, IntentStep, Rows, NOT_READ_BACK};
use crate::wallet::nox::NOX_DECIMALS;
use crate::wallet::publish::sending::Sending;
use crate::wallet::publish::Kec;
use crate::wallet::send::exact::exact_text;
use crate::wallet::send::sent::Ledger;

fn nox(v: u128) -> String {
    exact_text(v, NOX_DECIMALS)
}

fn eth(v: u128) -> String {
    exact_text(v, crate::wallet::units::ETH_DECIMALS)
}

/// The rows while `s` is at its review; None when they do not read back.
pub fn page(s: &Sending) -> Option<Rows> {
    let IntentStep::Review { quote } = s.step else { return None };
    let at = |c| address(s.send.chain_id, c).ok();
    rows(s.send.kind, &s.send.calldata, &quote, &Kec, &at, nox, eth)
}

/// the project; once sent, the send ledger's fate for the hash followed.
pub fn lead(s: &Sending, read_back: bool, ledger: &Ledger) -> String {
    let more = match s.queue.len() {
        0 => String::new(),
        n => format!(" {n} more follow, each reviewed and signed in turn."),
    };
    let line = match s.step {
        IntentStep::Quoting => "Asking settle for a quote…",
        IntentStep::Funds(_) => "Reading this account's ETH…",
        IntentStep::Review { .. } if !read_back => NOT_READ_BACK,
        IntentStep::Review { .. } if s.pressed => "Reading the account's nonce, then signing…",
        IntentStep::Review { .. } => "Check every line: it is read from the bytes signed.",
        IntentStep::Broadcasting => "Sending…",
        IntentStep::Unsure { .. } => "It may not have gone; reading the account's nonce first…",
        IntentStep::Sent(h) => return said(&h, fate_of(ledger, &s.step), &more),
        IntentStep::Landed if s.send.kind == 5 => "Landed: the bond is no longer held.",
        IntentStep::Landed => "Landed: the chain now reads what it set.",
        IntentStep::Failed(why) => why,
    };
    format!("{line}{more}")
}
