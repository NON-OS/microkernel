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

//! Walking the records to find that end.

extern crate alloc;

use alloc::vec::Vec;

use crate::traffic_keys::TrafficKeys;

use super::content::{APPLICATION_DATA, HANDSHAKE};
use super::finished::has_finished;
use super::open_record::open;
use super::span::Span;

/*
 * A session ticket after the handshake goes out under the APPLICATION keys, so
 * counting records leaves the application read sequence one out. The record
 * carrying Finished is the only exact boundary.
 */
pub(super) fn handshake_span(keys: &TrafficKeys, from: usize, bytes: &[u8]) -> Span {
    let mut pos = from;
    let mut seq = 0u64;
    let mut msgs: Vec<u8> = Vec::new();
    while pos + 5 <= bytes.len() {
        let len = u16::from_be_bytes([bytes[pos + 3], bytes[pos + 4]]) as usize;
        let end = pos + 5 + len;
        if end > bytes.len() {
            return Span::Incomplete;
        }
        if bytes[pos] == APPLICATION_DATA {
            let Some(plain) = open(keys, seq, &bytes[pos..end]) else {
                return Span::Broken;
            };
            if let Some((inner, kind)) = crate::inner_plain::split(&plain) {
                // Before the handshake case: an alert here is the peer explaining itself.
                if let Some(description) = crate::alert::description_in_plaintext(kind, inner) {
                    return Span::Alert(description);
                }
                if kind == HANDSHAKE {
                    msgs.extend_from_slice(inner);
                    if has_finished(&msgs) {
                        return Span::Found(end);
                    }
                }
            }
            seq += 1;
        }
        pos = end;
    }
    Span::Incomplete
}
