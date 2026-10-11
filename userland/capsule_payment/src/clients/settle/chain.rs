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

//! What settle reads from the chain for the meter.

use nonos_nox_meter::state::ChainView;
use nonos_payment_proto::EIO;
use nonos_settle_proto::{
    chain_hash, ChannelState, Gas, CHANNEL_STATE_LEN, GAS_LEN, OP_CHANNEL_STATE, OP_GAS,
};

use super::call::settle_call;

/// The longest wait for an answer that crosses the mixnet: settle may try
/// more than one RPC host, each a TLS connection over Nym or Anyone.
pub(crate) const ROUTED_MS: u64 = 120_000;

/// Best effort, waiting at most `timeout_ms`: a settle that is down, slow,
/// or whose two RPC hosts disagree leaves the meter on its cached state and
/// its local floor, which is what an airgapped image runs on.
pub(crate) fn channel_state(
    chain_id: u64,
    channel_id: &[u8; 32],
    timeout_ms: u64,
) -> Option<ChainView> {
    let body = chain_hash(chain_id, channel_id);
    let b = settle_call(OP_CHANNEL_STATE, &body, CHANNEL_STATE_LEN, timeout_ms).ok()?;
    let s = ChannelState::parse(&b)?;
    Some(ChainView { deposit: s.deposited, settled: s.settled, expiry: s.expiry, closed: s.closed })
}

/// The meter's ETH balance, next nonce and fee suggestions.
pub(crate) fn gas(chain_id: u64, address: &[u8; 20]) -> Result<Gas, i32> {
    let b = settle_call(OP_GAS, &Gas::request(chain_id, address), GAS_LEN, ROUTED_MS)?;
    Gas::parse(&b).ok_or(EIO)
}
