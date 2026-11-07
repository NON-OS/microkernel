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

use crate::protocol::{E_BAD_LEN, OP_COVER_TICK};
use crate::server::handlers::io::u32_at;
use crate::server::parse_req::Request;
use crate::server::respond::respond;

/// A client asking for cover is answered and nothing more is sent. Cover
/// leaves on the sender's own clocks (server/outbound), idle or not; a burst
/// sent on request would be a mark on the wire of exactly the moment a
/// client was active. The body is still checked as before.
pub fn handle(pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let errno =
        if body.is_empty() || (body.len() == 4 && u32_at(body, 0).is_ok()) { 0 } else { E_BAD_LEN };
    respond(pid, OP_COVER_TICK, errno, req.request_id, 0, tx);
}
