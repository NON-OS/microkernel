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

//! Status colours on every ground.

use crate::support::{gap, luma, profiles};

/// A status colour that vanishes into the ground is worse than no colour at
/// all: it silently drops the signal an error or a path was meant to carry.
#[test]
fn status_colours_are_legible_on_every_ground() {
    for (name, t) in profiles() {
        for (role, c) in [
            ("accent", t.accent),
            ("ok", t.ok),
            ("err", t.err),
            ("path", t.path),
            ("dim", t.dim),
            ("run", t.run),
        ] {
            let d = gap(c, t.bg);
            assert!(
                d > 40,
                "{}.{}: {:#010X} luma {} vs bg luma {} — gap {}, want > 40",
                name,
                role,
                c,
                luma(c),
                luma(t.bg),
                d
            );
        }
    }
}
