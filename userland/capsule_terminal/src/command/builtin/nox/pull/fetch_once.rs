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

//! One GET on its own connection, read until the server closes it.

use alloc::vec::Vec;

use super::super::conn::{connect, Rx};
use super::super::http;
use super::{finish, Fetched, CLOSE_WAIT, EMPTY_BUDGET, MAX_FETCH};

pub(super) fn attempt(
    ip: [u8; 4],
    port: u16,
    host: &[u8],
    path: &[u8],
    extra: &[u8],
) -> Result<Fetched, (bool, &'static str)> {
    let conn = connect(ip, port).ok_or((true, "connect failed"))?;
    if !conn.send(&http::build_get(host, path, extra)) {
        conn.close();
        return Err((true, "send failed"));
    }
    let mut raw = Vec::new();
    let mut empties = 0u32;
    loop {
        match conn.recv(&mut raw) {
            Rx::Data => empties = 0,
            Rx::Empty => {
                if conn.state() >= CLOSE_WAIT {
                    break;
                }
                empties += 1;
                if empties > EMPTY_BUDGET {
                    conn.close();
                    return Err((true, "recv stalled"));
                }
            }
            Rx::Gone => break,
        }
        if raw.len() > MAX_FETCH {
            conn.close();
            return Err((false, "response too large"));
        }
    }
    conn.close();
    finish(&raw).map_err(|e| (false, e))
}
