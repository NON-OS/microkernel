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

//! Reading and writing plaintext over an open session.

extern crate alloc;

use alloc::vec::Vec;

use crate::session::{Io, SessionError};

use super::limits::{PARTIAL_MAX, PLAINTEXT_MAX};
use super::types::Stream;

impl Stream {
    /// Encrypt and send `body`, split across records if it is longer than one.
    pub fn write_all<S: Io>(&mut self, io: &mut S, body: &[u8]) -> Result<(), SessionError> {
        for chunk in body.chunks(PLAINTEXT_MAX) {
            let record = self.seal(chunk).ok_or(SessionError::Io)?;
            io.write_all(&record)?;
        }
        Ok(())
    }

    /// Plaintext that has arrived. Empty means nothing yet, not end of stream;
    /// `is_done` reports that.
    pub fn read<S: Io>(&mut self, io: &mut S) -> Result<Vec<u8>, SessionError> {
        let mut chunk = [0u8; 4096];
        loop {
            while self.absorb() {}
            if !self.plain.is_empty() || self.is_done() {
                return Ok(core::mem::take(&mut self.plain));
            }
            let n = io.read(&mut chunk)?;
            if n == 0 {
                return Ok(Vec::new());
            }
            if self.partial.len() + n > PARTIAL_MAX {
                return Err(SessionError::TooLarge);
            }
            self.partial.extend_from_slice(&chunk[..n]);
        }
    }
}
