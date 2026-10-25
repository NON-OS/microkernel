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

//! The fields of a locked stake and of closing a position.

use alloc::vec::Vec;

use super::consts::{CHAIN_ID, STAKING_PROXY};
use super::stake_data::{stake_locked_calldata, unstake_position_calldata};
use super::tx_fields::{tx_fields, TxFields};

// stake(amount) on the staking proxy.
pub fn nox_stake_locked_fields(
    nonce: &[u8; 32],
    max_priority: &[u8; 32],
    max_fee: &[u8; 32],
    gas: &[u8; 32],
    amount: &[u8; 32],
    lock: &[u8; 32],
) -> Vec<Vec<u8>> {
    tx_fields(&TxFields {
        chain_id: &[CHAIN_ID],
        nonce,
        max_priority,
        max_fee,
        gas,
        to: &STAKING_PROXY,
        value: &[0u8; 32],
        data: &stake_locked_calldata(amount, lock),
    })
}

pub fn nox_unstake_fields(
    nonce: &[u8; 32],
    max_priority: &[u8; 32],
    max_fee: &[u8; 32],
    gas: &[u8; 32],
    index: &[u8; 32],
) -> Vec<Vec<u8>> {
    tx_fields(&TxFields {
        chain_id: &[CHAIN_ID],
        nonce,
        max_priority,
        max_fee,
        gas,
        to: &STAKING_PROXY,
        value: &[0u8; 32],
        data: &unstake_position_calldata(index),
    })
}
