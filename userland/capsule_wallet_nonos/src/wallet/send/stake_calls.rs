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

//! The calldata for the staking calls: each selector, and the 32-byte
//! words that follow it.

use alloc::vec::Vec;

/// `approve(address,uint256)` on the NOX token.
pub(super) const APPROVE: [u8; 4] = [0x09, 0x5e, 0xa7, 0xb3];
/// `stake(uint256)` on the staking proxy.
pub(super) const STAKE: [u8; 4] = [0xa6, 0x94, 0xfc, 0x3a];
/// `stakeLocked(uint256,uint256)`: an amount and one of the contract's terms.
pub(super) const STAKE_LOCKED: [u8; 4] = [0x17, 0xb1, 0x8c, 0x89];
/// `unstakePosition(uint256)`: a whole position, by its index.
pub(super) const UNSTAKE_POSITION: [u8; 4] = [0x4a, 0x23, 0x5e, 0xb6];

pub(super) fn word(d: &mut Vec<u8>, v: u128) {
    d.extend_from_slice(&[0u8; 16]);
    d.extend_from_slice(&v.to_be_bytes());
}

pub(super) fn call(selector: [u8; 4], words: &[u128]) -> Vec<u8> {
    let mut d = Vec::with_capacity(4 + 32 * words.len());
    d.extend_from_slice(&selector);
    for w in words {
        word(&mut d, *w);
    }
    d
}
