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

//! DRAIN SIGN (plan/21, v1.4.7): settle(vouchers, sigs) built from the
//! meter's own newest signed vouchers of the named channels, to the config
//! Settlement. Settle supplies only the nonce, the gas limit and the fees.
//!
//! The bound: a settle that lied about settled could have the meter sign
//! settle() again and again, each mined one spending gas. So the meter signs
//! another only when a named channel has a newer voucher than the last
//! settle carried, or to replace that last one at its own nonce (a fee bump:
//! only one of the two can be mined). The returned state records what this
//! one carries, and is written before the transaction is signed.

use alloc::vec::Vec;

use nonos_nox_config::SETTLE_GAS_MAX;
use nonos_payment_proto::DrainSign;

use crate::error::MeterError;
use crate::fees::fees_ok;
use crate::open::TxRequest;
use crate::settle::calldata::settle_data;
use crate::state::MeterState;

pub fn settle_plan(
    s: &MeterState,
    payer: &[u8; 20],
    req: &DrainSign,
    settlement: &[u8; 20],
) -> Result<(MeterState, TxRequest), MeterError> {
    fees_ok(req.max_fee, req.max_priority)?;
    if req.gas_limit == 0 || req.gas_limit > SETTLE_GAS_MAX {
        return Err(MeterError::NoGas);
    }
    let (mut next, mut signed, mut newer) = (s.clone(), Vec::new(), false);
    for id in &req.channels {
        let at = s.channels.iter().position(|c| &c.id == id && c.chain_id == req.chain_id);
        let at = at.ok_or(MeterError::NoChannel)?;
        let c = &s.channels[at];
        let found = c.signed(payer).filter(|_| c.cumulative > c.settled);
        signed.push(found.ok_or(MeterError::NothingOwed)?);
        newer |= c.cumulative > c.carried;
        next.channels[at].carried = c.cumulative;
    }
    let replacing = s.last_settle == Some((req.chain_id, req.nonce));
    if !newer && !replacing {
        return Err(MeterError::NothingOwed);
    }
    next.last_settle = Some((req.chain_id, req.nonce));
    let tx = TxRequest {
        chain_id: req.chain_id,
        to: *settlement,
        nonce: req.nonce,
        max_priority_fee_per_gas: req.max_priority,
        max_fee_per_gas: req.max_fee,
        gas: req.gas_limit,
        value: 0,
        data: settle_data(&signed),
    };
    Ok((next, tx))
}
