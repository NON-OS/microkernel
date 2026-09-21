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
//! The accent, and the scheme that overrides every other.

use crate::theme::select::theme_of;
use crate::wcag::{ratio, BODY, LARGE};
use nonos_policy_proto::theme_labels::THEME_LABELS;

/// The accent is not text, so it takes the 3:1 floor. It marks selection and focus,
#[test]
fn the_accent_is_visible_against_the_ground_in_every_scheme() {
    for (i, name) in THEME_LABELS.iter().enumerate() {
        let t = theme_of(i as u8, false);
        let name = core::str::from_utf8(name).unwrap_or("?");
        let r = ratio(t.accent_argb, t.background_argb);
        assert!(r >= LARGE, "{name}: accent is {r:.2}:1 on its own ground");
    }
}
#[test]
fn high_contrast_is_the_same_from_every_scheme() {
    let first = theme_of(0, true);
    for i in 0..THEME_LABELS.len() as u8 {
        let t = theme_of(i, true);
        assert_eq!(t.text_argb, first.text_argb, "one high-contrast scheme, not eight");
        assert_eq!(t.background_argb, first.background_argb);
    }
    let text = ratio(first.text_argb, first.background_argb);
    assert!(text >= 7.0, "AAA body text wants 7:1; this is {text:.1}:1");
    let muted = ratio(first.muted_argb, first.background_argb);
    assert!(muted >= 7.0, "and secondary text with it, at {muted:.1}:1");
    let accent = ratio(first.accent_argb, first.background_argb);
    assert!(accent >= 7.0, "and the accent, at {accent:.1}:1");
}
