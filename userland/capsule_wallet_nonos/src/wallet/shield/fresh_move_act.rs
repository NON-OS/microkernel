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
//! A landed row's one button while its NOX moves into the Shield (v1.4.56).
//! Pure.

use super::fresh_move::Moving;
use super::landed_held::Held;

/// What a landed row's one button does now.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Act {
    /// Review moving the NOX: the first time, after a refusal, or again.
    Move,
    /// Confirm the review: its approval, or its deposits.
    Confirm { approval: bool },
    /// Withdraw the ETH the address lacks from the Shield.
    PayGas,
    /// That withdraw is under way: a press reviews again, paying nothing.
    GasWaiting,
}

impl Act {
    pub fn label(self) -> &'static str {
        match self {
            Act::Move => "Move to the Shield",
            Act::Confirm { approval: true } => "Approve exactly their sum",
            Act::Confirm { approval: false } => "Confirm the deposits",
            Act::PayGas => "Pay the gas from the Shield",
            Act::GasWaiting => "Waiting for the gas withdraw; review again",
        }
    }
}

/// The button for `m` at `now_s`, with the gas held as a dispute's is.
pub fn act(m: Option<&Moving>, now_s: u64) -> Act {
    let wait = super::super::order::send_resume::GAS_WAIT_S;
    let Some((m, r)) = m.and_then(|m| Some((m, m.review.as_ref()?))) else { return Act::Move };
    let paid = m.gas_paid_s.is_some_and(|t| now_s < t.saturating_add(wait));
    match r {
        r if r.short_eth > 0 && paid => Act::GasWaiting,
        r if r.short_eth > 0 => Act::PayGas,
        r if r.refusal.is_some() => Act::Move,
        r => Act::Confirm { approval: r.approval },
    }
}

/// What a landed row moves: its NOX first, else its ETH, as (is ETH, wei).
pub fn moves(held: Held) -> Option<(bool, u128)> {
    match (held.nox, held.eth) {
        (0, 0) => None,
        (0, eth) => Some((true, eth)),
        (nox, _) => Some((false, nox)),
    }
}

/// The button's words, naming ETH when that is what moves.
pub fn label_of(a: Act, eth: bool) -> &'static str {
    match a {
        Act::Move if eth => "Move the ETH to the Shield",
        a => a.label(),
    }
}
