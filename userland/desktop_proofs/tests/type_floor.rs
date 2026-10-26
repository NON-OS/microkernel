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
//! The type floor (HANDOFF/ui/identity.md, Type): nothing is drawn under 12,
//! and only the mono face, which `type.label` is set in, under 14. Draw
//! and measure share the clamp, so the measure is the drawn size. One
//! process of its own, because the text-size setting is the process's.

use nonos_toolkit::font::ttf::{measure, text_scale, MIN_MONO_PX, MIN_UI_PX};

const LINE: &str = "Wallpapers kept 4 of 4";

#[test]
fn the_floors_are_the_brands() {
    assert_eq!((MIN_UI_PX, MIN_MONO_PX), (14.0, 12.0));
}

#[test]
fn text_under_its_floor_is_drawn_at_the_floor_and_no_lower() {
    text_scale::set_steps(1);
    for (mono, floor) in [(false, MIN_UI_PX), (true, MIN_MONO_PX)] {
        let at = |px: f32| measure(LINE, px, mono);
        assert_eq!(at(floor - 3.0), at(floor), "mono {mono}: under the floor is the floor");
        assert!(at(floor + 1.0) > at(floor), "mono {mono}: over it is its own size");
    }
    let (sans, mono) = (measure(LINE, 13.0, false), measure(LINE, 13.0, true));
    assert_eq!(sans, measure(LINE, 14.0, false), "13 px text is drawn at 14");
    assert!(mono < measure(LINE, 14.0, true), "a 13 px label stays 13");
}

#[test]
fn seventeen_is_no_longer_the_least() {
    text_scale::set_steps(1);
    assert!(measure(LINE, 14.0, false) < measure(LINE, 17.0, false), "a caption is smaller");
}
