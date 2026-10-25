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

//! A HEAD on a kept connection, reconnecting once when it went stale.

use super::super::conn::{connect, Conn};
use super::fetch_head::head_len;

pub fn head_reuse(
    conn: &mut Option<Conn>,
    ip: [u8; 4],
    port: u16,
    host: &[u8],
    path: &[u8],
    extra: &[u8],
) -> Option<usize> {
    for _ in 0..2 {
        if conn.is_none() {
            *conn = connect(ip, port);
        }
        let c = match conn {
            Some(c) => c,
            None => return None,
        };
        if let Some(n) = head_len(&*c, host, path, extra) {
            return Some(n);
        }
        if let Some(c) = conn.take() {
            c.close();
        }
    }
    None
}
