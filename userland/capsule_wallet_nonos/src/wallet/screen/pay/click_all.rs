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
//! Use all: everything this account holds, put in the amount field.

use crate::wallet::send;
use crate::wallet::state::State;

/* Everything held, in the field. For ETH the field shows what is held less
 * a fee at the last fee read; the review sets the exact amount aside from
 * the fees it reads itself. Typing in the field undoes it. */
pub(super) fn use_all(state: &mut State) {
    let asset = state.send.token;
    let Some(have) = send::held(state, asset) else {
        state.failure = Some("The balance has not been read yet. Try again shortly.");
        return;
    };
    let shown = if asset == send::ASSET_ETH {
        let rough = u128::from(state.fee_wei).saturating_mul(2);
        have.saturating_sub(rough.saturating_mul(u128::from(send::gas::PLAIN_GAS)))
    } else {
        have
    };
    state.send.amount = crate::wallet::num::Amount::of_units(shown, send::decimals(asset));
    state.send.all = true;
    state.send.focus = crate::wallet::state::SEND_FIELD_AMOUNT;
    state.failure = None;
}
