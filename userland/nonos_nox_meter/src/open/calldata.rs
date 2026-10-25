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

//! The calldata the meter encodes itself for OPEN (plan/26): settle never
//! supplies a `to`, a value or calldata. abi.encode words, selectors pinned
//! by tests against their signatures.

use crate::words::{addr_word, u128_word, u64_word};

/// approve(address,uint256)
pub const APPROVE_SELECTOR: [u8; 4] = [0x09, 0x5e, 0xa7, 0xb3];
/// open(address,bytes32,uint256,uint64,address)
pub const OPEN_SELECTOR: [u8; 4] = [0x63, 0x19, 0x69, 0xa5];

/// NOX approve(spender, amount): the settlement contract, for exactly the
/// channel's deposit, never unlimited.
pub fn approve_data(spender: &[u8; 20], amount: u128) -> [u8; 68] {
    let mut out = [0u8; 68];
    out[..4].copy_from_slice(&APPROVE_SELECTOR);
    out[4..36].copy_from_slice(&addr_word(spender));
    out[36..].copy_from_slice(&u128_word(amount));
    out
}

pub fn open_data(
    payee: &[u8; 20],
    listing_id: &[u8; 32],
    amount: u128,
    expiry: u64,
    reclaim_to: &[u8; 20],
) -> [u8; 164] {
    let mut out = [0u8; 164];
    out[..4].copy_from_slice(&OPEN_SELECTOR);
    out[4..36].copy_from_slice(&addr_word(payee));
    out[36..68].copy_from_slice(listing_id);
    out[68..100].copy_from_slice(&u128_word(amount));
    out[100..132].copy_from_slice(&u64_word(expiry));
    out[132..].copy_from_slice(&addr_word(reclaim_to));
    out
}
