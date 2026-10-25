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

//! The fields of a stake: approving the staking proxy, then staking.

use alloc::vec::Vec;

use super::consts::{CHAIN_ID, NOX_TOKEN, STAKING_PROXY};
use super::stake_data::{stake_approve_calldata, stake_calldata};
use super::tx_fields::{tx_fields, TxFields};

// approve(stakingProxy, amount) on the NOX token.
pub fn nox_stake_approve_fields(
    nonce: &[u8; 32],
    max_priority: &[u8; 32],
    max_fee: &[u8; 32],
    gas: &[u8; 32],
    amount: &[u8; 32],
) -> Vec<Vec<u8>> {
    tx_fields(&TxFields {
        chain_id: &[CHAIN_ID],
        nonce,
        max_priority,
        max_fee,
        gas,
        to: &NOX_TOKEN,
        value: &[0u8; 32],
        data: &stake_approve_calldata(amount),
    })
}

pub fn nox_stake_fields(
    nonce: &[u8; 32],
    max_priority: &[u8; 32],
    max_fee: &[u8; 32],
    gas: &[u8; 32],
    amount: &[u8; 32],
) -> Vec<Vec<u8>> {
    tx_fields(&TxFields {
        chain_id: &[CHAIN_ID],
        nonce,
        max_priority,
        max_fee,
        gas,
        to: &STAKING_PROXY,
        value: &[0u8; 32],
        data: &stake_calldata(amount),
    })
}
