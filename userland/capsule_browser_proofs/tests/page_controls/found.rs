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

use capsule_browser_proofs::browser::shell::cards::TabCard;
use capsule_browser_proofs::browser::shell::types::Bookmark;
use capsule_browser_proofs::browser::shell::Shell;
use capsule_browser_proofs::browser::ui::page_key::key_at;
use capsule_browser_proofs::browser::ui::page_part::PagePart;
use capsule_browser_proofs::browser::ui::rect::Rect;
use nonos_toolkit::widgets::{Controls, Step};

use super::kit::{mid, paint, spots};

/* The page's controls: each acts at the centre of the one box it acts
 * across, and every point of that box acts for it. */
pub fn laid(s: &Shell, q: u32) -> Vec<(PagePart, Rect)> {
    let mut c = Controls::acting_on_press();
    paint(s, &mut c);
    let found = spots(&c);
    for &(part, r) in &found {
        let (x, y) = mid(r);
        assert_eq!(c.pressed(x, y), Step::Act(part), "{part:?} at {q}");
        c.released();
        let corners = [(r.x, r.y), (r.right() - 1, r.bottom() - 1)];
        assert!(corners.iter().all(|&(x, y)| c.at(x, y) == Some(part)), "{part:?} at {q}");
    }
    found
}

pub fn has(found: &[(PagePart, Rect)], part: PagePart) -> Rect {
    found.iter().find(|(p, _)| *p == part).map(|(_, r)| *r).unwrap_or_else(|| panic!("{part:?}"))
}

/// The key entry or tab `i` of `s` is drawn with.
pub fn id(s: &Shell, i: usize) -> u64 {
    key_at(s, i).unwrap_or_else(|| panic!("no entry {i}"))
}

/// A bookmark added since the page was drawn.
pub fn mark(i: i64) -> Bookmark {
    Bookmark { url: format!("http://new{i}.example/"), title: "New".into(), at_ms: 90 + i }
}

/// A tab opened since the page was drawn.
pub fn new_tab() -> TabCard {
    TabCard { title: "T".into(), host: "t".into(), group: None, active: false, serial: 9 }
}
