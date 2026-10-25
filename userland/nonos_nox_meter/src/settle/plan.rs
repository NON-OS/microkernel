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

//! DRAIN SIGN: settle(vouchers, sigs) built from the
//! meter's own newest signed vouchers of the named channels, to the config
//! Settlement. Settle supplies only the nonce, the gas limit and the fees.
//! The bound on another settle() is `bound::allowed`. The returned state
//! records what this one carries, and is written before it is signed.

use alloc::vec::Vec;

use nonos_nox_config::SETTLE_GAS_MAX;
use nonos_payment_proto::DrainSign;

use crate::error::MeterError;
use crate::fees::fees_ok;
use crate::open::TxRequest;
use crate::settle::calldata::settle_data;
use crate::state::MeterState;

use super::bound::allowed;

pub fn settle_plan(
    s: &MeterState,
    payer: &[u8; 20],
    req: &DrainSign,
    settlement: &[u8; 20],
    now: u64,
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
    next.last_retry = allowed(s, req.chain_id, req.nonce, newer, now)?;
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
