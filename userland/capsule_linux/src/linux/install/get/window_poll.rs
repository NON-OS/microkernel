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

//! One pass over the GETs in flight (window.rs): what each has brought, and
//! each whole or failed one moved to `ready`, its connection kept when fit.
//! A GET on a reused connection that the mirror has closed, nothing come,
//! goes out once more on a fresh one of the same route, and so does, once,
//! one whose reply did not come whole.

use alloc::string::String;
use alloc::vec::Vec;

use super::super::http::conn::Conn;
use super::super::http_reply::complete;
use super::super::route_read::read::QUIET_MS;
use super::super::timing;
use super::window_end::{finish, Flight};

/// `fresh` opens a new connection for a path and sends its GET.
pub(super) fn poll_all(
    flying: &mut Vec<Flight>,
    ready: &mut Vec<(usize, Option<Vec<u8>>)>,
    kept: &mut Vec<Conn>,
    (paths, at, fresh): (&[String], &str, &dyn Fn(&str) -> Option<Conn>),
    mark: &mut u64,
) {
    let mut i = 0;
    while i < flying.len() {
        let f = &mut flying[i];
        let moved = f.conn.poll(&mut f.got);
        let now = timing::now();
        f.quiet =
            if matches!(moved, Ok(n) if n > 0) { now.saturating_add(QUIET_MS) } else { f.quiet };
        let whole = complete(&f.got) && f.got.len() <= f.cap;
        let dead = moved.is_err() || f.conn.ended();
        if f.reused && f.got.is_empty() && dead {
            match fresh(&paths[f.at]) {
                Some(conn) => *f = Flight::new(f.at, conn, false, f.cap),
                None => drop(ready.push((flying.swap_remove(i).at, None))),
            }
            continue;
        }
        if !(whole || dead || now > f.quiet || f.got.len() > f.cap) {
            i += 1;
            continue;
        }
        let f = flying.swap_remove(i);
        let (n, cap, again) = (f.at, f.cap, !f.again);
        let done = finish(n, f.conn, f.got, whole, (paths, at), (kept, mark));
        // A reply that did not come whole is asked once more, on a fresh
        // connection of the same route, before the install is told.
        match (again && done.1.is_none()).then(|| fresh(&paths[n])).flatten() {
            Some(conn) => flying.push(Flight { again: true, ..Flight::new(n, conn, false, cap) }),
            None => ready.push(done),
        }
    }
    let _ = nonos_libc::mk_yield();
}
