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

//! What a stream keeps until it is connected, so its open can go again.

extern crate alloc;

use alloc::vec::Vec;

use super::again::REASON_TIMEOUT;
use super::stage::StreamStage;
use super::table::Stream;

/// Bytes sent behind the BEGIN, before CONNECTED, that are kept to send
/// behind a second one. A request line or a TLS hello fits; a stream that
/// sends more before it is connected keeps nothing and does not go again.
pub const EARLY_MAX: usize = 8 * 1024;

/// How long a stream waits for a circuit whose exit takes its port, from its
/// open or from the END that sent it to another exit. The route_link tunnel
/// gives a stream 45 seconds to open, as an onion lookup is given.
pub const WAIT_SECONDS: u64 = 45;

/// The open of a stream that is not yet connected.
pub struct Held {
    /// The RELAY_BEGIN body: host, port and flags.
    pub body: Vec<u8>,
    pub port: u16,
    /// Bytes sent before CONNECTED, which the next exit is sent as well.
    pub early: Vec<u8>,
    /// The open has gone to a second exit, so the next END is passed back.
    pub again: bool,
    /// When a stream still waiting for a circuit stops waiting, and the
    /// reason it ends with then.
    pub until: u64,
    pub why: u8,
    /// Opened as an install, so it may go down the warm circuit and the next
    /// circuit is chosen by the same rule as the first (warm_rule.rs).
    pub install: bool,
}

impl Held {
    pub fn new(body: Vec<u8>, port: u16, now: u64, install: bool) -> Self {
        let (until, why) = (now.saturating_add(WAIT_SECONDS), REASON_TIMEOUT);
        Held { body, port, early: Vec::new(), again: false, until, why, install }
    }
}

/// Keep `bytes` just sent on `stream` if it is not yet connected. Past
/// EARLY_MAX the stream lets go of its open, and an END is passed back.
pub fn keep(stream: &mut Stream, bytes: &[u8]) {
    if stream.stage != StreamStage::Opening {
        return;
    }
    let Some(held) = stream.held.as_mut() else { return };
    if held.early.len().saturating_add(bytes.len()) > EARLY_MAX {
        stream.held = None;
        return;
    }
    held.early.extend_from_slice(bytes);
}
