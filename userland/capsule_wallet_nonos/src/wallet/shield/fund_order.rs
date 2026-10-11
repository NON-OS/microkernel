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
 * A withdrawal that funds an escrow order (B8, plan/27): the order's exact
 * price, a standard size, to the deposit address the meter computed and
 * checked against the chain. The review shows the pool's fees before the
 * person confirms.
 */

use shield_wire::OP_QUOTE;

use super::actions::{amount, coin};
use crate::wallet::screen::receive_address::checksummed_text;
use crate::wallet::state::State;

pub fn review(state: &mut State, deposit: [u8; 20]) {
    state.shield_ui.withdraw_to = Some(checksummed_text(&deposit));
    let a = amount(&state.shield_ui);
    super::job::start(state, OP_QUOTE, &[coin(state.shield_ui.asset), &a, "1"]);
}
