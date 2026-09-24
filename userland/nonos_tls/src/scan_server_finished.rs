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

//! Walking the server's handshake messages through to Finished.

use alloc::vec::Vec;

use super::scan_messages::{certificate, certificate_verify, finished};
use super::scan_messages::{CERTIFICATE, CERTIFICATE_VERIFY, FINISHED};

pub struct ScanState<'a> {
    pub secret: &'a [u8; 32],
    pub transcript: &'a mut Vec<u8>,
    pub host: &'a [u8],
    pub now: u64,
    pub cert11: &'a mut Vec<u8>,
    pub validated: &'a mut bool,
    /// Whether the certificate has to chain to a trusted root for this host.
    ///
    /// True for every ordinary connection. False only where the caller
    /// authenticates the peer itself, which it must then actually do: the
    /// session is otherwise bound to nothing but whoever answered.
    pub require_chain: bool,
}

pub fn scan(msgs: &[u8], state: &mut ScanState) -> bool {
    let mut pos = 0usize;
    while pos + 4 <= msgs.len() {
        let len = ((msgs[pos + 1] as usize) << 16)
            | ((msgs[pos + 2] as usize) << 8)
            | msgs[pos + 3] as usize;
        let end = pos + 4 + len;
        if end > msgs.len() {
            return false;
        }
        let kind = msgs[pos];
        let body = &msgs[pos + 4..end];
        /*
         * Finished returns from inside the loop rather than falling through,
         * because it ends the flight and, on failure, must not reach the
         * transcript. Every other message that is accepted is appended below.
         */
        if kind == FINISHED {
            let ok = finished(body, state);
            if ok {
                state.transcript.extend_from_slice(&msgs[pos..end]);
            }
            return ok;
        }
        if kind == CERTIFICATE && !certificate(body, state) {
            return false;
        }
        if kind == CERTIFICATE_VERIFY && !certificate_verify(body, state) {
            return false;
        }
        state.transcript.extend_from_slice(&msgs[pos..end]);
        pos = end;
    }
    false
}
