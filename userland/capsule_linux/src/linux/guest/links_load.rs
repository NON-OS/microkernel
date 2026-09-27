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

//! Reading the image's link table.

use crate::linux::file::{key, store_read, visible};

use super::links::Links;

const TABLE: &[u8] = b"/etc/nonos-links";
const MAX_TABLE: u32 = 64 << 10;

impl Links {
    pub fn load() -> Links {
        let Ok(raw) = store_read(&key(TABLE), MAX_TABLE) else {
            return Links::default();
        };
        let pairs = raw.split(|b| *b == b'\n').filter_map(|line| {
            let at = line.iter().position(|b| *b == b' ')?;
            let (from, to) = (&line[..at], &line[at + 1..]);
            (from.first() == Some(&b'/') && !to.is_empty())
                .then(|| (visible(b"/", from), to.to_vec()))
        });
        Links(pairs.collect())
    }
}
