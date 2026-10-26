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
//! No Settings card text runs past its card, in the window Settings asks for
//! on any display from 1024x768 to 2560x1440, or filling that display: each
//! note line, measured in the font it is drawn in, fits the card; no word is
//! lost to the breaking; the head is as tall as the lines it holds.

use nonos_toolkit::font::ttf::{line_height, measure};

use crate::card_text::manifest::WIDTH;
use crate::card_text::metrics::{CARD_HEAD_NOTE_H, CARD_TITLE_PX, NOTE_PX};
use crate::card_text::note_wrap::{head_h, lines, note_w};
use crate::settings::schema::blocks_for;
use crate::settings::section::SECTIONS;

pub(super) fn notes() -> impl Iterator<Item = &'static str> {
    SECTIONS.iter().flat_map(|&s| blocks_for(s).iter().filter_map(|b| b.note))
}

fn width(s: &str, px: f32) -> u32 {
    measure(s, px, false).max(0) as u32
}

#[test]
fn every_note_fits_its_card_at_every_width() {
    let above = 14 + line_height(CARD_TITLE_PX) as u32 + 2;
    let line = line_height(NOTE_PX) as u32;
    let below = CARD_HEAD_NOTE_H - above - line;
    for note in notes() {
        // The asked window on every display, up to one filling the widest.
        for w in WIDTH..=2560 {
            let mut got = Vec::new();
            let n = lines(note, note_w(w), |l| got.push(l));
            for (i, l) in got.iter().enumerate() {
                assert!(width(l, NOTE_PX) <= note_w(w), "{w}: {l}");
                // A line takes every word that fits: the next one would not.
                let next = got.get(i + 1).map(|n| n.split(' ').next().unwrap_or(""));
                let more = next.map(|word| [*l, word].join(" "));
                assert!(more.is_none_or(|m| width(&m, NOTE_PX) > note_w(w)), "{w}: {l}");
            }
            let words: Vec<_> = got.iter().flat_map(|l| l.split(' ')).collect();
            assert_eq!(words, note.split_whitespace().collect::<Vec<_>>(), "{w}");
            assert_eq!(head_h(note, w), above + n * line + below, "{w}: {note}");
        }
    }
}
