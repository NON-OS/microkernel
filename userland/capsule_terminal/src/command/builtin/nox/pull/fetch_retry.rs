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

//! A GET on a fresh connection each try, retried with a pause when the network failed it.

use nonos_libc::{mk_time_millis, mk_yield};

use super::fetch_once::attempt;
use super::{Fetched, BACKOFF_MS, RETRIES};

pub fn get(
    ip: [u8; 4],
    port: u16,
    host: &[u8],
    path: &[u8],
    extra: &[u8],
) -> Result<Fetched, &'static str> {
    let mut last = "connect failed";
    for i in 0..RETRIES {
        match attempt(ip, port, host, path, extra) {
            Ok(body) => return Ok(body),
            Err((false, e)) => return Err(e),
            Err((true, e)) => {
                last = e;
                if i + 1 < RETRIES {
                    backoff(BACKOFF_MS);
                }
            }
        }
    }
    Err(last)
}

fn backoff(ms: i64) {
    let start = mk_time_millis();
    while mk_time_millis().wrapping_sub(start) < ms {
        mk_yield();
    }
}
