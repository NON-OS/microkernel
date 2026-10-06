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

//! The sections Settings lists, and the badges on its cards. A section is
//! listed only when it has something to show, and a badge only reports what
//! was read from the running system.

use crate::settings::schema::blocks_for;
use crate::settings::schema::rows::{Pill, Row};
use crate::settings::section::{Section, SECTIONS, SECTION_COUNT};

#[test]
fn each_section_sits_at_its_own_index() {
    // The per-section cursor and scroll arrays are indexed by `index`.
    for (i, s) in SECTIONS.iter().enumerate() {
        assert_eq!(s.index(), i);
        assert!(Section::from_index(i) == *s);
    }
    assert_eq!(SECTION_COUNT, 9);
    assert!(Section::from_index(SECTION_COUNT) == Section::Developer);
}

#[test]
fn every_section_has_rows_to_show() {
    for s in SECTIONS {
        let blocks = blocks_for(s);
        assert!(blocks.iter().any(|b| !b.rows.is_empty()), "section {} is empty", s.index());
    }
}

#[test]
fn the_image_card_claims_no_signature() {
    let image = &blocks_for(Section::Updates)[0];
    assert_eq!(image.title, "System image");
    assert!(matches!(image.pill, Pill::None));
    assert!(image.rows.iter().all(|r| matches!(r, Row::Live(..))));
}
