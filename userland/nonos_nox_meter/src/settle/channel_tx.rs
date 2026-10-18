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

//! TOPUP (plan/21, v1.4.7): approve then topUp, at the meter's next two
//! nonces, signed with its own key, under the fee caps and only with ETH for
//! both. The calldata is encoded here; settle supplies none.

use nonos_nox_config::{APPROVE_GAS, TOPUP_GAS};
use nonos_payment_proto::TopUp;
use nonos_settle_proto::Gas;

use crate::error::MeterError;
use crate::fees::{covers, fees_ok};
use crate::open::{approve_data, TxRequest};
use crate::settle::calldata::topup_data;
use crate::state::MeterState;

pub(super) fn tx(
    chain_id: u64,
    to: &[u8; 20],
    nonce: u64,
    gas: u64,
    g: &Gas,
    data: &[u8],
) -> TxRequest {
    TxRequest {
        chain_id,
        to: *to,
        nonce,
        max_priority_fee_per_gas: g.max_priority_fee_per_gas,
        max_fee_per_gas: g.max_fee_per_gas,
        gas,
        value: 0,
        data: data.to_vec(),
    }
}

/// approve then topUp. `nox` and `settlement` are the config's on the chain.
pub fn topup_plan(
    s: &MeterState,
    t: &TopUp,
    (nox, settlement): (&[u8; 20], &[u8; 20]),
    g: &Gas,
    now: u64,
) -> Result<[TxRequest; 2], MeterError> {
    let c = s.channels.iter().find(|c| c.id == t.channel_id && c.chain_id == t.chain_id);
    let c = c.ok_or(MeterError::NoChannel)?;
    if t.amount == 0 || c.expired(now) {
        return Err(if t.amount == 0 { MeterError::Mismatch } else { MeterError::Expired });
    }
    fees_ok(g.max_fee_per_gas, g.max_priority_fee_per_gas)?;
    covers(g.balance, APPROVE_GAS + TOPUP_GAS, g.max_fee_per_gas)?;
    let next = g.tx_nonce.checked_add(1).ok_or(MeterError::Overflow)?;
    let approve = approve_data(settlement, t.amount);
    let top = topup_data(&t.channel_id, t.amount);
    Ok([
        tx(t.chain_id, nox, g.tx_nonce, APPROVE_GAS, g, &approve),
        tx(t.chain_id, settlement, next, TOPUP_GAS, g, &top),
    ])
}
