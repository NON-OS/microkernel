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
//! A note breaks only where it must: one line when it fits, a word wider
//! than the card on a line of its own; and every title and row note fits.

use nonos_toolkit::font::ttf::measure;

use crate::card_text::field_note_system;
use crate::card_text::manifest::WIDTH;
use crate::card_text::metrics::{CARD_TITLE_PX, NOTE_PX};
use crate::card_text::note_wrap::{lines, note_w};
use crate::field_note_user;
use crate::settings::schema::{blocks_for, ALL_FIELDS};
use crate::settings::section::SECTIONS;

use super::card_text_tests::notes;

fn width(s: &str, px: f32) -> u32 {
    measure(s, px, false).max(0) as u32
}

#[test]
fn a_note_that_fits_keeps_one_line() {
    let mut wrapped = 0;
    for note in notes() {
        let n = lines(note, note_w(WIDTH), |_| {});
        assert_eq!(n == 1, width(note, NOTE_PX) <= note_w(WIDTH), "{note}");
        wrapped += (n > 1) as u32;
    }
    assert!(wrapped > 0, "the Network page's note is wider than the card");
}

#[test]
fn a_word_wider_than_the_card_takes_its_own_line() {
    let mut seen = Vec::new();
    assert_eq!(lines("a Unbreakable b", width("Unbreakable", NOTE_PX) - 1, |l| seen.push(l)), 3);
    assert_eq!(seen, ["a", "Unbreakable", "b"]);
}

#[test]
fn every_title_and_row_note_fits_its_card() {
    for b in SECTIONS.iter().flat_map(|&s| blocks_for(s)) {
        assert!(width(b.title, CARD_TITLE_PX) <= note_w(WIDTH), "{}", b.title);
    }
    for &f in ALL_FIELDS {
        let user = field_note_user::note(f).into_iter();
        for n in user.chain(field_note_system::note(f)) {
            assert!(width(n, NOTE_PX) <= note_w(WIDTH), "{n}");
        }
    }
}

#[test]
fn the_room_is_the_card_less_its_padding() {
    // 760 less the 212 sidebar, the pane's 28 each side and the card's 16 each side.
    assert_eq!(note_w(WIDTH), 460);
    assert_eq!(note_w(2560), 2260);
    let mut seen = Vec::new();
    assert_eq!(lines("  a  b ", 1000, |l| seen.push(l)), 1);
    assert_eq!(seen, ["a  b"]);
}
