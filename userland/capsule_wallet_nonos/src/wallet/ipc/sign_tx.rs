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

use alloc::vec::Vec;

use super::call::keyring_call;
use super::constants::{HDR_LEN, OP_SIGN_TX};
use super::push_word::push_word;

/// What the keyring signs: one EIP-1559 transaction on the picked network.
pub struct TxRequest<'a> {
    pub to: [u8; 20],
    pub value: u128,
    pub data: &'a [u8],
    pub nonce: u64,
    pub gas: u64,
    pub max_priority: u128,
    pub max_fee: u128,
}

/// The signed transaction, ready to broadcast, or the keyring's errno.
pub fn sign_tx(
    port: u32,
    owner_pid: u32,
    wallet_id: u32,
    chain_id: u64,
    tx: &TxRequest<'_>,
) -> Result<Vec<u8>, i32> {
    let mut payload = Vec::with_capacity(200 + tx.data.len());
    payload.extend_from_slice(&owner_pid.to_le_bytes());
    payload.extend_from_slice(&wallet_id.to_le_bytes());
    payload.extend_from_slice(&chain_id.to_le_bytes());
    payload.extend_from_slice(&tx.to);
    push_word(&mut payload, tx.nonce as u128);
    push_word(&mut payload, tx.max_priority);
    push_word(&mut payload, tx.max_fee);
    push_word(&mut payload, tx.gas as u128);
    push_word(&mut payload, tx.value);
    payload.extend_from_slice(&(tx.data.len() as u32).to_le_bytes());
    payload.extend_from_slice(tx.data);
    // The signed form is the calldata plus at most a few hundred bytes.
    let rx = keyring_call(port, OP_SIGN_TX, &payload, tx.data.len() + 512)?;
    if rx.len() <= HDR_LEN {
        return Err(-11);
    }
    Ok(rx[HDR_LEN..].to_vec())
}
