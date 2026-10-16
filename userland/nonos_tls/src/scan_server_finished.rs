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

use super::scan_messages::{certificate, certificate_verify, finished};
use super::scan_messages::{CERTIFICATE, CERTIFICATE_REQUEST, CERTIFICATE_VERIFY, FINISHED};
use super::scan_request::certificate_request;
use super::scan_state::ScanState;

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
                state.transcript.push(&msgs[pos..end]);
            }
            return ok;
        }
        if kind == CERTIFICATE_REQUEST && !certificate_request(body, state) {
            return false;
        }
        if kind == CERTIFICATE && !certificate(body, state) {
            return false;
        }
        if kind == CERTIFICATE_VERIFY && !certificate_verify(body, state) {
            return false;
        }
        state.transcript.push(&msgs[pos..end]);
        pos = end;
    }
    false
}
