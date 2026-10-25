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

// JSON-RPC 2.0 batching: several requests in one POST over one TLS
// connection. Answers come in any order, each found by its request id.

use alloc::vec::Vec;

/// Join complete JSON-RPC request objects into one batch array body.
pub fn request_batch(parts: &[&[u8]]) -> Vec<u8> {
    let cap = parts.iter().map(|p| p.len() + 1).sum::<usize>() + 2;
    let mut out = Vec::with_capacity(cap);
    out.push(b'[');
    for (i, p) in parts.iter().enumerate() {
        if i > 0 {
            out.push(b',');
        }
        out.extend_from_slice(p);
    }
    out.push(b']');
    out
}
