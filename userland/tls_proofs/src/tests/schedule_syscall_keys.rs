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

//! Traffic keys from a stage secret, derived through the pool's syscalls.

use super::schedule_syscall::label;
use crate::traffic_keys::TrafficKeys;

fn arr<const N: usize>(v: &[u8]) -> [u8; N] {
    let mut out = [0u8; N];
    out[..v.len()].copy_from_slice(v);
    out
}

/// Traffic keys from a stage secret; `stage` is "hs" or "ap".
pub fn keys(base: &[u8; 32], stage: &[u8], th: &[u8; 32], suite: u16) -> TrafficKeys {
    let key_len = if suite == 0x1301 { 16 } else { 32 };
    let c: [u8; 32] = arr(&label(base, &[b"c ", stage, b" traffic"].concat(), th, 32));
    let s: [u8; 32] = arr(&label(base, &[b"s ", stage, b" traffic"].concat(), th, 32));
    TrafficKeys {
        suite,
        handshake_secret: *base,
        client_secret: c,
        server_secret: s,
        client_key: arr(&label(&c, b"key", &[], key_len)),
        client_iv: arr(&label(&c, b"iv", &[], 12)),
        server_key: arr(&label(&s, b"key", &[], key_len)),
        server_iv: arr(&label(&s, b"iv", &[], 12)),
    }
}
