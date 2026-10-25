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

//! Pinned projects: round trip, truncation and bounds.

use crate::support::same;
use crate::term::prefs::codec::{decode, encode, HEAD, LEN};
use crate::term::prefs::types::{Prefs, MAX_PROJECTS};

/// Pinned project paths are settings like any other: dropping them in the codec
/// looks like the rail forgetting what the user saved.
#[test]
fn projects_round_trip() {
    let mut p = Prefs::default();
    assert!(p.push_project(b"/home/user/src"));
    assert!(p.push_project(b"/etc"));
    let got = decode(&encode(&p));
    assert!(same(&got, &p), "projects lost: {} slots back", got.project_count);
    assert_eq!(got.project_slice()[0].as_str(), "/home/user/src");
    assert_eq!(got.project_slice()[1].as_str(), "/etc");
}

/// A record whose head is intact but whose blob was cut short must still yield
/// the settings it does carry, with no projects, rather than defaults or a panic.
#[test]
fn a_truncated_project_blob_keeps_the_head_fields() {
    let mut p = Prefs { theme: 3, font_scale: 5, ..Prefs::default() };
    assert!(p.push_project(b"/home/user/src"));
    let full = encode(&p);
    for n in HEAD..LEN {
        let got = decode(&full[..n]);
        assert_eq!(got.theme, 3, "{n} bytes lost the theme");
        assert_eq!(got.font_scale, 5, "{n} bytes lost the font scale");
        assert_eq!(got.project_count, 0, "{n} bytes kept a partial project");
    }
}

/// The table is full at `MAX_PROJECTS`, and pinning the same path twice would
/// spend a slot on a row the user already has.
#[test]
fn pinning_is_bounded_and_deduplicated() {
    let mut p = Prefs::default();
    assert!(p.push_project(b"/a"));
    assert!(!p.push_project(b"/a"));
    assert!(!p.push_project(b""));
    for i in 1..MAX_PROJECTS {
        assert!(p.push_project(format!("/p{i}").as_bytes()), "slot {i} refused");
    }
    assert_eq!(p.project_count as usize, MAX_PROJECTS);
    assert!(!p.push_project(b"/overflow"));
}
