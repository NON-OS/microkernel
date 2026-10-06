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

//! A whole account refresh in one round trip. Every field the UI shows is put
//! into a single JSON-RPC batch and sent over one TLS connection, so a refresh
//! costs one handshake instead of one per field. A missing field in the reply
//! leaves that value untouched for the caller. The request is carried a step
//! at a time by `step::Job`, and a failed one marks the link degraded so the
//! diagnostic runs again.

use alloc::vec::Vec;

use crate::wallet::nox::constants::{
    SEL_ACTIVE_POSITIONS, SEL_BALANCE_OF, SEL_PENDING_REWARDS, SEL_PROTOCOL_STATS,
    STATS_EMISSION_RATE, STATS_REWARDS_DISTRIBUTED, STATS_TOTAL_STAKED,
};
use crate::wallet::nox::{apr_bps, calldata_addr, q32_to_u128};
use crate::wallet::rpc;

// Request ids, one per field, matched back out of the batch reply.
const ID_ETH_BALANCE: u64 = 2;
const ID_NONCE: u64 = 3;
const ID_FEE: u64 = 4;
const ID_NOX_BALANCE: u64 = 10;
const ID_CLAIMABLE: u64 = 11;
const ID_POSITIONS: u64 = 12;
const ID_STATS: u64 = 13;
const ID_STAKE_INFO: u64 = 14;
const ID_USDC_BALANCE: u64 = 15;
const ID_HEAD: u64 = 16;

pub struct NoxStats {
    pub total: [u8; 32],
    pub rewards: [u8; 32],
    pub apr: Option<u64>,
}

#[derive(Default)]
pub struct Snapshot {
    pub eth_balance: Option<[u8; 32]>,
    pub nonce: Option<u64>,
    pub fee: Option<u64>,
    pub nox_balance: Option<[u8; 32]>,
    pub usdc_balance: Option<[u8; 32]>,
    pub claimable: Option<[u8; 32]>,
    pub positions: Option<u64>,
    /// ZeroState Passes the staking contract counts for this account. Taken
    /// from getStakeInfo rather than the NFT contract, since the boost is
    /// applied from what staking itself believes.
    pub passes: Option<u64>,
    pub stats: Option<NoxStats>,
    /// The newest block the host had when it answered.
    pub head: Option<u64>,
}

/// The single batched request that asks for every displayed field of
/// `addr` on the network picked now.
pub fn snapshot_request(addr: &[u8; 20]) -> Vec<u8> {
    let chain = crate::wallet::chain::current();
    let balance_of = calldata_addr(&SEL_BALANCE_OF, addr);
    let mut parts: Vec<Vec<u8>> = Vec::from([
        rpc::request_balance(addr, ID_ETH_BALANCE),
        rpc::request_nonce(addr, ID_NONCE),
        rpc::request_fee(ID_FEE),
        rpc::request_eth_call(&chain.nox, &balance_of, ID_NOX_BALANCE),
        rpc::request_eth_call(&chain.usdc, &balance_of, ID_USDC_BALANCE),
        rpc::request_block_number(ID_HEAD),
    ]);
    // Staking lives on mainnet only; on Sepolia nothing is asked of it.
    if let Some(staking) = chain.staking {
        let claim = calldata_addr(&SEL_PENDING_REWARDS, addr);
        let positions = calldata_addr(&SEL_ACTIVE_POSITIONS, addr);
        let info = calldata_addr(&crate::wallet::nox::SEL_GET_STAKE_INFO, addr);
        parts.push(rpc::request_eth_call(&staking, &claim, ID_CLAIMABLE));
        parts.push(rpc::request_eth_call(&staking, &positions, ID_POSITIONS));
        parts.push(rpc::request_eth_call(&staking, &SEL_PROTOCOL_STATS, ID_STATS));
        parts.push(rpc::request_eth_call(&staking, &info, ID_STAKE_INFO));
    }
    let refs: Vec<&[u8]> = parts.iter().map(Vec::as_slice).collect();
    rpc::request_batch(&refs)
}

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
