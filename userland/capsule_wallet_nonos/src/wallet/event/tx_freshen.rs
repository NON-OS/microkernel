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

use crate::wallet::send::gas::fee_refusal;
use crate::wallet::state::State;

/// Take a fresh nonce and fee read right before a transaction is built, so
/// signing never uses a stale background value or a zero the refresh had
/// not filled yet. A reading that did not come leaves the value as it was,
/// and is refused: a signature is made only on a nonce and a fee read for
/// it. Ok only when both came in this reading and the fee is under the ceiling; a caller
/// that gets Err must not sign, and shows the reason. The account nonce
/// takes priority over the send screen's editable value so a resend after a
/// confirmed transaction advances correctly.
pub fn take_nonce_and_fee(
    state: &mut State,
    nonce: Option<u64>,
    fee: Option<u64>,
) -> Result<(), &'static [u8]> {
    if let Some(n) = nonce {
        state.live_nonce = n;
        state.send_nonce = n;
        state.nonce_ready = true;
    }
    if let Some(f) = fee {
        state.fee_wei = f;
        state.fee_ready = true;
    }
    if nonce.is_none() {
        return Err(b"cannot reach network for nonce and fee, try again");
    }
    match fee_refusal(fee) {
        Some(why) => Err(why),
        None => Ok(()),
    }
}
