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

//! Stepping one fetch as far as it will go now.

use super::budget::budget;
use super::deadline::due;
use super::expire::expire;
use super::types::{Fetch, Phase};
use super::wire::Wire;
use super::{connect, plain, socks, tls};

/*
 * A phase used to run once per tick: a connect that completed waited a tick
 * for its ClientHello, a finished flight a tick for its verify, and the
 * request a tick after that. Each phase now runs straight into the next
 * while the fetch moves, and stops when it waits on the network, ends, or
 * `until` has passed. A phase change counts as progress for the deadline.
 */
/// Step `f` until it waits, ends, or `until` passes. True if its phase moved.
pub fn run<W: Wire>(w: &mut W, f: &mut Fetch, until: i64) -> bool {
    let first = f.phase;
    let b = budget(w.mixnet());
    while !f.ended() {
        if let Some(reason) = due(f, b, w.now_ms()) {
            expire(f, reason);
            break;
        }
        let before = f.phase;
        match f.phase {
            Phase::Connecting => connect::connect(w, f),
            Phase::SocksHello => socks::hello(w, f),
            Phase::SocksMethod => socks::method(w, f),
            Phase::SocksConnect => socks::connect(w, f),
            Phase::TlsHello => tls::hello(w, f),
            Phase::TlsFlight => tls::read_flight(w, f, until),
            Phase::SendReq => plain::send_req(w, f),
            Phase::ReadBody => plain::read_body(w, f, b, until),
            Phase::Decrypt | Phase::Done | Phase::Error => {}
        }
        if f.phase == before || f.ended() {
            break;
        }
        f.progress_ms = w.now_ms();
        if f.progress_ms >= until {
            break;
        }
    }
    f.phase != first
}
