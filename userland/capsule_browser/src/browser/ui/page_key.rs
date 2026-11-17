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

use crate::browser::shell::{Page, Shell};

use super::page_part::PagePart;

/// The key of a list entry: its address and when it was made, folded.
pub fn key(url: &str, at_ms: i64) -> u64 {
    let fold = |h: u64, b: &u8| (h ^ u64::from(*b)).wrapping_mul(0x0100_0000_01b3);
    let h = url.as_bytes().iter().fold(0xcbf2_9ce4_8422_2325, fold);
    at_ms.to_le_bytes().iter().fold(h, fold)
}

/// The key of entry `i` of the list the page shows, or of tab `i` on the
/// Tabs page; None when there is no such entry.
pub fn key_at(shell: &Shell, i: usize) -> Option<u64> {
    match shell.page {
        Page::History => shell.visits.get(i).map(|v| key(&v.url, v.at_ms)),
        Page::Bookmarks => shell.bookmarks.get(i).map(|b| key(&b.url, b.at_ms)),
        Page::Downloads => shell.downloads.get(i).map(|d| key(&d.url, d.at_ms)),
        Page::Tabs => shell.cards.get(i).map(|c| u64::from(c.serial)),
        _ => None,
    }
}

/// Whether `part` still names what it was drawn over: a part naming an
/// entry or tab acts only while its index holds the same key. Any other
/// part always does.
pub fn live(shell: &Shell, part: PagePart) -> bool {
    match part {
        PagePart::Open(i, k)
        | PagePart::Remove(i, k)
        | PagePart::Tab(i, k)
        | PagePart::Close(i, k)
        | PagePart::Chip(i, k) => key_at(shell, i) == Some(k),
        _ => true,
    }
}
