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

//! Pin resolution over a synthetic layout, made up for the proof and named
//! after no platform: a group's firmware numbers mapped onto its pads, a
//! group whose numbering starts at zero, a gap and an unnumbered group
//! refused, and the pad counted from its own community's first pad.

use nonos_pinctrl::{resolve, Community, Group, Layout, Pad, PinError};

const SYNTHETIC: Layout = Layout {
    name: "synthetic",
    communities: &[
        Community {
            first: 0,
            groups: &[
                Group { first: 0, size: 24, gpio: Some(0) },
                Group { first: 24, size: 8, gpio: None },
            ],
        },
        Community {
            first: 32,
            groups: &[
                Group { first: 32, size: 16, gpio: Some(64) },
                Group { first: 48, size: 4, gpio: Some(0) },
            ],
        },
    ],
};

#[test]
fn numbers_resolve_to_their_community_and_pad() {
    assert_eq!(resolve(&SYNTHETIC, 5), Ok(Pad { bar: 0, pad: 5 }));
    assert_eq!(resolve(&SYNTHETIC, 64), Ok(Pad { bar: 1, pad: 0 }));
    assert_eq!(resolve(&SYNTHETIC, 79), Ok(Pad { bar: 1, pad: 15 }));
}

#[test]
fn the_first_group_holding_a_number_wins() {
    // Numbers 0..=3 are held by both the first group and the zero-based
    // one; table order decides, as a datasheet's order would.
    assert_eq!(resolve(&SYNTHETIC, 2), Ok(Pad { bar: 0, pad: 2 }));
}

#[test]
fn gaps_and_unnumbered_groups_are_refused() {
    assert_eq!(resolve(&SYNTHETIC, 30), Err(PinError::NotMapped), "between the groups");
    assert_eq!(resolve(&SYNTHETIC, 80), Err(PinError::NotMapped), "past the last");
}
