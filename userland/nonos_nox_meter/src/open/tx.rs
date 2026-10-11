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

//! The keyring's OP_SIGN_TX request (op 25): one EIP-1559 transaction it
//! signs with a key the caller owns, on chain 1 or 11155111. LE header, the
//! numbers as 32-byte big-endian words:
//!
//!   pid u32 | key u32 | chain_id u64 | to[20] | nonce[32] | max_priority[32]
//!   | max_fee[32] | gas[32] | value[32] | data_len u32 | data
//!
//! The reply is the raw signed transaction, 0x02 then its RLP.

use alloc::vec::Vec;

pub const OP_SIGN_TX: u16 = 25;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TxRequest {
    pub chain_id: u64,
    pub to: [u8; 20],
    pub nonce: u64,
    pub max_priority_fee_per_gas: u128,
    pub max_fee_per_gas: u128,
    pub gas: u64,
    pub value: u128,
    pub data: Vec<u8>,
}

fn be_word(v: u128) -> [u8; 32] {
    let mut w = [0u8; 32];
    w[16..].copy_from_slice(&v.to_be_bytes());
    w
}

pub fn sign_tx_request(pid: u32, key: u32, t: &TxRequest) -> Vec<u8> {
    let mut out = Vec::with_capacity(200 + t.data.len());
    out.extend_from_slice(&pid.to_le_bytes());
    out.extend_from_slice(&key.to_le_bytes());
    out.extend_from_slice(&t.chain_id.to_le_bytes());
    out.extend_from_slice(&t.to);
    for v in
        [t.nonce as u128, t.max_priority_fee_per_gas, t.max_fee_per_gas, t.gas as u128, t.value]
    {
        out.extend_from_slice(&be_word(v));
    }
    out.extend_from_slice(&(t.data.len() as u32).to_le_bytes());
    out.extend_from_slice(&t.data);
    out
}
