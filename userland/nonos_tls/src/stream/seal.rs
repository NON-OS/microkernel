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

//! Sealing one application record.

extern crate alloc;

use alloc::vec::Vec;

use super::content::APPLICATION_DATA;
use super::types::Stream;

impl Stream {
    /// One sealed application record. `None` if the cipher refuses or the write
    /// sequence has run out, which at 2^64 records it will not.
    pub fn seal(&mut self, body: &[u8]) -> Option<Vec<u8>> {
        let record = crate::record_seal::seal(
            self.app.suite,
            &self.app.client_key,
            &self.app.client_iv,
            self.write_seq,
            APPLICATION_DATA,
            body,
        )?;
        self.write_seq = self.write_seq.checked_add(1)?;
        Some(record)
    }
}
