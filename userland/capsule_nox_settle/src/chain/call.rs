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

//! eth_call, in one batch to one host. A call that reverts, or answers
//! anything but hex, is EPROTO: the host answered, and the answer is no.

use alloc::vec::Vec;

use nonos_eth_rpc::{hex_bytes, object_for_id, request_batch, request_eth_call, string_field};
use nonos_settle_proto::EPROTO;

use crate::net::Rpc;

/// The raw returns of `calls`, in order.
pub fn eth_calls(
    rpc: &mut Rpc,
    chain_id: u64,
    calls: &[([u8; 20], Vec<u8>)],
) -> Result<Vec<Vec<u8>>, i32> {
    eth_calls_at(rpc, chain_id, calls, false)
}

/// The same, from a second host when `second`: never the one `eth_calls` asks.
pub fn eth_calls_at(
    rpc: &mut Rpc,
    chain_id: u64,
    calls: &[([u8; 20], Vec<u8>)],
    second: bool,
) -> Result<Vec<Vec<u8>>, i32> {
    let parts: Vec<Vec<u8>> =
        calls.iter().zip(1u64..).map(|((to, data), id)| request_eth_call(to, data, id)).collect();
    let refs: Vec<&[u8]> = parts.iter().map(Vec::as_slice).collect();
    let batch = request_batch(&refs);
    let resp =
        if second { rpc.call_second(chain_id, &batch)? } else { rpc.call(chain_id, &batch)? };
    (1u64..)
        .take(calls.len())
        .map(|id| {
            let obj = object_for_id(&resp, id).ok_or(EPROTO)?;
            hex_bytes(string_field(obj, b"result").ok_or(EPROTO)?).ok_or(EPROTO)
        })
        .collect()
}
