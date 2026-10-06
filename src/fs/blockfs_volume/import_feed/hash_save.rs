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

/* A hash in flight as bytes and back, for a stream's mark on the volume. */

use alloc::vec::Vec;

use super::hash::PinHash;

/* Bytes `save` writes: the words, the count, the filling block. */
pub(super) const HASH_STATE_BYTES: usize = 32 + 8 + 64;

impl PinHash {
    pub(super) fn save(&self, out: &mut Vec<u8>) {
        self.h.iter().for_each(|w| out.extend_from_slice(&w.to_le_bytes()));
        out.extend_from_slice(&self.total.to_le_bytes());
        out.extend_from_slice(&self.buf);
    }

    pub(super) fn load(b: &[u8; HASH_STATE_BYTES]) -> Self {
        let mut s = PinHash::new();
        let word = |i: usize| u32::from_le_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]]);
        s.h.iter_mut().enumerate().for_each(|(i, w)| *w = word(i * 4));
        s.total = u64::from_le_bytes(b[32..40].try_into().unwrap_or([0; 8]));
        s.buf.copy_from_slice(&b[40..]);
        s
    }
}
