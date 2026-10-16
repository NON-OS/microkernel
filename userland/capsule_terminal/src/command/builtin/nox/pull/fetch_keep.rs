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

//! One GET on a kept-alive connection, framed by its length so the connection can be used again.

use alloc::vec::Vec;

use super::super::conn::{Rx, Transport};
use super::super::{framing, http};
use super::fetch_body::decode;
use super::{is_redirect, Fetched, CLOSE_WAIT, EMPTY_BUDGET, MAX_FETCH};

pub fn get_on<T: Transport>(
    conn: &T,
    host: &[u8],
    path: &[u8],
    extra: &[u8],
) -> Result<Fetched, (bool, &'static str)> {
    if !conn.send(&http::build_get_ka(host, path, extra)) {
        return Err((true, "send failed"));
    }
    let mut raw = Vec::new();
    let mut empties = 0u32;
    loop {
        if let Some(f) = framing::frame(&raw)? {
            if f.status == 200 {
                return decode(&raw[f.body], f.gzip).map(Fetched::Body);
            }
            return match f.location {
                Some(loc) if is_redirect(f.status) => Ok(Fetched::Redirect(loc)),
                _ => Err((false, "http status not 200")),
            };
        }
        match conn.recv(&mut raw) {
            Rx::Data => empties = 0,
            Rx::Empty => {
                if conn.state() >= CLOSE_WAIT {
                    return Err((true, "truncated body"));
                }
                empties += 1;
                if empties > EMPTY_BUDGET {
                    return Err((true, "recv stalled"));
                }
            }
            Rx::Gone => return Err((true, "connection closed")),
        }
        if raw.len() > MAX_FETCH {
            return Err((false, "response too large"));
        }
    }
}
