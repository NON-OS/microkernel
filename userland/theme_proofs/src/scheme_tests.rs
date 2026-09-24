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
//! Every scheme keeps its text legible.

use crate::theme::schemes::SCHEMES;
use crate::theme::select::theme_of;
use crate::wcag::{ratio, BODY};
use nonos_policy_proto::theme_labels::THEME_LABELS;

#[test]
fn every_theme_name_has_a_scheme() {
    assert_eq!(SCHEMES.len(), THEME_LABELS.len(), "one scheme per offered name");
}
/*
 * Body text against both the ground and the raised surface, for every scheme. Text
 * does not know which it will be drawn on: a card sits on the ground and a row sits
 * on the card, so a scheme that only works over one of them has a screen where the
 * reading is 3:1.
 */
#[test]
fn text_is_legible_on_ground_and_surface_in_every_scheme() {
    for (i, name) in THEME_LABELS.iter().enumerate() {
        let t = theme_of(i as u8, false);
        let name = core::str::from_utf8(name).unwrap_or("?");
        let on_bg = ratio(t.text_argb, t.background_argb);
        let on_surface = ratio(t.text_argb, t.surface_argb);
        assert!(on_bg >= BODY, "{name}: text on the ground is {on_bg:.2}:1");
        assert!(on_surface >= BODY, "{name}: text on a card is {on_surface:.2}:1");
    }
}
#[test]
fn an_unknown_theme_index_falls_back_rather_than_failing() {
    let fallback = theme_of(200, false);
    let default = theme_of(0, false);
    assert_eq!(fallback.background_argb, default.background_argb);
    assert_eq!(fallback.text_argb, default.text_argb);
}
