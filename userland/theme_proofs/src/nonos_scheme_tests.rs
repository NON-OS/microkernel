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

//! The NØNOS schemes hold the brand's own numbers (nonos.software/brand), and
//! hold them to more than the floors every scheme meets: the accent is the one
//! thing a view spends cyan on, the current place or the next action, so it
//! has to read as text, not only as a mark.

use crate::theme::select::theme_of;
use crate::wcag::{ratio, BODY};
use nonos_policy_proto::theme_labels::THEME_LABELS;

/// The index of a scheme by its label.
fn at(label: &[u8]) -> u8 {
    THEME_LABELS.iter().position(|l| *l == label).expect("labelled") as u8
}

#[test]
fn the_default_is_ink_in_the_brand_s_values() {
    assert_eq!(at(b"Ink"), 0, "a stored 0 and an unknown index both mean Ink");
    let t = theme_of(0, false);
    assert_eq!(t.background_argb, 0xFF0A_0B0D, "Ink");
    assert_eq!(t.text_argb, 0xFFF2_F3F7, "Paper");
    assert_eq!(t.accent_argb, 0xFF66_FFFF, "NØNOS Cyan");
}

#[test]
fn each_nonos_accent_reads_as_text_on_its_ground_and_surface() {
    for label in [&b"Ink"[..], b"Paper"] {
        let t = theme_of(at(label), false);
        for ground in [t.background_argb, t.surface_argb] {
            let r = ratio(t.accent_argb, ground);
            assert!(
                r >= BODY,
                "{:?}: accent {r:.2}:1 is under {BODY}",
                core::str::from_utf8(label)
            );
        }
    }
}

#[test]
fn paper_is_light_ink_is_dark_and_both_read() {
    let (ink, paper) = (theme_of(at(b"Ink"), false), theme_of(at(b"Paper"), false));
    let lum = |c: u32| ((c >> 16) & 0xFF) + ((c >> 8) & 0xFF) + (c & 0xFF);
    assert!(lum(ink.background_argb) < lum(ink.text_argb), "Ink: light text on a dark ground");
    assert!(
        lum(paper.background_argb) > lum(paper.text_argb),
        "Paper: dark text on a light ground"
    );
    for t in [ink, paper] {
        assert!(ratio(t.text_argb, t.background_argb) >= 7.0, "body text at AAA");
    }
}
