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
//! The payment, worked out before anything is signed.

use alloc::vec::Vec;

use super::{ASSET_ETH, ASSET_NOX, ASSET_USDC};
use crate::wallet::act::Purpose;
use crate::wallet::chain;
use crate::wallet::state::State;

/// `transfer(address,uint256)`.
const TRANSFER: [u8; 4] = [0xa9, 0x05, 0x9c, 0xbb];

/// What the review shows and the confirm signs, field for field.
#[derive(Clone)]
pub struct Draft {
    /// A payment, or one of the staking transactions.
    pub purpose: Purpose,
    pub asset: u8,
    pub recipient: [u8; 20],
    pub amount: u128,
    /// The transaction's own target: the recipient for ETH, the token
    /// contract for a token.
    pub to: [u8; 20],
    pub value: u128,
    pub data: Vec<u8>,
    pub gas: u64,
    pub nonce: u64,
    pub tip: u128,
    pub max_fee: u128,
    pub chain_id: u64,
}

impl Draft {
    /// The most the network fee can come to: every unit of gas at the cap.
    pub fn fee_cap_wei(&self) -> u128 {
        (self.gas as u128).saturating_mul(self.max_fee)
    }
}

fn low_u128(v: &[u8; 32]) -> Option<u128> {
    if v[..16].iter().any(|b| *b != 0) {
        return None;
    }
    let mut b = [0u8; 16];
    b.copy_from_slice(&v[16..]);
    Some(u128::from_be_bytes(b))
}

/* What may still be spent of `asset`: the balance read, less what the
 * payments still waiting for a block may take, which it does not count. */
pub fn held(state: &State, asset: u8) -> Option<u128> {
    let read = match asset {
        ASSET_ETH if state.balance_ready => low_u128(&state.balance_wei),
        ASSET_NOX if state.nox.balance_ready => low_u128(&state.nox.balance_wei),
        ASSET_USDC if state.usdc_ready => low_u128(&state.usdc_units),
        _ => None,
    }?;
    let (eth, token) = state.sent.held(chain::current().id, &state.address, asset);
    Some(read.saturating_sub(if asset == ASSET_ETH { eth } else { token }))
}

fn transfer_data(to: &[u8; 20], amount: u128) -> Vec<u8> {
    let mut d = Vec::with_capacity(68);
    d.extend_from_slice(&TRANSFER);
    d.extend_from_slice(&[0u8; 12]);
    d.extend_from_slice(to);
    d.extend_from_slice(&[0u8; 16]);
    d.extend_from_slice(&amount.to_be_bytes());
    d
}

/// The payment typed on the form, checked against what is held, before the
/// network is asked for the nonce, the fee and the gas.
#[derive(Clone)]
pub struct Plan {
    pub purpose: Purpose,
    pub asset: u8,
    pub recipient: [u8; 20],
    pub amount: u128,
    pub to: [u8; 20],
    pub value: u128,
    pub data: Vec<u8>,
    pub chain_id: u64,
}

/// Work out the payment typed on the form, or say in one sentence why it
/// cannot be made. Nothing here reaches the network.
pub fn plan(state: &State) -> Result<Plan, &'static str> {
    if state.wallet_id == 0 || !state.address_ready {
        return Err("Make or restore a wallet first.");
    }
    let recipient = crate::wallet::event::recipient(state)
        .ok_or("The address must be 0x and 40 hex digits.")?;
    if recipient == [0u8; 20] {
        return Err("That is the zero address. Nothing sent there can come back.");
    }
    /* A mixed-case address carries an EIP-55 checksum, and it must hold. */
    let typed = &state.send_to_hex;
    if crate::wallet::event::address_text::mixed_case(typed) {
        let hash = crate::wallet::screen::receive_address::keccak(typed)
            .ok_or("The address's checksum could not be checked. Try again in a moment.")?;
        if !crate::wallet::event::address_text::case_ok(typed, &hash) {
            return Err("The capital letters in this address do not match its checksum, so a \
                 character is likely wrong. Check it against where it came from.");
        }
    }
    let c = chain::current();
    if recipient == c.nox || recipient == c.usdc {
        return Err("That is a token contract. Anything sent to it is lost: send to the \
             person's own address.");
    }
    if c.staking == Some(recipient) {
        return Err("That is the staking contract. Stake from Staking; a payment sent to it \
             is lost.");
    }
    let asset = state.send_token;
    let amount = state.send_amount.scaled(super::decimals(asset));
    if amount == 0 {
        return Err("Type an amount above zero.");
    }
    let have = held(state, asset).ok_or("The balance has not been read yet. Try again shortly.")?;
    if amount > have {
        return Err("That is more than this account holds.");
    }
    let (to, value, data) = match asset {
        ASSET_ETH => (recipient, amount, Vec::new()),
        ASSET_NOX => (c.nox, 0, transfer_data(&recipient, amount)),
        _ => (c.usdc, 0, transfer_data(&recipient, amount)),
    };
    Ok(Plan { purpose: Purpose::Pay, asset, recipient, amount, to, value, data, chain_id: c.id })
}

/// The draft from a plan and the gas the node estimated for it, once the
/// nonce and the fee in `state` are fresh. None for the estimate is a node
/// that would not estimate the payment.
pub fn finish(state: &State, plan: Plan, estimate: Option<u64>) -> Result<Draft, &'static str> {
    let Some(estimate) = estimate else {
        return Err("The network would not estimate this payment, so it would likely fail.");
    };
    let gas = super::gas::gas_limit(estimate, plan.data.is_empty());
    if let Some(why) = super::gas::fee_refusal(Some(state.fee_wei)) {
        return Err(core::str::from_utf8(why).unwrap_or("The network fee could not be read."));
    }
    let (tip, max_fee) = state
        .offered_fees
        .ok_or("The network's recent fees could not be read. Review the payment again.")?;
    if max_fee > u128::from(super::gas::FEE_CEILING_WEI) {
        return Err("The network fee is above 1000 gwei. Nothing is signed at that price.");
    }
    let draft = Draft {
        purpose: plan.purpose,
        asset: plan.asset,
        recipient: plan.recipient,
        amount: plan.amount,
        to: plan.to,
        value: plan.value,
        data: plan.data,
        gas,
        /* Never a nonce this wallet already sent with: a node that has not
         * seen the last payment yet would otherwise let this one replace it. */
        nonce: state.sent.next_nonce(plan.chain_id, &state.address, state.live_nonce),
        tip,
        max_fee,
        chain_id: plan.chain_id,
    };
    let eth = held(state, ASSET_ETH)
        .ok_or("The ETH balance has not been read, so the network fee cannot be checked. Try again shortly.")?;
    let mut draft = draft;
    /* Everything held, in ETH: the amount is what is left once this
     * review's fee cap is set aside, so the payment and its fee fit. */
    if state.send_all && draft.purpose == Purpose::Pay && draft.asset == ASSET_ETH {
        let left = eth.saturating_sub(draft.fee_cap_wei());
        if left == 0 {
            return Err("The ETH held does not cover the network fee, so nothing is left to send.");
        }
        draft.amount = left;
        draft.value = left;
    }
    if draft.value.saturating_add(draft.fee_cap_wei()) > eth {
        return Err("Not enough ETH for the amount and the network fee.");
    }
    Ok(draft)
}
