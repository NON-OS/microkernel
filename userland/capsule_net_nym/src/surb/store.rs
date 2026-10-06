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

//! The keys of reply blocks handed out and not yet answered on.

use alloc::vec::Vec;

use super::types::SURB_KEY_BYTES;

/// Bytes of the digest a reply names its key by.
pub const DIGEST_BYTES: usize = 32;

/// How many unanswered reply block keys are kept.
///
/// A far end spends the blocks it holds oldest first, so a key is needed for
/// as long as its block sits unspent there, however many blocks were handed
/// out after it. A ring of the newest 512 dropped exactly the keys the far
/// end was about to use once a page had made a few dozen requests, and their
/// replies were dropped as addressed to somebody else. Keys now leave when
/// their reply arrives; the bound is far above what the reply block budget
/// lets the far end hold, and is only reached by blocks whose replies were
/// lost on the way, which are the oldest.
pub const KEYS_MAX: usize = 4096;

struct Entry {
    digest: [u8; DIGEST_BYTES],
    key: [u8; SURB_KEY_BYTES],
}

/// Keys by the digest a reply carries, oldest first.
pub struct KeyStore {
    entries: Vec<Entry>,
}

impl Default for KeyStore {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyStore {
    pub const fn new() -> Self {
        Self { entries: Vec::new() }
    }

    /// Keys held.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether none are held.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Keep `key`, named by `digest`, until a reply arrives on it.
    pub fn remember(&mut self, key: [u8; SURB_KEY_BYTES], digest: [u8; DIGEST_BYTES]) {
        if self.entries.len() >= KEYS_MAX {
            self.entries.remove(0);
        }
        self.entries.push(Entry { digest, key });
    }

    /// The key named by `digest`, taken out: a reply block is used once, so
    /// a second reply naming the same key is a copy and does not open.
    pub fn take(&mut self, digest: &[u8]) -> Option<[u8; SURB_KEY_BYTES]> {
        let at = self.entries.iter().position(|e| e.digest[..] == *digest)?;
        Some(self.entries.remove(at).key)
    }
}
