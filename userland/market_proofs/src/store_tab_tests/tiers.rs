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

//! The seventeen Qwen tiers as an index, and how many of them a tab shows.

use crate::store::listing::Listing;
use crate::store::tab::Tab;

const TIERS: [&str; 17] = [
    "0.6b",
    "1.7b",
    "4b",
    "8b",
    "14b",
    "32b",
    "30b-a3b",
    "coder-0.5b",
    "coder-1.5b",
    "coder-3b",
    "coder-7b",
    "coder-14b",
    "coder-32b",
    "2.5-0.5b",
    "2.5-1.5b",
    "2.5-3b",
    "2.5-7b",
];

pub(super) fn index() -> Vec<Listing> {
    TIERS
        .iter()
        .map(|t| {
            let id = format!("linux.qwen-{t}").into_bytes();
            Listing::new(id, [0; 32], format!("Qwen {t}").into_bytes(), true)
        })
        .collect()
}

pub(super) fn shown(tab: Tab, all: &[Listing]) -> usize {
    all.iter().filter(|l| tab.accepts(l.source)).count()
}
