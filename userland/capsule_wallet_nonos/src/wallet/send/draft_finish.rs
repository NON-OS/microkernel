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
//! The draft from a plan, once the nonce, the fee and the gas are in.

use super::*;

#[path = "draft_fund.rs"]
mod draft_fund;

const NO_NONCE: &str = "This account has no nonce left to sign with, so nothing was signed.";

/// The draft from a plan and the gas the node estimated for it, once the
/// nonce and the fee in `state` are fresh. None for the estimate is a node
/// that would not estimate the payment.
pub fn finish(state: &State, plan: Plan, estimate: Option<u64>) -> Result<Draft, &'static str> {
    let Some(estimate) = estimate else {
        return Err("The network would not estimate this payment, so it would likely fail.");
    };
    let gas = super::super::gas::gas_limit(estimate, plan.data.is_empty());
    if let Some(why) = super::super::gas::fee_refusal(Some(state.reads.fee_wei)) {
        return Err(core::str::from_utf8(why).unwrap_or("The network fee could not be read."));
    }
    let (tip, max_fee) = state
        .tx
        .offered_fees
        .ok_or("The network's recent fees could not be read. Review the payment again.")?;
    if max_fee > u128::from(super::super::gas::FEE_CEILING_WEI) {
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
        nonce: state
            .broadcast
            .sent
            .next_nonce(plan.chain_id, &state.address, state.reads.live_nonce)
            .ok_or(NO_NONCE)?,
        tip,
        max_fee,
        chain_id: plan.chain_id,
    };
    draft_fund::fund(state, draft)
}
