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

use crate::wallet::net::read_field;
use crate::wallet::state::State;

/*
 * No transfer is signed above this gas price. Mainnet runs at a few gwei
 * and its worst spikes have been hundreds; a reading past a thousand is a
 * broken or lying node, and the fee cap is twice the reading, so signing on
 * it would spend real money on nothing.
 */
pub const FEE_CEILING_WEI: u64 = 1_000_000_000_000;

/// Pull a fresh nonce and fee from the chain right before a transaction is
/// built, so signing never uses a stale background value or a zero the probe
/// had not filled yet. Both reads are the bounded per-field RPC calls. Ok only
/// when both are current and the fee is under the ceiling; a caller that gets
/// Err must not sign, and shows the reason. The account nonce takes priority
/// over the send screen's editable value so a resend after a confirmed
/// transaction advances correctly.
pub fn freshen_nonce_and_fee(state: &mut State) -> Result<(), &'static [u8]> {
    if let Some(n) = read_field::nonce(&state.address) {
        state.live_nonce = n;
        state.send_nonce = n;
        state.nonce_ready = true;
    }
    if let Some(f) = read_field::fee() {
        state.fee_wei = f;
        state.fee_ready = true;
    }
    if !(state.nonce_ready && state.fee_ready && state.fee_wei != 0) {
        return Err(b"cannot reach network for nonce and fee, try again");
    }
    if state.fee_wei > FEE_CEILING_WEI {
        return Err(b"network fee above 1000 gwei, not signing");
    }
    Ok(())
}
