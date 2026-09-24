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

//! What the inner content type of an opened record means.

use super::content::{ALERT, APPLICATION_DATA, HANDSHAKE, KEY_UPDATE};
use super::types::Stream;

impl Stream {
    pub(super) fn dispatch(&mut self, plain: &[u8]) {
        let Some((inner, kind)) = crate::inner_plain::split(plain) else {
            self.done = true;
            return;
        };
        match kind {
            APPLICATION_DATA => self.plain.extend_from_slice(inner),
            /*
             * A KeyUpdate changes the keys for everything after it, so the session ends at
             * that record rather than reading on with keys that no longer apply.
             */
            HANDSHAKE => self.done |= inner.first() == Some(&KEY_UPDATE),
            ALERT => self.done = true,
            _ => self.done = true,
        }
    }
}
