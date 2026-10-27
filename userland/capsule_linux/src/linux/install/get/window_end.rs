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

//! A GET in flight (window.rs), and its end (window_poll.rs): its reply
//! said on serial and counted, its connection kept when it may carry the
//! next GET.

use alloc::string::String;
use alloc::vec::Vec;

use super::super::http::conn::Conn;
use super::super::http::get::said;
use super::super::http_reply::{body, reusable};
use super::super::route_read::read::FIRST_MS;
use super::super::timing;

pub(super) struct Flight {
    pub(super) at: usize,
    pub(super) conn: Conn,
    pub(super) reused: bool,
    pub(super) got: Vec<u8>,
    pub(super) quiet: u64,
    pub(super) cap: usize,
}

impl Flight {
    pub(super) fn new(at: usize, conn: Conn, reused: bool, cap: usize) -> Flight {
        let quiet = timing::now().saturating_add(FIRST_MS);
        Flight { at, conn, reused, got: Vec::new(), quiet, cap }
    }
}

/// Which GETs are held: in flight, or come and waiting their turn.
pub(super) fn held(flying: &[Flight], ready: &[(usize, Option<Vec<u8>>)]) -> Vec<usize> {
    flying.iter().map(|f| f.at).chain(ready.iter().map(|r| r.0)).collect()
}

/// What GET `n` brought, `raw`, as the install takes it: the body of a
/// whole 200 reply, or None.
pub(super) fn finish(
    n: usize,
    conn: Conn,
    raw: Vec<u8>,
    whole: bool,
    (paths, at): (&[String], &str),
    (kept, mark): (&mut Vec<Conn>, &mut u64),
) -> (usize, Option<Vec<u8>>) {
    let now = timing::now();
    timing::got(raw.len(), now.saturating_sub(*mark));
    *mark = now;
    let keep = whole && reusable(&raw);
    let got = whole.then(|| body(raw)).flatten();
    said(at, &paths[n], got.as_deref());
    if keep && conn.keepable() {
        kept.push(conn);
    }
    (n, got)
}
