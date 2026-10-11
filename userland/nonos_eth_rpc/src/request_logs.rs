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

//! eth_getLogs over one contract's events of one kind, in a block range
//!: by address and topic0 only, so the node never learns which
//! channel, order, bond or listing the reader is after. Ids are filtered
//! locally, from what comes back.

use alloc::vec::Vec;

use super::append::{append_hex20, append_hex_bytes, append_quantity};

pub fn request_logs(address: &[u8; 20], topic0: &[u8; 32], from: u64, to: u64, id: u64) -> Vec<u8> {
    let mut out = Vec::with_capacity(240);
    out.extend_from_slice(
        b"{\"jsonrpc\":\"2.0\",\"method\":\"eth_getLogs\",\"params\":[{\"address\":\"",
    );
    append_hex20(&mut out, address);
    out.extend_from_slice(b"\",\"topics\":[\"");
    append_hex_bytes(&mut out, topic0);
    out.extend_from_slice(b"\"],\"fromBlock\":\"");
    append_quantity(&mut out, from);
    out.extend_from_slice(b"\",\"toBlock\":\"");
    append_quantity(&mut out, to);
    out.extend_from_slice(b"\"}],\"id\":");
    super::append::append_dec_u64(&mut out, id);
    out.push(b'}');
    out
}
