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

use alloc::vec::Vec;

/// `eth_estimateGas` for a call from `from` to `to` carrying `value` wei and
/// `data`, so a token transfer or a contract call is given the gas the node
/// says it takes rather than a guess.
pub fn request_estimate_gas(
    from: &[u8; 20],
    to: &[u8; 20],
    value: u128,
    data: &[u8],
    id: u64,
) -> Vec<u8> {
    let mut out = Vec::with_capacity(200 + data.len() * 2);
    out.extend_from_slice(b"{\"jsonrpc\":\"2.0\",\"method\":\"eth_estimateGas\",\"params\":[{\"from\":\"");
    super::append_hex20::append_hex20(&mut out, from);
    out.extend_from_slice(b"\",\"to\":\"");
    super::append_hex20::append_hex20(&mut out, to);
    out.extend_from_slice(b"\",\"value\":\"");
    out.extend_from_slice(alloc::format!("{value:#x}").as_bytes());
    out.extend_from_slice(b"\",\"data\":\"");
    super::append_hex_bytes::append_hex_bytes(&mut out, data);
    out.extend_from_slice(b"\"}],\"id\":");
    super::append_dec_u64::append_dec_u64(&mut out, id);
    out.extend_from_slice(b"}");
    out
}
