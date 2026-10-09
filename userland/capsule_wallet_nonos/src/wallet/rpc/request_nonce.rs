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

/// The account's nonce counting what the node holds waiting: the next to sign.
pub fn request_nonce(address: &[u8; 20], id: u64) -> Vec<u8> {
    request_nonce_at(address, b"pending", id)
}

/// The account's nonce in the newest block: every nonce under it is used.
pub fn request_nonce_latest(address: &[u8; 20], id: u64) -> Vec<u8> {
    request_nonce_at(address, b"latest", id)
}

fn request_nonce_at(address: &[u8; 20], tag: &[u8], id: u64) -> Vec<u8> {
    let mut out = Vec::with_capacity(144);
    out.extend_from_slice(
        b"{\"jsonrpc\":\"2.0\",\"method\":\"eth_getTransactionCount\",\"params\":[\"",
    );
    super::append_hex20::append_hex20(&mut out, address);
    out.extend_from_slice(b"\",\"");
    out.extend_from_slice(tag);
    out.extend_from_slice(b"\"],\"id\":");
    super::append_dec_u64::append_dec_u64(&mut out, id);
    out.extend_from_slice(b"}");
    out
}
