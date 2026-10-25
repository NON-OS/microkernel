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

//! What a read from net.anon brought, taken into the socket's buffer, and
//! the error a stream that ended early is given.

use crate::linux::abi::errno::{ECONNREFUSED, ECONNRESET, EIO, ENETUNREACH, ETIMEDOUT};

use super::super::super::anon_ops::{PAYLOAD_MAX, REASON_DESTROY, REASON_DONE, REASON_TIMEOUT};
use super::super::backend::{Anon, End};
use super::got::Got;

impl Anon {
    /// Take in what a read from net.anon brought. Only an empty buffer on a
    /// stream still open takes anything, so bytes are never put behind
    /// bytes, or after the end.
    pub fn fill(&mut self, got: Got) {
        if !self.wants_fill() {
            return;
        }
        let end = match got {
            Got::Bytes(b) if b.len() <= PAYLOAD_MAX => {
                self.heard |= !b.is_empty();
                self.rx = b;
                return;
            }
            Got::Nothing => return,
            Got::End(REASON_DONE) => End::Eof,
            Got::End(reason) => End::Error(self.ended(reason)),
            Got::Gone => End::Error(ECONNRESET),
            Got::Bytes(_) | Got::Garbled | Got::Silent => End::Error(EIO),
        };
        self.end = Some(end);
    }

    /*
     * A stream net.anon ended before a byte came back was never connected:
     * its circuit went (no route), the exit gave up on the host, or the exit
     * or the host refused it. One that ended after was reset.
     */
    fn ended(&self, reason: u8) -> i64 {
        match reason {
            _ if self.heard => ECONNRESET,
            REASON_DESTROY => ENETUNREACH,
            REASON_TIMEOUT => ETIMEDOUT,
            _ => ECONNREFUSED,
        }
    }
}
