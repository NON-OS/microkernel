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

//! BROADCAST: chain_id u64 | a signed raw transaction -> its hash. The hash
//! the node names must be keccak256 of the bytes sent; a node that already
//! holds them has them on their way, and their hash is the answer.

use alloc::vec::Vec;

use nonos_eth_rpc::{broadcast_error, parse_hash32, request_broadcast, NodeSaid};
use nonos_settle_proto::{EINVAL, EPROTO, MAX_RAW_TX};

use crate::keccak::keccak;
use crate::settle::Settle;

pub fn broadcast(s: &mut Settle, body: &[u8]) -> Result<Vec<u8>, i32> {
    if body.len() <= 8 || body.len() > MAX_RAW_TX.saturating_add(8) {
        return Err(EINVAL);
    }
    let (chain_id, raw) = body.split_first_chunk::<8>().ok_or(EINVAL)?;
    Ok(send_raw(s, u64::from_le_bytes(*chain_id), raw)?.to_vec())
}

/// Send `raw` and answer its hash, which must be keccak256 of the bytes.
pub(crate) fn send_raw(s: &mut Settle, chain_id: u64, raw: &[u8]) -> Result<[u8; 32], i32> {
    let hash = keccak(raw);
    let resp = s.rpc.call(chain_id, &request_broadcast(raw, 1))?;
    match parse_hash32(&resp) {
        Some(h) if h == hash => Ok(h),
        Some(_) => Err(EPROTO),
        None => match broadcast_error(&resp) {
            NodeSaid::AlreadyKnown => Ok(hash),
            NodeSaid::Refused(_) => Err(EPROTO),
        },
    }
}
