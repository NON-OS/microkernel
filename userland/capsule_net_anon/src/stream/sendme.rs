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

//! The authenticated SENDME that grants the far end more room.

extern crate alloc;

use alloc::vec::Vec;

/// tor-spec: version 1 carries the digest of the cell that triggered it.
const VERSION_V1: u8 = 1;

const DIGEST_LEN: usize = 20;

/*
 * version[1] data_len[2] digest[20], from sendme_cell_encode.
 *
 * Version 1 rather than 0. A version 0 SENDME carries no digest, so a relay
 * cannot tell whether the sender actually received the cells it is
 * acknowledging, and the network's own default asks for 1.
 */
pub fn body(digest: &[u8; DIGEST_LEN]) -> Vec<u8> {
    let mut out = Vec::with_capacity(3 + DIGEST_LEN);
    out.push(VERSION_V1);
    out.extend_from_slice(&(DIGEST_LEN as u16).to_be_bytes());
    out.extend_from_slice(digest);
    out
}
