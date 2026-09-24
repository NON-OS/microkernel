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

//! Taking one record off the buffer and opening it.

use super::content::{APPLICATION_DATA, CHANGE_CIPHER_SPEC};
use super::limits::BODY_MAX;
use super::open_record::open;
use super::types::Stream;

impl Stream {
    pub(super) fn absorb(&mut self) -> bool {
        let Some((kind, body_len)) = self.peek_header() else {
            return false;
        };
        let total = 5 + body_len;
        /*
         * A middlebox compatibility change_cipher_spec can arrive after the handshake
         * and is not application data. Dropped rather than counted.
         */
        if kind == CHANGE_CIPHER_SPEC {
            self.partial.drain(..total);
            return true;
        }
        if kind != APPLICATION_DATA {
            self.done = true;
            return false;
        }
        let Some(plain) = open(&self.app, self.read_seq, &self.partial[..total]) else {
            self.done = true;
            return false;
        };
        self.partial.drain(..total);
        self.read_seq = self.read_seq.saturating_add(1);
        self.dispatch(&plain);
        true
    }

    fn peek_header(&self) -> Option<(u8, usize)> {
        if self.partial.len() < 5 {
            return None;
        }
        let len = u16::from_be_bytes([self.partial[3], self.partial[4]]) as usize;
        if len > BODY_MAX || self.partial.len() < 5 + len {
            return None;
        }
        Some((self.partial[0], len))
    }
}
