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
use super::stake_calls::{call, word, APPROVE, STAKE, STAKE_LOCKED, UNSTAKE_POSITION};
use super::ASSET_NOX;
use crate::wallet::act::Purpose;
use crate::wallet::chain;
use crate::wallet::nox::LOCK_TERMS;
use crate::wallet::state::State;

/// The transaction the stake screen asks for next, or why there is none.
pub fn plan(state: &State) -> Result<Plan, &'static str> {
    let c = chain::current();
    let staking = c.staking.ok_or("Staking runs on Ethereum mainnet.")?;
    if let Some(why) = crate::wallet::event::stake_refusal(state) {
        return Err(core::str::from_utf8(why).unwrap_or("Staking is not ready."));
    }
    if state.stake.unstake == 1 {
        let data = call(UNSTAKE_POSITION, &[u128::from(state.stake.position)]);
        return Ok(stake_plan(Purpose::Unstake, staking, 0, data, c.id));
    }
    let amount = crate::wallet::event::stake_wei(state);
    if state.stake.step == 0 {
        let mut data = Vec::with_capacity(68);
        data.extend_from_slice(&APPROVE);
        data.extend_from_slice(&[0u8; 12]);
        data.extend_from_slice(&staking);
        word(&mut data, amount);
        return Ok(stake_plan(Purpose::Approve, c.nox, amount, data, c.id));
    }
    let lock = LOCK_TERMS[(state.stake.lock as usize).min(LOCK_TERMS.len() - 1)].0;
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
