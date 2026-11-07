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

//! The record's bytes: what the host has not been handed yet, the whole
//! record for the `RECORD` frames, and its wiping when the connection ends.

use super::Record;
use alloc::vec::Vec;

impl Record {
    /// The record's bytes the host has not been handed yet.
    pub(crate) fn take_new(&mut self) -> Vec<u8> {
        let new = self.bytes[self.handed..].to_vec();
        self.handed = self.bytes.len();
        new
    }

    /// The whole record, for the `RECORD` frames.
    pub(crate) fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub(crate) fn wipe(&mut self) {
        nonos_hash::wipe(&mut self.bytes);
        self.bytes.clear();
    }
}
