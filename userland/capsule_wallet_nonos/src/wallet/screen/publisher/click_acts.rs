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

//! A Bond's or a release's row pressed: its calls built from the entry and
//! what was typed and read (intent's entry builders), then the first of
//! them quoted. Nothing is signed here: the review's Sign does that.

use crate::wallet::arm::rows::{ROW_INTENT, ROW_RELEASE};
use crate::wallet::intent::{agent_of, anchors_of, bond_of, credit_of, publish_of};
use crate::wallet::intent::{unbond_of, withdraw_bond_of, withdraw_of};
use crate::wallet::nox::NOX_DECIMALS;
use crate::wallet::publish::send_start::start;
use crate::wallet::publish::Kec;
use crate::wallet::state::State;

const NOT_READ: &str = "the Bond's minimum and this account's allowance are not read yet";

/// A view word as a u128 amount, or None when it does not fit one.
fn amount(w: &[u8; 32]) -> Option<u128> {
    let (high, low) = w.split_at(16);
    high.iter().all(|b| *b == 0).then(|| u128::from_be_bytes(low.try_into().unwrap_or([0; 16])))
}

pub fn press(state: &mut State, r: u8) {
    let Some(e) = state.publisher.entry.as_ref() else { return };
    let (p, me) = (&state.publisher, &state.acct.address);
    let (v, now) = (&p.views, crate::wallet::clock::now_s());
    let calls = match r {
        ROW_INTENT => {
            let (Some(min), Some(allowance)) = (v.min_bond.as_ref().and_then(amount), v.allowance)
            else {
                state.failure = Some(NOT_READ);
                return;
            };
            bond_of(&Kec, e, me, p.amount.scaled(NOX_DECIMALS), min, &allowance)
        }
        x if x == ROW_INTENT + 1 => agent_of(&Kec, e, me, &p.agent),
        x if x == ROW_INTENT + 2 => unbond_of(&Kec, e, me),
        // Unread, the bond reads as locked and the credit as none.
        x if x == ROW_INTENT + 3 => {
            withdraw_bond_of(&Kec, e, me, v.bond.as_deref().unwrap_or(&[]), now)
        }
        x if x == ROW_INTENT + 4 => credit_of(me, &v.credit.unwrap_or([0; 32])),
        x if x >= ROW_RELEASE => {
            let (i, which) = (usize::from((x - ROW_RELEASE) / 3), (x - ROW_RELEASE) % 3);
            match which {
                0 => publish_of(&Kec, e, me, i, &p.uri),
                1 => withdraw_of(&Kec, e, me, i),
                _ => anchors_of(&Kec, e, me, i),
            }
        }
        _ => return,
    };
    match calls {
        Ok(calls) => start(state, calls),
        Err(w) => state.failure = Some(super::why::why(w)),
    }
}
