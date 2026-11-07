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

//! An image's missing chunks over Anyone, up to four at once: each stream a
//! named conversation of the install's own on a circuit none of the others
//! is on (nonos_route_link connect_install_on, capsule_net_anon
//! spread_rule.rs), each with one ranged GET in flight, read without
//! blocking so one loop serves all. Each chunk is held to its digest and
//! kept the moment it is whole; a stream that breaks or goes quiet is
//! dropped and its chunk asked again on another (sched.rs).

use alloc::vec::Vec;
use core::ops::Range;

use nonos_app_image::ChunkList;

use super::super::http::conn::Conn;
use super::super::http_reply::{complete, reusable};
use super::super::route_read::read::QUIET_MS;
use super::super::timing;
use super::fetch::Mirror;
use super::sched::Sched;
use super::{cache, span};

/// Room for a reply's status and headers beside a chunk.
const HEAD: usize = 16 << 10;

struct Lane {
    conn: Option<Conn>,
    got: Vec<u8>,
    at: Range<u64>,
    since: u64,
    quiet: u64,
}

impl Lane {
    fn idle() -> Self {
        Lane { conn: None, got: Vec::new(), at: 0..0, since: 0, quiet: 0 }
    }
}

/// Fetch what `sched` says is missing; true when every chunk is held.
pub fn fetch(m: &Mirror, pin: &[u8; 32], list: &ChunkList, path: &str, sched: &mut Sched) -> bool {
    let mut lanes: Vec<Lane> = (0..sched.width()).map(|_| Lane::idle()).collect();
    while !sched.over() {
        for (slot, chunk) in sched.assign() {
            if !ask(m, path, list, &mut lanes[slot], slot, chunk) {
                sched.done(slot, false);
            }
        }
        for (slot, lane) in lanes.iter_mut().enumerate() {
            let Some(chunk) = sched.on(slot) else { continue };
            if let Some(ok) = read(pin, list, lane, chunk) {
                sched.done(slot, ok);
            }
        }
        let _ = nonos_libc::mk_yield();
    }
    sched.whole()
}

/// Send the GET for `chunk` on the lane's stream, opening it when it has
/// none or the kept one will not take the send.
fn ask(m: &Mirror, path: &str, list: &ChunkList, lane: &mut Lane, slot: usize, chunk: usize) -> bool {
    let Some(at) = list.range(chunk) else { return false };
    let req = span::request(path, m.host, &at);
    let sent = lane.conn.as_mut().is_some_and(|c| c.send(req.as_bytes()).is_ok());
    if !sent {
        lane.conn = None;
        let Ok(mut conn) = Conn::open_on(m.at, m.port, slot as u32 + 1) else { return false };
        if conn.send(req.as_bytes()).is_err() {
            return false;
        }
        lane.conn = Some(conn);
    }
    let now = timing::now();
    (lane.got, lane.at, lane.since, lane.quiet) = (Vec::new(), at, now, now.saturating_add(QUIET_MS));
    true
}

/// What the lane's stream has brought: None while its chunk is still coming,
/// Some(true) when it came whole and is its digest and kept, Some(false)
/// when the stream broke, went quiet or brought something else.
fn read(pin: &[u8; 32], list: &ChunkList, lane: &mut Lane, chunk: usize) -> Option<bool> {
    let conn = lane.conn.as_mut()?;
    let moved = conn.poll(&mut lane.got);
    let now = timing::now();
    if matches!(moved, Ok(n) if n > 0) {
        lane.quiet = now.saturating_add(QUIET_MS);
    }
    let cap = usize::try_from(lane.at.end - lane.at.start).unwrap_or(usize::MAX).saturating_add(HEAD);
    let whole = complete(&lane.got) && lane.got.len() <= cap;
    let dead = moved.is_err() || conn.ended() || now > lane.quiet || lane.got.len() > cap;
    if !whole && !dead {
        return None;
    }
    let raw = core::mem::take(&mut lane.got);
    timing::got(raw.len(), now.saturating_sub(lane.since));
    let fit = whole && reusable(&raw) && conn.keepable();
    if !fit {
        lane.conn = None;
    }
    let bytes = whole.then(|| span::chunk(raw, &lane.at, list.total)).flatten();
    let ok = bytes.filter(|b| list.holds(chunk, b)).map(|b| {
        // One the store would not take is fetched again when it is read.
        let _ = cache::keep_chunk(pin, chunk, &b);
    });
    if ok.is_none() {
        lane.conn = None;
    }
    Some(ok.is_some())
}
