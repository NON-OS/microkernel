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

use crate::wallet::nox::calldata_addr;
use crate::wallet::nox::constants::{
    SEL_ACTIVE_POSITIONS, SEL_BALANCE_OF, SEL_PENDING_REWARDS, SEL_PROTOCOL_STATS,
};
use crate::wallet::rpc;

pub use super::snapshot_fields::Snapshot;
pub use super::snapshot_parse::parse_snapshot;

// Request ids, one per field, matched back out of the batch reply.
pub(super) const ID_ETH_BALANCE: u64 = 2;
pub(super) const ID_NONCE: u64 = 3;
pub(super) const ID_FEE: u64 = 4;
pub(super) const ID_NOX_BALANCE: u64 = 10;
pub(super) const ID_CLAIMABLE: u64 = 11;
pub(super) const ID_POSITIONS: u64 = 12;
pub(super) const ID_STATS: u64 = 13;
pub(super) const ID_STAKE_INFO: u64 = 14;
pub(super) const ID_USDC_BALANCE: u64 = 15;
pub(super) const ID_HEAD: u64 = 16;

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
