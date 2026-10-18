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

//! OPEN's START (plan/21, v1.4.1): the two transactions the meter signs with
//! its own key, approve then open, at the meter's next two nonces. Refused
//! for a zero amount (a zero NOX approve or transfer reverts on V3_1), and,
//! as NoGas (EGAS), for fees outside the config's caps (v1.4.3) or ETH
//! short of gas for both.

use alloc::vec::Vec;

use nonos_nox_config::{APPROVE_GAS, OPEN_GAS};
use nonos_payment_proto::OpenStart;
use nonos_settle_proto::Gas;

use super::calldata::{approve_data, open_data};
use super::tx::TxRequest;
use crate::error::MeterError;
use crate::fees::{covers, fees_ok};

pub struct OpenPlan {
    pub approve: TxRequest,
    pub open: TxRequest,
}

/// `nox` and `settlement` are the config's addresses on the request's chain.
pub fn open_plan(
    s: &OpenStart,
    nox: &[u8; 20],
    settlement: &[u8; 20],
    g: &Gas,
) -> Result<OpenPlan, MeterError> {
    if s.amount == 0 {
        return Err(MeterError::Mismatch);
    }
    fees_ok(g.max_fee_per_gas, g.max_priority_fee_per_gas)?;
    covers(g.balance, APPROVE_GAS + OPEN_GAS, g.max_fee_per_gas)?;
    let next = g.tx_nonce.checked_add(1).ok_or(MeterError::Overflow)?;
    let tx = |to: &[u8; 20], nonce, gas, data: Vec<u8>| TxRequest {
        chain_id: s.chain_id,
        to: *to,
        nonce,
        max_priority_fee_per_gas: g.max_priority_fee_per_gas,
        max_fee_per_gas: g.max_fee_per_gas,
        gas,
        value: 0,
        data,
    };
    let approve = tx(nox, g.tx_nonce, APPROVE_GAS, approve_data(settlement, s.amount).to_vec());
    let call = open_data(&s.payee, &s.listing_id, s.amount, s.expiry, &s.reclaim_to);
    let open = tx(settlement, next, OPEN_GAS, call.to_vec());
    Ok(OpenPlan { approve, open })
}
