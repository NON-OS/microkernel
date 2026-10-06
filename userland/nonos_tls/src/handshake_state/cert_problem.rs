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

//! Naming what was wrong with a certificate the handshake refused.

use super::types::HandshakeState;
use crate::cert_problem::CertProblem;

/// Handshake message type of the server's Certificate, RFC 8446 section 4.
const CERTIFICATE: u8 = 11;

impl HandshakeState {
    /*
     * For the error page only, after answer() has returned Unverified. It
     * reads the Certificate message out of the flight already decrypted and
     * asks cert_problem; it decides nothing.
     */
    /// Why the server's certificate was refused for `host` at `now`, or
    /// `None` if no reason a person can act on was found.
    pub fn cert_problem(&self, host: &[u8], now: u64) -> Option<CertProblem> {
        let msgs = self.msgs.as_slice();
        let mut pos = 0usize;
        while pos + 4 <= msgs.len() {
            let len = ((msgs[pos + 1] as usize) << 16)
                | ((msgs[pos + 2] as usize) << 8)
                | msgs[pos + 3] as usize;
            let end = pos + 4 + len;
            let body = msgs.get(pos + 4..end)?;
            if msgs[pos] == CERTIFICATE {
                return crate::cert_problem::cert_problem(body, host, now);
            }
            pos = end;
        }
        None
    }
}
