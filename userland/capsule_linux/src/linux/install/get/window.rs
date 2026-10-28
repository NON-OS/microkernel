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

//! An install's packages fetched (fast downloads, D): at most WIDTH GETs
//! in flight to the one mirror, one while a proxy keeps one stream per
//! caller (window_room.rs), each on a kept connection of the route the
//! install chose when one is fit, and each reply handed on in the order
//! asked, so the install still verifies and places its closure in order.

use alloc::string::String;
use alloc::vec::Vec;

use super::super::http::conn::Conn;
use super::super::http::get::request;
use super::super::mirror::mirror;
use super::super::timing;
use super::super::Why;
use super::window_end::{held, Flight};
use super::window_heap::unused;
use super::window_poll::poll_all;
use super::window_room::{cap, room};

/// Hand `each` the reply to each of `paths`, of the index's `sizes`, in
/// order: None for one not come whole. An error from `each` ends it.
pub(in crate::linux::install) fn window(
    paths: &[String],
    sizes: &[usize],
    each: &mut dyn FnMut(usize, Option<Vec<u8>>) -> Result<(), Why>,
) -> Result<(), Why> {
    let (at, port, host) = mirror();
    let (mut kept, mut flying, mut ready) = (Vec::new(), Vec::new(), Vec::new());
    let (mut next, mut give, mut mark) = (0, 0, timing::now());
    while give < paths.len() {
        while next < paths.len() && room(sizes, &held(&flying, &ready), next, unused()) {
            match start(&paths[next], (at, port, host), &mut kept) {
                Some((c, reused)) => flying.push(Flight::new(next, c, reused, cap(sizes[next]))),
                None => ready.push((next, None)),
            }
            next += 1;
        }
        let fresh = |path: &str| start(path, (at, port, host), &mut Vec::new()).map(|c| c.0);
        poll_all(&mut flying, &mut ready, &mut kept, (paths, at, &fresh), &mut mark);
        while let Some(i) = ready.iter().position(|(n, _)| *n == give) {
            let (_, body) = ready.swap_remove(i);
            each(give, body)?;
            (give, mark) = (give + 1, timing::now());
        }
    }
    Ok(())
}

/// `path` asked on a kept connection (true), else a fresh one (false).
fn start(path: &str, m: (&str, u16, &str), kept: &mut Vec<Conn>) -> Option<(Conn, bool)> {
    let req = request(path, m.2);
    if let Some(mut conn) = kept.pop() {
        if conn.send(req.as_bytes()).is_ok() {
            return Some((conn, true));
        }
    }
    let mut conn = Conn::open_or_again(m.0, m.1).ok()?;
    conn.send(req.as_bytes()).ok().map(|()| (conn, false))
}
