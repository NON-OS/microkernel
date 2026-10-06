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


//! Handshake messages, reassembled from however the server split them
//! across records (RFC 5246 6.2.1, 7.4).

extern crate alloc;

use alloc::vec::Vec;

use super::constants::MESSAGE_MAX;
use super::error::Tls12Error;

/// Handshake bytes received and not yet taken as whole messages.
#[derive(Default)]
pub struct Messages {
    pending: Vec<u8>,
}

impl Messages {
    pub fn push(&mut self, fragment: &[u8]) -> Result<(), Tls12Error> {
        if self.pending.len().saturating_add(fragment.len()) > MESSAGE_MAX + 4 {
            return Err(Tls12Error::TooLarge);
        }
        self.pending.extend_from_slice(fragment);
        Ok(())
    }

    /// The next whole message, header included, as the transcript hashes it.
    pub fn next(&mut self) -> Result<Option<Vec<u8>>, Tls12Error> {
        if self.pending.len() < 4 {
            return Ok(None);
        }
        let len = usize::from(self.pending[1]) << 16 | usize::from(self.pending[2]) << 8 | usize::from(self.pending[3]);
        if len > MESSAGE_MAX {
            return Err(Tls12Error::TooLarge);
        }
        if self.pending.len() < 4 + len {
            return Ok(None);
        }
        Ok(Some(self.pending.drain(..4 + len).collect()))
    }

    /// True when no partial message is held. A ChangeCipherSpec may only
    /// arrive between messages, never in the middle of one.
    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }
}

/// A handshake message header for `kind` with a body of `len` bytes.
pub fn header(kind: u8, len: usize) -> [u8; 4] {
    [kind, (len >> 16) as u8, (len >> 8) as u8, len as u8]
}
