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
//! A draft checked against the ETH held, for its amount and its fee.

use super::super::*;

/// The draft once the ETH held is known to cover its amount and its fee.
pub(super) fn fund(state: &State, draft: Draft) -> Result<Draft, &'static str> {
    let eth = held(state, ASSET_ETH)
        .ok_or("The ETH balance has not been read, so the network fee cannot be checked. Try again shortly.")?;
    let mut draft = draft;
    /* Everything held, in ETH: the amount is what is left once this
     * review's fee cap is set aside, so the payment and its fee fit. */
    if state.send.all && draft.purpose == Purpose::Pay && draft.asset == ASSET_ETH {
        let left = eth.saturating_sub(draft.fee_cap_wei());
        if left == 0 {
            return Err("The ETH held does not cover the network fee, so nothing is left to send.");
        }
        draft.amount = left;
        draft.value = left;
    }
    if draft.value.saturating_add(draft.fee_cap_wei()) > eth {
        return Err("Not enough ETH for the amount and the network fee.");
    }
    Ok(draft)
}
