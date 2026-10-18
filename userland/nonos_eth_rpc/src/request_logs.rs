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

/// The most blocks one eth_getLogs asks for; public nodes refuse wider.
pub const LOG_PAGE_BLOCKS: u64 = 10_000;

/// [from, to] in inclusive ranges of at most `step` blocks, oldest first;
/// nothing when from > to or step is 0.
pub fn block_pages(from: u64, to: u64, step: u64) -> impl Iterator<Item = (u64, u64)> {
    let mut next = Some(from).filter(|f| *f <= to && step > 0);
    core::iter::from_fn(move || {
        let start = next?;
        let end = start.saturating_add(step - 1).min(to);
        next = end.checked_add(1).filter(|n| *n <= to);
        Some((start, end))
    })
}
