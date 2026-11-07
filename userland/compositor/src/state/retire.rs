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

//! Buffers a present took off screen, held until the frame after it is
//! composed: then nothing reads them, and their release fence is signalled
//! (frame_pacer::release). Pure, so the host proofs hold it.

pub const MAX_RETIRING: usize = 32;

pub struct Retiring {
    held: [u64; MAX_RETIRING],
    count: usize,
}

impl Retiring {
    pub const fn new() -> Self {
        Retiring { held: [0; MAX_RETIRING], count: 0 }
    }

    /// Hold `handle`; false when every place is taken, and the present is
    /// then refused rather than its buffer released early.
    pub fn hold(&mut self, handle: u64) -> bool {
        if self.held[..self.count].contains(&handle) {
            return true;
        }
        if self.count == MAX_RETIRING {
            return false;
        }
        self.held[self.count] = handle;
        self.count += 1;
        true
    }

    /// Hand every held buffer no layer shows to `release`, and stop holding
    /// it. One a layer shows again (a client that presented it once more)
    /// stays held until it is replaced.
    pub fn release_unshown(&mut self, shown: impl Fn(u64) -> bool, mut release: impl FnMut(u64)) {
        let mut kept = 0;
        for i in 0..self.count {
            let h = self.held[i];
            if shown(h) {
                self.held[kept] = h;
                kept += 1;
            } else {
                release(h);
            }
        }
        self.count = kept;
    }

    /// Room to hold one more.
    pub fn has_room(&self) -> bool {
        self.count < MAX_RETIRING
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}

impl Default for Retiring {
    fn default() -> Self {
        Self::new()
    }
}
