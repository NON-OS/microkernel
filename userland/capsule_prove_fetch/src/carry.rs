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

//! One upload or download, carried to its end: on the route the person
//! chose, which must be Nym or Anyone, to the pinned relay, over TLS with
//! the chain checked, within the budgets in desk/relay.rs. Frames that come
//! in meanwhile are answered from the desk between reads and writes.

use alloc::vec::Vec;

use nonos_libc::mk_uptime_ms;
use nonos_proof_abi::wire::fetch::{Stage, NO_RELAY, RELAY, ROUTE};
use nonos_route_link::{Route, RouteStream};

use crate::desk::{get_request, put_request, Desk};
use crate::desk::{OP_MS, READ_ANYONE_MS, READ_NYM_MS, RELAY as PINNED, ROTATIONS};
use crate::io::RouteIo;

pub fn run(desk: &mut Desk, buf: &mut [u8]) {
    let Some(t) = desk.t.as_ref() else { return };
    let (anchor, up) = (t.anchor, t.stage == Stage::Uploading);
    let body = if up { Some(t.bytes.clone()) } else { None };
    let mut between = || crate::serve::drain(desk, buf);
    let got = carry(&anchor, body.as_deref(), &mut between);
    desk.finish(got);
}

fn carry(
    anchor: &[u8; 32],
    body: Option<&[u8]>,
    between: &mut dyn FnMut(),
) -> Result<Vec<u8>, i32> {
    let (host, port) = PINNED.ok_or(NO_RELAY)?;
    let route = Route::chosen();
    let read_ms = match route {
        Route::Nym(_) => READ_NYM_MS,
        Route::Anon(_) => READ_ANYONE_MS,
        Route::Direct | Route::Down(_) => return Err(ROUTE),
    };
    let until = mk_uptime_ms().saturating_add(OP_MS);
    let req = match body {
        Some(b) => put_request(host, anchor, b),
        None => get_request(host, anchor),
    };
    for _ in 0..ROTATIONS {
        if mk_uptime_ms() >= until {
            break;
        }
        let Ok(stream) = RouteStream::connect(route, host, port) else { continue };
        let mut io = RouteIo { stream, between: &mut *between, until };
        match crate::exchange::exchange(&mut io, host, &req, body.is_none(), (read_ms, until)) {
            Ok(b) => return Ok(b),
            Err(Some(s)) => return Err(s),
            Err(None) => {}
        }
    }
    Err(RELAY)
}
