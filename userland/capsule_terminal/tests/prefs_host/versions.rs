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

//! Older and newer record versions.

use crate::support::{same, show};
use crate::term::prefs::codec::{decode, encode, HEAD, VERSION};
use crate::term::prefs::types::Prefs;

/// A record written by a newer build has fields this one cannot interpret;
/// reading it as version 1 would apply garbage settings.
#[test]
fn an_unknown_version_falls_back_to_defaults() {
    let mut b =
        encode(&Prefs { theme: 3, font_scale: 5, cursor: 2, rails: 0b11, ..Prefs::default() });
    b[4] = 7;
    b[5] = 0;
    let got = decode(&b);
    assert!(same(&got, &Prefs::default()), "got {}", show(&got));
}

/// A record written before the projects table existed is still a valid record:
/// its head is byte-identical to the current one, so the user's theme and font
/// scale must survive the upgrade rather than silently resetting to defaults.
#[test]
fn a_v1_record_keeps_its_head_fields() {
    let mut p = Prefs::default();
    p.theme = 2;
    p.font_scale = 3;
    let mut b = encode(&p);
    b[4..6].copy_from_slice(&1u16.to_le_bytes());
    let got = decode(&b[..HEAD]);
    assert_eq!(got.theme, 2, "v1 theme lost");
    assert_eq!(got.font_scale, 3, "v1 font scale lost");
    assert_eq!(got.project_count, 0, "v1 record cannot carry projects");
}

/// A version from the future is not decodable and must fall back cleanly.
#[test]
fn a_future_version_falls_back_to_defaults() {
    let mut b = encode(&Prefs::default());
    b[4..6].copy_from_slice(&(VERSION + 1).to_le_bytes());
    assert_eq!(decode(&b).theme, Prefs::default().theme);
}
