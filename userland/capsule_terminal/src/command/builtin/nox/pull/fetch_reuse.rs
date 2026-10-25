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

//! A GET on a kept connection, reconnecting when it went stale and following redirects.

use alloc::vec::Vec;

use super::super::conn::{connect, Conn};
use super::{get, get_on, Fetched, MAX_HOPS};

fn fetch_body(
    conn: &mut Option<Conn>,
    ip: [u8; 4],
    port: u16,
    host: &[u8],
    path: &[u8],
    extra: &[u8],
) -> Result<Fetched, &'static str> {
    for _ in 0..2 {
        if conn.is_none() {
            *conn = connect(ip, port);
        }
        let c = match conn {
            Some(c) => c,
            None => break,
        };
        match get_on(&*c, host, path, extra) {
            Ok(f) => return Ok(f),
            Err((false, e)) => return Err(e),
            Err((true, _)) => {
                if let Some(c) = conn.take() {
                    c.close();
                }
            }
        }
    }
    get(ip, port, host, path, extra)
}

pub fn get_reuse(
    conn: &mut Option<Conn>,
    ip: [u8; 4],
    port: u16,
    host: &[u8],
    path: &[u8],
    extra: &[u8],
) -> Result<Vec<u8>, &'static str> {
    let mut cur = path.to_vec();
    for _ in 0..MAX_HOPS {
        match fetch_body(conn, ip, port, host, &cur, extra)? {
            Fetched::Body(b) => return Ok(b),
            Fetched::Redirect(loc) => {
                if let Some(c) = conn.take() {
                    c.close();
                }
                cur = super::super::redirect::resolve(host, &cur, &loc)?;
            }
        }
    }
    Err("too many redirects")
}
