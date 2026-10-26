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
//! The nine RLP fields every EIP-1559 transaction this keyring signs starts
//! from, before its signature: chain id through an empty access list.

use alloc::vec::Vec;

use super::super::rlp::{rlp_list, rlp_string, rlp_uint_be};

/// What one transaction says, by name, so no two of its words can be swapped.
pub struct TxFields<'a> {
    pub chain_id: &'a [u8],
    pub nonce: &'a [u8; 32],
    pub max_priority: &'a [u8; 32],
    pub max_fee: &'a [u8; 32],
    pub gas: &'a [u8; 32],
    pub to: &'a [u8; 20],
    pub value: &'a [u8; 32],
    pub data: &'a [u8],
}

pub fn tx_fields(t: &TxFields<'_>) -> Vec<Vec<u8>> {
    // Sized once for its nine fields, so no push below ever reallocates.
    let mut f = Vec::with_capacity(9);
    let chain_id = rlp_uint_be(t.chain_id);
    f.push(chain_id);
    f.push(rlp_uint_be(t.nonce));
    f.push(rlp_uint_be(t.max_priority));
    f.push(rlp_uint_be(t.max_fee));
    f.push(rlp_uint_be(t.gas));
    f.push(rlp_string(t.to));
    f.push(rlp_uint_be(t.value));
    f.push(rlp_string(t.data));
    f.push(rlp_list(&[]));
    f
}
