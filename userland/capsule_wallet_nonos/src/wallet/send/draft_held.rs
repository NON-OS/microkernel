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
//! What may still be spent, and a token transfer's calldata.

use super::*;

/// `transfer(address,uint256)`.
const TRANSFER: [u8; 4] = [0xa9, 0x05, 0x9c, 0xbb];

fn low_u128(v: &[u8; 32]) -> Option<u128> {
    if v[..16].iter().any(|b| *b != 0) {
        return None;
    }
    let mut b = [0u8; 16];
    b.copy_from_slice(&v[16..]);
    Some(u128::from_be_bytes(b))
}

/* What may still be spent of `asset`: the balance read, less what the
 * payments still waiting for a block may take, which it does not count. */
pub fn held(state: &State, asset: u8) -> Option<u128> {
    let read = match asset {
        ASSET_ETH if state.balance_ready => low_u128(&state.balance_wei),
        ASSET_NOX if state.nox.balance_ready => low_u128(&state.nox.balance_wei),
        ASSET_USDC if state.usdc_ready => low_u128(&state.usdc_units),
        _ => None,
    }?;
    let (eth, token) = state.broadcast.sent.held(chain::current().id, &state.address, asset);
    Some(read.saturating_sub(if asset == ASSET_ETH { eth } else { token }))
}

pub(super) fn transfer_data(to: &[u8; 20], amount: u128) -> Vec<u8> {
    let mut d = Vec::with_capacity(68);
    d.extend_from_slice(&TRANSFER);
    d.extend_from_slice(&[0u8; 12]);
    d.extend_from_slice(to);
    d.extend_from_slice(&[0u8; 16]);
    d.extend_from_slice(&amount.to_be_bytes());
    d
}
