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

//! A request as a class driver sends it, and the one reply the server sends
//! back, its header checked against its length.

use nonos_libc::take_replies;

use super::bench::Bench;
use super::hc_device::lock;
use super::hc_input::Ran;
use crate::protocol::{Request, OP_ENABLE_SLOT};

/// The runner's reply buffer, no larger: a reply that would not fit the
/// capsule's fails here as it would there.
const TX: usize = 4124;
const _: () = assert!(TX == crate::server::runner::TX_LEN);

pub struct Reply {
    pub status: i32,
    pub body: Vec<u8>,
}

impl Bench {
    pub fn call(&mut self, op: u16, body: &[u8]) -> Reply {
        let req = Request { op, flags: 0, request_id: 0x5EED, payload_len: body.len() as u32 };
        let mut tx = vec![0u8; TX];
        let _ = take_replies();
        self.mux.dispatch(&req, body, &mut tx);
        let sent = take_replies();
        assert_eq!(sent.len(), 1, "one reply per request");
        let frame = &sent[0];
        assert!(frame.len() >= 24, "a header and a status");
        let payload = u32::from_le_bytes([frame[16], frame[17], frame[18], frame[19]]) as usize;
        assert_eq!(frame.len(), 20 + payload, "the header's length is the frame's");
        let status = i32::from_le_bytes([frame[20], frame[21], frame[22], frame[23]]);
        Reply { status, body: frame[24..].to_vec() }
    }

    pub fn enable_slot(&mut self) -> u8 {
        let r = self.call(OP_ENABLE_SLOT, &[]);
        assert_eq!(r.status, 0, "Enable Slot");
        r.body[0]
    }

    /// Every command the controller ran, oldest first.
    pub fn ran(&self) -> Vec<Ran> {
        lock(&self.hc).ran.clone()
    }
}
