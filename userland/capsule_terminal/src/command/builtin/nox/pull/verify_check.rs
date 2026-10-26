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

//! `--verify`'s round trip: the sidecar fetched on the same connection, the
//! body hashed by the kernel, and the two judged by `verify::judge`.

use super::conn::Conn;
use super::verify::{judge, sidecar_path};

/// SHA-256 of `data`, from the kernel; none when it refuses.
pub fn hash_sha256(data: &[u8]) -> Option<[u8; 32]> {
    let mut out = [0u8; 32];
    let n = nonos_libc::crypto_hash(1, data.as_ptr(), data.len(), out.as_mut_ptr(), 32);
    (n == 32).then_some(out)
}

/// Fetch `path`'s sidecar and hold `body` to it.
pub fn check(
    conn: &mut Option<Conn>,
    ip: [u8; 4],
    port: u16,
    host: &[u8],
    path: &[u8],
    extra: &[u8],
    body: &[u8],
) -> Result<(), &'static str> {
    let raw = super::fetch::get_reuse(conn, ip, port, host, &sidecar_path(path), extra)?;
    judge(&raw, hash_sha256(body))
}
