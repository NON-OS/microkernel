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

/* The words the history uses for each kind of entry and each state. */

use crate::wallet::state::shield_log::{Kind, Stage};

pub fn kind(k: Kind) -> &'static str {
    match k {
        Kind::Deposit => "Deposit",
        Kind::Send => "Sent",
        Kind::Receive => "Received",
        Kind::Withdraw => "Withdrawal",
    }
}

pub fn stage(s: Stage) -> &'static str {
    match s {
        Stage::Proving => "proving",
        Stage::HandedOff => "handed off",
        Stage::Settled => "settled",
        Stage::Failed => "failed",
    }
}
