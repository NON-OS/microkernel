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

//! Text, run and dim colours stay legible on their ground.

use crate::support::{gap, luma, profiles};
use crate::term::theme::profiles::LIGHT;

/// The profiles replaced a set where the old `light` entry swapped only the
/// background and kept the near-white foreground constants, painting white on
/// white. Every profile must carry its own foreground far from its own ground.
#[test]
fn every_profile_separates_text_from_ground() {
    for (name, t) in profiles() {
        let d = gap(t.fg, t.bg);
        assert!(
            d > 90,
            "{}: fg {:#010X} luma {} vs bg {:#010X} luma {} — gap {}, want > 90",
            name,
            t.fg,
            luma(t.fg),
            t.bg,
            luma(t.bg),
            d
        );
    }
}

/// `run` ("a command is running") and `dim` ("muted text") are distinct roles
/// that happen to share a value on the three dark grounds today — that
/// coincidence is legitimate and deliberately not asserted here. LIGHT is
/// where they were pulled apart on purpose, so only LIGHT is pinned.
#[test]
fn run_and_dim_stay_distinguishable_on_light() {
    assert_ne!(
        luma(LIGHT.run),
        luma(LIGHT.dim),
        "LIGHT run {:#010X} and dim {:#010X} both land on luma {}",
        LIGHT.run,
        LIGHT.dim,
        luma(LIGHT.run)
    );
}
