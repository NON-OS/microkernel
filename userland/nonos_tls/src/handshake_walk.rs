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

//! One pass over the server's encrypted handshake records.

use alloc::vec::Vec;

use super::handshake_step::{step, Step};
use super::traffic_keys::TrafficKeys;

const APPLICATION_DATA: u8 = 23;

pub(super) struct Flight {
    pub msgs: Vec<u8>,
    /// Set when the server stopped with an alert instead of finishing.
    pub alert: Option<u8>,
}

pub(super) fn walk(keys: &TrafficKeys, from: usize, bytes: &[u8]) -> Option<Flight> {
    let mut pos = from;
    let mut seq = 0u64;
    let mut out = Flight { msgs: Vec::new(), alert: None };
    while pos + 5 <= bytes.len() {
        let len = u16::from_be_bytes([bytes[pos + 3], bytes[pos + 4]]) as usize;
        let end = pos + 5 + len;
        if end > bytes.len() {
            /*
             * Records are self delimiting, so stop here and scan what arrived rather than
             * discarding a whole handshake for a trailing fragment.
             */
            break;
        }
        if bytes[pos] == APPLICATION_DATA {
            let plain = super::record_open::open(
                keys.suite,
                &keys.server_key,
                &keys.server_iv,
                seq,
                &bytes[pos..end],
            )?;
            match step(&plain) {
                Step::Messages(inner) => out.msgs.extend_from_slice(inner),
                Step::Stop(description) => {
                    /* Stop reading: the server has already stopped writing. */
                    out.alert = Some(description);
                    return Some(out);
                }
                Step::Ignore => {}
            }
            seq += 1;
        }
        pos = end;
    }
    Some(out)
}
