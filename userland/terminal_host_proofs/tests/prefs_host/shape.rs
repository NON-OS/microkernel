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

//! The documented record shape, its defaults and a full round trip.

use crate::support::{same, show};
use crate::term::prefs::codec::{decode, encode, HEAD, LEN, MAGIC, VERSION};
use crate::term::prefs::rail_flags::RAIL_VISIBLE;
use crate::term::prefs::types::{Prefs, MAX_PROJECTS, PATH_CAP};

/// The record is written to a file that outlives the build, so its shape is a
/// compatibility contract, not an implementation detail.
#[test]
fn the_record_shape_is_the_documented_one() {
    assert_eq!(HEAD, 12);
    assert_eq!(LEN, HEAD + 1 + MAX_PROJECTS * (1 + PATH_CAP));
    assert_eq!(MAGIC, *b"NTP1");
    assert_eq!(VERSION, 2);
}

/// These are the values `State::new` hardcoded before preferences existed, so
/// a first boot with no file on disk must look exactly like the old build.
#[test]
fn the_defaults_match_the_historical_hardcoded_values() {
    let d = Prefs::default();
    assert_eq!(d.font_scale, 2);
    assert_eq!(d.theme, 0);
    assert_eq!(d.cursor, 0);
    assert_eq!(d.rails & RAIL_VISIBLE, 0, "the left rail starts hidden");
}

/// Every setting a reader can change must survive a save and a reboot; a
/// field dropped in the codec looks like the setting silently not sticking.
#[test]
fn every_field_round_trips() {
    let p = Prefs { theme: 3, font_scale: 5, cursor: 2, rails: 0b11, ..Prefs::default() };
    let got = decode(&encode(&p));
    assert!(same(&got, &p), "expected {}, got {}", show(&p), show(&got));
}
