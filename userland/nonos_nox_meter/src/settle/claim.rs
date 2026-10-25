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

//! CLAIM (plan/21, v1.4.17): ShieldedPool.claim(NOX's asset id, meter), the
//! NOX the pool credited the meter when it could not push it. Only the
//! meter can claim what is credited to it, and it claims to itself.

use alloc::vec::Vec;

use nonos_nox_config::CLAIM_GAS;
use nonos_settle_proto::Gas;

use super::channel_tx::tx;
use crate::error::MeterError;
use crate::fees::{covers, fees_ok};
use crate::open::TxRequest;

/// NOX's asset id in the Shield pool (plan/20).
pub const NOX_ASSET: u64 = 1;
pub const CLAIM_SELECTOR: [u8; 4] = [0x88, 0x1a, 0x1c, 0xe0];

pub fn claim_data(asset: u64, to: &[u8; 20]) -> Vec<u8> {
    let mut out = CLAIM_SELECTOR.to_vec();
    out.extend_from_slice(&[0u8; 24]);
    out.extend_from_slice(&asset.to_be_bytes());
    out.extend_from_slice(&[0u8; 12]);
    out.extend_from_slice(to);
    out
}

/// claim(NOX, meter) to `pool`, once something is claimable.
pub fn claim_plan(
    chain_id: u64,
    meter: &[u8; 20],
    pool: &[u8; 20],
    claimable: u128,
    g: &Gas,
) -> Result<TxRequest, MeterError> {
    if claimable == 0 {
        return Err(MeterError::NothingOwed);
    }
    fees_ok(g.max_fee_per_gas, g.max_priority_fee_per_gas)?;
    covers(g.balance, CLAIM_GAS, g.max_fee_per_gas)?;
    Ok(tx(chain_id, pool, g.tx_nonce, CLAIM_GAS, g, &claim_data(NOX_ASSET, meter)))
}
