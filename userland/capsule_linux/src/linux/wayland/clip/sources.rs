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

//! The mime types each of a guest's sources offers, as the bridge keeps
//! them. Pure, so the host proofs hold it.

use alloc::vec::Vec;

use super::mime::TEXT;
use super::state::{Clip, DESKTOP};

/// The most mime types one source may offer, and the longest one kept: a
/// real source offers a handful, and a client offering endlessly would
/// otherwise grow this capsule's memory.
const MOST_MIMES: usize = 64;
const LONGEST_MIME: usize = 256;

impl Clip {
    /// `source` offers `mime`, kept within the bounds above.
    pub fn add_mime(&mut self, source: u32, mime: &[u8]) {
        if !self.sources.iter().any(|s| s.0 == source) {
            self.sources.push((source, Vec::new()));
        }
        let Some((_, mimes)) = self.sources.iter_mut().find(|s| s.0 == source) else { return };
        if mimes.len() < MOST_MIMES
            && mime.len() <= LONGEST_MIME
            && !mimes.iter().any(|m| m == mime)
        {
            mimes.push(mime.to_vec());
        }
    }

    /// The mime types an offer of `source` names: what the source offered,
    /// or for the desktop, the text types.
    pub fn mimes_of(&self, source: u32) -> Vec<Vec<u8>> {
        if source == DESKTOP {
            return TEXT.iter().map(|m| m.to_vec()).collect();
        }
        self.sources.iter().find(|s| s.0 == source).map(|s| s.1.clone()).unwrap_or_default()
    }
}
