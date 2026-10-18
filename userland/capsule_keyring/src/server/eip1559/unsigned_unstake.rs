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

//! The payloads a signature over a locked stake and an unstake covers.

use alloc::vec;
use alloc::vec::Vec;

use super::super::rlp::rlp_list;
use super::fields_unstake::{nox_stake_locked_fields, nox_unstake_fields};

pub fn unsigned_nox_stake_locked_payload(
    nonce: &[u8; 32],
    max_priority: &[u8; 32],
    max_fee: &[u8; 32],
    gas: &[u8; 32],
    amount: &[u8; 32],
    lock: &[u8; 32],
) -> Vec<u8> {
    let f = nox_stake_locked_fields(nonce, max_priority, max_fee, gas, amount, lock);
    let mut out = vec![0x02u8];
    out.extend_from_slice(&rlp_list(&f));
    out
}

pub fn unsigned_nox_unstake_payload(
    nonce: &[u8; 32],
    max_priority: &[u8; 32],
    max_fee: &[u8; 32],
    gas: &[u8; 32],
    index: &[u8; 32],
) -> Vec<u8> {
    let f = nox_unstake_fields(nonce, max_priority, max_fee, gas, index);
    let mut out = vec![0x02u8];
    out.extend_from_slice(&rlp_list(&f));
    out
}
