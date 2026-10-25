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

//! Out-of-range values and hostile blobs are clamped.

use crate::term::prefs::codec::{decode, encode, HEAD};
use crate::term::prefs::types::{Prefs, MAX_PROJECTS, PATH_CAP};

/// The file is untrusted input. An out-of-range theme or cursor indexes a
/// table, and an out-of-range font scale sizes every glyph on screen.
#[test]
fn out_of_range_values_are_clamped() {
    let b = encode(&Prefs {
        theme: 9999,
        font_scale: 200,
        cursor: 99,
        rails: 0xFF,
        ..Prefs::default()
    });
    let got = decode(&b);
    assert!(got.theme < 4, "theme {} out of range", got.theme);
    assert!(
        got.font_scale >= 1 && got.font_scale <= 6,
        "font_scale {} out of range",
        got.font_scale
    );
    assert!(got.cursor < 4, "cursor {} out of range", got.cursor);
    assert!(got.rails <= 0b11, "rails {:#04b} out of range", got.rails);
}

/// The slot table is fixed, and the file is untrusted: a count or a length past
/// the table must be clamped rather than indexed.
#[test]
fn a_hostile_project_blob_is_clamped() {
    let mut b = encode(&Prefs::default());
    b[HEAD] = 0xFF;
    for slot in b[HEAD + 1..].iter_mut() {
        *slot = 0xFF;
    }
    let got = decode(&b);
    assert!(got.project_count as usize <= MAX_PROJECTS, "count {}", got.project_count);
    for pr in got.project_slice() {
        assert!(pr.as_bytes().len() <= PATH_CAP, "slot overran the cap");
    }
}
