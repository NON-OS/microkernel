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

//! A bond's words as settle's READs return them, and a reading at NOW.

use alloc::vec::Vec;

use nonos_marketplace_abi::BondReading;

pub const NOW: u64 = 1_800_000_000;
pub const MS: u64 = NOW * 1_000;
pub const CHAIN: u64 = 11_155_111;
pub const ID: [u8; 32] = [0xb0; 32];

pub fn one(n: u128) -> Vec<[u8; 32]> {
    let mut w = [0u8; 32];
    w[16..].copy_from_slice(&n.to_be_bytes());
    Vec::from([w])
}

/// bondOf's five words: payee, publisher key, amount, unlockAt 0, slashed 0.
pub fn bond_of(payee: [u8; 20], key: [u8; 32], amount: u128) -> Vec<[u8; 32]> {
    let mut p = [0u8; 32];
    p[12..].copy_from_slice(&payee);
    [Vec::from([p, key]), one(amount), one(0), one(0)].concat()
}

pub fn reading(read_at: u64) -> BondReading {
    BondReading {
        payee: [0x5a; 20],
        publisher_key: [0x11; 32],
        amount: 5_000,
        unlock_at: 0,
        slashed: 0,
        active: true,
        read_at,
    }
}
