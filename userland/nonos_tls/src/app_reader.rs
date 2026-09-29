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

//! Decrypting a response as it arrives, each record once.

use alloc::vec::Vec;

use super::record_frame::{record_at, APPLICATION_DATA};
use super::traffic_keys::TrafficKeys;

/*
 * The response used to be decrypted from its first record on every read, so
 * a page arriving a few kilobytes at a time was opened again and again:
 * quadratic in records, and each open was a kernel round trip. This keeps the
 * wire offset, the next sequence number and the plaintext so far, and opens
 * only the records that completed since the last call.
 */
/// The server's application records, opened in order as they complete.
pub struct AppReader {
    pub(crate) cursor: usize,
    pub(crate) seq: u64,
    pub(crate) plain: Vec<u8>,
    pub(crate) broken: bool,
}

impl AppReader {
    pub const fn new() -> Self {
        Self { cursor: 0, seq: 0, plain: Vec::new(), broken: false }
    }

    /// Open the records of `wire` completed since the last call, and return
    /// how many were opened. A record that fails to open ends the stream:
    /// nothing after a forged or damaged record is believed.
    pub fn feed(&mut self, app: &TrafficKeys, wire: &[u8]) -> usize {
        let mut opened = 0;
        while !self.broken {
            let Some((kind, end)) = record_at(wire, self.cursor) else { break };
            if kind == APPLICATION_DATA {
                let record = &wire[self.cursor..end];
                let (key, iv) = (&app.server_key, &app.server_iv);
                match crate::record_open::open(app.suite, key, iv, self.seq, record) {
                    Some(plain) => self.take(&plain),
                    None => self.broken = true,
                }
                self.seq += 1;
                opened += 1;
            }
            self.cursor = end;
        }
        opened
    }

    /// Application data only: session tickets and alerts carry none.
    fn take(&mut self, plain: &[u8]) {
        if let Some((body, APPLICATION_DATA)) = crate::inner_plain::split(plain) {
            self.plain.extend_from_slice(body);
        }
    }
}
