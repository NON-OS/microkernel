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

/*
 * A fixed set of keys. When full, the oldest goes; there is no other
 * eviction and no way to put in anything but a 32-byte key.
 */
pub(crate) struct Table<const N: usize> {
    keys: [[u8; 32]; N],
    len: usize,
    next: usize,
}

impl<const N: usize> Table<N> {
    pub(crate) const fn new() -> Self {
        Self { keys: [[0; 32]; N], len: 0, next: 0 }
    }

    pub(crate) fn contains(&self, k: &[u8; 32]) -> bool {
        self.keys[..self.len].iter().any(|x| x == k)
    }

    pub(crate) fn insert(&mut self, k: [u8; 32]) {
        if N == 0 || self.contains(&k) {
            return;
        }
        if self.len < N {
            self.keys[self.len] = k;
            self.len += 1;
            return;
        }
        self.keys[self.next] = k;
        self.next = (self.next + 1) % N;
    }

    pub(crate) fn clear(&mut self) {
        self.keys = [[0; 32]; N];
        self.len = 0;
        self.next = 0;
    }
}
