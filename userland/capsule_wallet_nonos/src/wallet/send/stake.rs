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

//! A staking transaction, planned like a payment: the approve that lets
//! the staking contract take NOX, the stake (plain or for a term), or the
//! close of a position. Each is reviewed on fresh reads before it is
//! signed, and signed as the reviewed transaction itself, so what the
//! review shows is byte for byte what goes out.

use alloc::vec::Vec;

use super::draft::Plan;
use super::ASSET_NOX;
use crate::wallet::act::Purpose;
use crate::wallet::chain;
use crate::wallet::nox::LOCK_TERMS;
use crate::wallet::state::State;

/// `approve(address,uint256)` on the NOX token.
const APPROVE: [u8; 4] = [0x09, 0x5e, 0xa7, 0xb3];
/// `stake(uint256)` on the staking proxy.
const STAKE: [u8; 4] = [0xa6, 0x94, 0xfc, 0x3a];
/// `stakeLocked(uint256,uint256)`: an amount and one of the contract's terms.
const STAKE_LOCKED: [u8; 4] = [0x17, 0xb1, 0x8c, 0x89];
/// `unstakePosition(uint256)`: a whole position, by its index.
const UNSTAKE_POSITION: [u8; 4] = [0x4a, 0x23, 0x5e, 0xb6];

fn word(d: &mut Vec<u8>, v: u128) {
    d.extend_from_slice(&[0u8; 16]);
    d.extend_from_slice(&v.to_be_bytes());
}

fn call(selector: [u8; 4], words: &[u128]) -> Vec<u8> {
    let mut d = Vec::with_capacity(4 + 32 * words.len());
    d.extend_from_slice(&selector);
    for w in words {
        word(&mut d, *w);
    }
    d
}

/// The transaction the stake screen asks for next, or why there is none.
pub fn plan(state: &State) -> Result<Plan, &'static str> {
    let c = chain::current();
    let staking = c.staking.ok_or("Staking runs on Ethereum mainnet.")?;
    if let Some(why) = crate::wallet::event::stake_refusal(state) {
        return Err(core::str::from_utf8(why).unwrap_or("Staking is not ready."));
    }
    if state.stake_unstake == 1 {
        let data = call(UNSTAKE_POSITION, &[u128::from(state.stake_position)]);
        return Ok(stake_plan(Purpose::Unstake, staking, 0, data, c.id));
    }
    let amount = crate::wallet::event::stake_wei(state);
    if state.stake_step == 0 {
        let mut data = Vec::with_capacity(68);
        data.extend_from_slice(&APPROVE);
        data.extend_from_slice(&[0u8; 12]);
        data.extend_from_slice(&staking);
        word(&mut data, amount);
        return Ok(stake_plan(Purpose::Approve, c.nox, amount, data, c.id));
    }
    let lock = LOCK_TERMS[(state.stake_lock as usize).min(LOCK_TERMS.len() - 1)].0;
    let data = if lock == 0 {
        call(STAKE, &[amount])
    } else {
        call(STAKE_LOCKED, &[amount, u128::from(lock)])
    };
    Ok(stake_plan(Purpose::Stake, staking, amount, data, c.id))
}

fn stake_plan(purpose: Purpose, to: [u8; 20], amount: u128, data: Vec<u8>, chain_id: u64) -> Plan {
    Plan { purpose, asset: ASSET_NOX, recipient: to, amount, to, value: 0, data, chain_id }
}
