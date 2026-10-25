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

//! Rows in and out of the bond cache. Pure.

use super::cache::{BondCache, Row, CACHE_MAX};

impl BondCache {
    /// Keep `row` in place of its key's, the oldest reading leaving when full.
    pub fn put(&mut self, row: Row) {
        self.rows.retain(|r| r.key != row.key);
        if self.rows.len() >= CACHE_MAX {
            let oldest = self.rows.iter().enumerate().min_by_key(|(_, r)| r.reading.read_at);
            if let Some((i, _)) = oldest {
                self.rows.remove(i);
            }
        }
        self.rows.push(row);
    }
}
