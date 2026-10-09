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

use alloc::vec::Vec;

use super::Entry;

/* Entries grouped by a name hash: one sorted key array searched in
 * O(log n) integer compares, each key owning a run of the entries. */
#[derive(Default)]
pub(super) struct Buckets {
    keys: Vec<u64>,
    spans: Vec<(u32, u32)>,
    entries: Vec<Entry>,
}

impl Buckets {
    /* The buckets of (hash, entry) pairs, each run in rule order. */
    pub fn from(mut pairs: Vec<(u64, Entry)>) -> Buckets {
        pairs.sort_by_key(|p| p.0);
        let mut b = Buckets::default();
        for (h, e) in pairs {
            if b.keys.last() != Some(&h) {
                b.keys.push(h);
                b.spans.push((b.entries.len() as u32, b.entries.len() as u32));
            }
            b.entries.push(e);
            if let Some(span) = b.spans.last_mut() {
                span.1 += 1;
            }
        }
        b
    }

    pub fn get(&self, h: u64) -> &[Entry] {
        match self.keys.binary_search(&h) {
            Ok(k) => {
                let (a, z) = self.spans[k];
                &self.entries[a as usize..z as usize]
            }
            Err(_) => &[],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }
}
