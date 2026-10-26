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

//! A row's label is cut to the room left of its value, so the two never
//! overlap (booted: the Qwen model row's note ran under its value). Measured
//! here at a fixed advance; on the device the font's own measure is used.

use crate::text::fit::fit;

/// Eight pixels a character, the ellipsis included.
fn w(s: &str) -> u32 {
    s.chars().count() as u32 * 8
}

#[test]
fn a_label_that_fits_is_left_whole() {
    assert_eq!(fit("Hostname", 64, w), "Hostname");
    assert_eq!(fit("", 0, w), "");
}

#[test]
fn a_long_label_is_cut_inside_its_room_with_an_ellipsis() {
    let note = "The model qwen runs when no tier is named. Left and right choose.";
    for room in [8, 40, 100, 300, 519] {
        let cut = fit(note, room, w);
        assert!(w(&cut) <= room, "{cut:?} is {} wide, room {room}", w(&cut));
        assert!(cut.ends_with('…'), "a cut says it was cut: {cut:?}");
        assert!(note.starts_with(cut.trim_end_matches('…')), "only the end goes");
    }
}

#[test]
fn no_room_gives_nothing_and_never_splits_a_character() {
    assert_eq!(fit("Ø Network", 4, w), "");
    let cut = fit("ØØØØØØØØ", 40, w);
    assert_eq!(cut, "ØØØØ…");
}
