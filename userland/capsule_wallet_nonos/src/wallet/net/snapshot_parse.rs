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

//! Reading the batch reply to `snapshot_request` back into its fields.

use crate::wallet::nox::constants::{
    STATS_EMISSION_RATE, STATS_REWARDS_DISTRIBUTED, STATS_TOTAL_STAKED,
};
use crate::wallet::nox::{apr_bps, q32_to_u128};
use crate::wallet::rpc;

use super::read_snapshot::{
    ID_CLAIMABLE, ID_ETH_BALANCE, ID_FEE, ID_HEAD, ID_NONCE, ID_NOX_BALANCE, ID_POSITIONS,
    ID_STAKE_INFO, ID_STATS, ID_USDC_BALANCE,
};
use super::snapshot_fields::{NoxStats, Snapshot};

/// The fields the batch reply to `snapshot_request` holds. A field it does
/// not hold is None, and leaves that value untouched for the caller.
pub fn parse_snapshot(resp: &[u8]) -> Snapshot {
    let obj = |id| rpc::object_for_id(resp, id);
    Snapshot {
        eth_balance: obj(ID_ETH_BALANCE).and_then(rpc::parse_quantity32),
        nonce: obj(ID_NONCE).and_then(rpc::parse_u64),
        fee: obj(ID_FEE).and_then(rpc::parse_u64),
        nox_balance: obj(ID_NOX_BALANCE).and_then(rpc::parse_quantity32),
        usdc_balance: obj(ID_USDC_BALANCE).and_then(rpc::parse_quantity32),
        claimable: obj(ID_CLAIMABLE).and_then(rpc::parse_quantity32),
        positions: obj(ID_POSITIONS)
            .and_then(rpc::parse_quantity32)
            .and_then(|w| q32_to_u128(&w))
            .map(|n| n as u64),
        // Word two of getStakeInfo is nftCount.
        passes: obj(ID_STAKE_INFO)
            .and_then(|o| rpc::parse_call_word(o, 2))
            .and_then(|w| q32_to_u128(&w))
            .map(|n| n as u64),
        stats: obj(ID_STATS).and_then(parse_stats),
        head: obj(ID_HEAD).and_then(rpc::parse_u64),
    }
}

fn parse_stats(obj: &[u8]) -> Option<NoxStats> {
    let total = rpc::parse_call_word(obj, STATS_TOTAL_STAKED)?;
    let emission = rpc::parse_call_word(obj, STATS_EMISSION_RATE)?;
    let rewards = rpc::parse_call_word(obj, STATS_REWARDS_DISTRIBUTED).unwrap_or([0; 32]);
    let apr = match (q32_to_u128(&emission), q32_to_u128(&total)) {
        (Some(e), Some(t)) => apr_bps(e, t),
        _ => None,
    };
    Some(NoxStats { total, rewards, apr })
}
