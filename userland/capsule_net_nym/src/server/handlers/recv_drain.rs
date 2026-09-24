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

use alloc::vec;

use super::recv_accept::accept;
use super::recv_control::control;
use crate::gateway_client::{self, E_RECV_TIMEOUT};
use crate::protocol::WIRE_PACKET_MAX;
use crate::setup;
use crate::state::TABLE;
use crate::trace;

/// Frames to take in one pass.
const BURST: usize = 16;

/// Take whatever the gateway has pushed and route it.
pub fn drain_stream(wait_ms: i64) {
    let tcp_port = setup::tcp_port();
    let gateway = match TABLE.lock().gateway() {
        Some(gateway) if tcp_port != 0 => gateway,
        _ => return,
    };
    let mut chunk = vec![0u8; WIRE_PACKET_MAX];
    for pass in 0..BURST {
        // Only the first pass waits.
        let budget = if pass == 0 { wait_ms } else { 0 };
        let frame = match gateway_client::recv(tcp_port, gateway, &mut chunk, budget) {
            Ok(frame) => frame,
            Err(e) => {
                // An empty wait is the normal state of a link with nothing in
                // flight. Anything else is the link itself in trouble.
                if e != E_RECV_TIMEOUT {
                    trace::say_num(b"gateway link error", e as u64);
                }
                return;
            }
        };
        if frame.len == 0 {
            return;
        }
        if frame.text {
            control(tcp_port, gateway.stream, &chunk[..frame.len]);
            continue;
        }
        accept(tcp_port, &chunk[..frame.len], &gateway.shared_key);
    }
}
