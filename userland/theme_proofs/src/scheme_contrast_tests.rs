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
//! The contrast floor each derived text role has to clear.

use crate::theme::select::theme_of;
use crate::wcag::{ratio, BODY, LARGE};
use nonos_policy_proto::theme_labels::THEME_LABELS;

/*
 * And the derived secondary text, which is the reason the muting is a walk rather
 * than a fixed step. Solarized has 4.86:1 between its own text and its own surface,
 * so a third of the way to the ground would put its captions at 2.81:1. This is
 * what fails if the walk is ever replaced by a constant.
 */
#[test]
fn muted_holds_the_contrast_floor() {
    for (i, name) in THEME_LABELS.iter().enumerate() {
        let t = theme_of(i as u8, false);
        let name = core::str::from_utf8(name).unwrap_or("?");
        for (ground, where_) in [(t.background_argb, "the ground"), (t.surface_argb, "a card")] {
            let r = ratio(t.muted_argb, ground);
            assert!(r >= BODY, "{name}: muted text on {where_} is {r:.2}:1, under {BODY}");
        }
    }
}
/// Placeholders are held to the 3:1 floor rather than 4.5:1, and must actually
#[test]
fn placeholder_text_clears_the_large_text_floor() {
    for (i, name) in THEME_LABELS.iter().enumerate() {
        let t = theme_of(i as u8, false);
        let name = core::str::from_utf8(name).unwrap_or("?");
        for ground in [t.background_argb, t.surface_argb] {
            let r = ratio(t.quiet_argb, ground);
            assert!(r >= LARGE, "{name}: placeholder is {r:.2}:1, under {LARGE}");
        }
    }
}
#[test]
fn muted_is_actually_quieter_than_body_text() {
    for (i, name) in THEME_LABELS.iter().enumerate() {
        let t = theme_of(i as u8, false);
        let name = core::str::from_utf8(name).unwrap_or("?");
        assert_ne!(t.muted_argb, t.text_argb, "{name}: secondary text is not secondary");
        let body = ratio(t.text_argb, t.background_argb);
        let muted = ratio(t.muted_argb, t.background_argb);
        assert!(muted < body, "{name}: muted {muted:.2} must sit below body {body:.2}");
    }
}
