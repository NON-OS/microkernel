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

/* A withdrawal's review: to a fresh account of this wallet, the meter (B7)
 * or an escrow order's deposit (B8), the pool's fees quoted first. */

use alloc::string::String;

use shield_wire::*;

use super::client::call;
use crate::wallet::state::State;

use super::actions::{amount, coin};

const NO_FRESH: &str = "A withdrawal goes only to a fresh account of this wallet, never one \
     used before, and a wallet from a private key has no other account. Restore from the \
     recovery words to withdraw.";

pub fn review_withdraw(state: &mut State) {
    if state.shield_ui.to_meter {
        return review_to_meter(state);
    }
    if let Some(deposit) = state.shield_ui.to_deposit {
        return super::fund_order::review(state, deposit);
    }
    let fresh = match call(OP_FRESH_ADDRESS, &[]) {
        Ok(a) => a,
        Err(why) => {
            state.shield_ui.failure = Some(String::from(why));
            return;
        }
    };
    let Some(to) = field(&fresh.body, "address").filter(|_| fresh.status == STATUS_OK) else {
        let why = if fresh.status == STATUS_OK {
            String::from(NO_FRESH)
        } else {
            String::from(field(&fresh.body, "why").unwrap_or(super::client::NO_ANSWER))
        };
        state.shield_ui.failure = Some(why);
        return;
    };
    state.shield_ui.withdraw_to = Some(String::from(to));
    let a = amount(&state.shield_ui);
    super::job::start(state, OP_QUOTE, &[coin(state.shield_ui.asset), &a, "1"]);
}

/* A withdrawal to the meter (B7): its address from the meter itself. */
fn review_to_meter(state: &mut State) {
    use crate::wallet::screen::shield::consts::{CHAIN_ID, POOL};
    let chain = CHAIN_ID.parse().unwrap_or(0);
    let account = crate::wallet::meter::call::account();
    match super::fund_meter::meter_destination(POOL, chain, account.as_ref()) {
        Ok(a) => {
            let to = crate::wallet::screen::receive_address::checksummed_text(&a);
            state.shield_ui.withdraw_to = Some(to);
            let amt = amount(&state.shield_ui);
            super::job::start(state, OP_QUOTE, &[coin(state.shield_ui.asset), &amt, "1"]);
        }
        Err(why) => state.shield_ui.failure = Some(String::from(why)),
    }
}
