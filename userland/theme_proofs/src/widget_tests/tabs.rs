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

//! Tabs: every selection and hover stays in the box, overflowing strips
//! included, and the hit test takes every pixel the strip draws.

use nonos_toolkit::icons::IconId;
use nonos_toolkit::widgets::Tabs;

use super::agree::assert_agree;
use super::bounds::assert_bounded;
use super::boxes::boxes;
use super::samples::LONG;
use super::serial::serial;

/// Three short labels, multi-byte among them.
pub(crate) const LABELS: [&str; 3] = ["General", "Grüße", "日本語"];

/// An icon for the first two of `LABELS`, so the last tab has none.
pub(crate) const ICONS: [IconId; 2] = [IconId::Home, IconId::Globe];

/// The second and third of `LABELS` busy: one over an icon, one without.
pub(crate) const BUSY: [bool; 3] = [false, true, true];

/// Three labels too long for any strip here.
pub(crate) const LONGS: [&str; 3] = [LONG, LONG, "x"];

fn variants() -> Vec<(String, Tabs<'static>)> {
    let named = |n: &str, t| (n.to_string(), t);
    vec![
        named("first selected", Tabs::new(&LABELS, 0)),
        named("last selected", Tabs::new(&LABELS, 2)),
        named("none selected", Tabs::new(&LABELS, 9)),
        named("hovered", Tabs::new(&LABELS, 0).hovered(Some(1))),
        named("hovered and selected", Tabs::new(&LABELS, 0).hovered(Some(0))),
        named("hovered past the end", Tabs::new(&LABELS, 0).hovered(Some(7))),
        named("one tab", Tabs::new(&["Only"], 0)),
        named("no tabs", Tabs::new(&[], 0)),
        named("overflowing", Tabs::new(&LONGS, 1).hovered(Some(2))),
        named("icons", Tabs::new(&LABELS, 0).icons(&ICONS)),
        named("icons and busy", Tabs::new(&LABELS, 1).icons(&ICONS).busy(&BUSY)),
        named("overflowing with leads", Tabs::new(&LONGS, 0).icons(&ICONS).busy(&BUSY)),
    ]
}

#[test]
fn tabs_paint_inside_their_box() {
    let _g = serial();
    assert_bounded(&variants());
}

#[test]
fn tabs_hit_takes_every_pixel_the_strip_draws() {
    let _g = serial();
    assert_agree(&variants(), boxes, |t, at, x, y| t.hit(at, x, y).is_some());
}
