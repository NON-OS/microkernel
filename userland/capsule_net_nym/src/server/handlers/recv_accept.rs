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

//! One gateway frame: authenticate it, refuse a repeat, route it.

use super::replay_gate::fresh;
use super::route_reply::route_reply;
use crate::gateway_client::{is_pushed_message, parse_blob};
use crate::trace;

pub(super) fn accept(tcp_port: u32, frame: &[u8], key: &[u8; 32]) {
    trace::say_num(b"gateway frame bytes", frame.len() as u64);
    let Some(incoming) = parse_blob(frame, key) else {
        trace::say(b"frame dropped: failed to authenticate under the session key");
        return;
    };
    if !is_pushed_message(incoming.kind) {
        trace::say_num(b"frame ignored: kind", incoming.kind as u64);
        return;
    }
    /*
     * After authentication, so only a frame that really came from the gateway
     * spends a slot in the window.
     */
    if !fresh(&incoming.nonce) {
        trace::say(b"frame dropped: nonce seen before, replayed");
        return;
    }
    route_reply(tcp_port, &incoming.plaintext);
}
