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

//! What a long lived TLS session holds between calls.

use alloc::vec::Vec;

use crate::traffic_keys::TrafficKeys;

/// A TLS 1.3 session that stays open across many reads and writes.
///
pub struct Stream {
    pub(super) app: TrafficKeys,
    pub(super) write_seq: u64,
    pub(super) read_seq: u64,
    pub(super) partial: Vec<u8>,
    pub(super) plain: Vec<u8>,
    /// The Certificate message the peer sent.
    ///
    pub certificates: Vec<u8>,
    pub(super) done: bool,
}

impl Stream {
    pub(super) fn new(app: TrafficKeys, certificates: Vec<u8>, leftover: Vec<u8>) -> Self {
        Self {
            app,
            write_seq: 0,
            read_seq: 0,
            partial: leftover,
            plain: Vec::new(),
            certificates,
            done: false,
        }
    }

    /// True once the peer has closed or the session has broken. A caller that
    /// keeps reading after this will only ever get nothing.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// The peer's leaf certificate, DER encoded, for out of band
    /// authentication. `None` if it sent none or the list is malformed.
    pub fn leaf(&self) -> Option<&[u8]> {
        crate::cert_at::cert_at(&self.certificates, 0)
    }
}
