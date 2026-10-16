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

//! Short or foreign buffers fall back to the defaults.

use crate::support::{same, show};
use crate::term::prefs::codec::{decode, encode, HEAD};
use crate::term::prefs::types::Prefs;

/// A truncated file is what a crash mid-write leaves behind. It must read as
/// "no preferences yet", and above all must not index past the end.
#[test]
fn short_buffers_fall_back_to_defaults() {
    let full =
        encode(&Prefs { theme: 3, font_scale: 5, cursor: 2, rails: 0b11, ..Prefs::default() });
    let d = Prefs::default();
    for n in 0..HEAD {
        let got = decode(&full[..n]);
        assert!(
            same(&got, &d),
            "{} bytes decoded to {}, want defaults {}",
            n,
            show(&got),
            show(&d)
        );
    }
}

/// Some other file landing at the preferences path must not be reinterpreted
/// as settings.
#[test]
fn bad_magic_falls_back_to_defaults() {
    let mut b =
        encode(&Prefs { theme: 3, font_scale: 5, cursor: 2, rails: 0b11, ..Prefs::default() });
    b[0] = b'X';
    let got = decode(&b);
    assert!(same(&got, &Prefs::default()), "got {}", show(&got));
}
