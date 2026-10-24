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

//! A join's passphrase on its way to the driver, in a buffer wiped when it
//! is dropped, whichever thread drops it and however the join went.

use super::super::cache::STRING_CAP;
use nonos_wifi_client::wipe;

/// A passphrase on its way to the driver, wiped when dropped.
pub(super) struct Secret {
    bytes: [u8; STRING_CAP],
    len: usize,
}

impl Secret {
    pub(super) fn new(from: &[u8]) -> Self {
        let mut s = Self { bytes: [0u8; STRING_CAP], len: from.len().min(STRING_CAP) };
        s.bytes[..s.len].copy_from_slice(&from[..s.len]);
        s
    }

    pub(super) fn as_slice(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}

impl Drop for Secret {
    fn drop(&mut self) {
        wipe(&mut self.bytes);
        self.len = 0;
    }
}
