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

//! Which profiles are light or dark, and the dark ground value.

use crate::support::luma;
use crate::term::theme::profiles::{ABYSS, DARK, DIM, LIGHT};

/// Guards against a future edit quietly turning "light" into a mid-grey, or
/// letting a dark ground drift up until the whole surface washes out.
#[test]
fn light_is_light_and_the_rest_are_dark() {
    assert!(
        luma(LIGHT.bg) > 200,
        "LIGHT.bg {:#010X} luma {}, want > 200",
        LIGHT.bg,
        luma(LIGHT.bg)
    );
    for (name, t) in [("DARK", DARK), ("DIM", DIM), ("ABYSS", ABYSS)] {
        assert!(luma(t.bg) < 60, "{}.bg {:#010X} luma {}, want < 60", name, t.bg, luma(t.bg));
    }
}

/// The default look must not shift underfoot. Only the background is pinned:
/// DARK's accents deliberately DID move in the refactor (accent 0x3FD0C9 ->
/// 0x5AE6D0, with path/ok/err following). Do not "fix" this test by adding
/// the old accent values back — they are intentionally gone.
#[test]
fn darks_background_is_unchanged_from_the_pre_refactor_constant() {
    assert_eq!(DARK.bg, 0xFF07_090B, "DARK.bg is {:#010X}, want 0xFF07090B", DARK.bg);
}
