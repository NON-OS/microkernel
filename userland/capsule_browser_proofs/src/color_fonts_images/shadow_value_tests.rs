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

//! box-shadow values the cascade must drop or reset, and a border colour
//! that must not fall back to the text colour.

use super::bg_shadow_tests::page;

#[test]
fn none_clears_and_a_broken_value_is_ignored() {
    assert!(page(".a{box-shadow:0 1px red}.a.b{box-shadow:none}").frag("a").shadow.is_none());
    let kept = page(".a{box-shadow:0 1px red}.a.b{box-shadow:0 1px 2px 3px 4px red}");
    assert_eq!(kept.frag("a").shadow.map(|s| s.layers[0].color), Some(0xFFFF_0000));
    let neg = page(".a{box-shadow:0 1px red}.a.b{box-shadow:0 1px -2px red}");
    assert_eq!(neg.frag("a").shadow.map(|s| s.layers[0].blur), Some(0), "negative blur is invalid");
}

#[test]
fn a_transparent_border_stays_transparent() {
    let clear = page("#a{color:#123456;border:2px solid transparent}");
    assert_eq!(clear.frag("a").border_color >> 24, 0, "a transparent border paints nothing");
    let plain = page("#a{color:#123456;border:2px solid}");
    assert_eq!(plain.frag("a").border_color, 0xFF12_3456, "no colour means currentColor");
}
