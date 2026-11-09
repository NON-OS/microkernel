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

//! Pill: with a count, a category dot or a category fill, in every category,
//! the pill stays in the box and the hit test takes the body it draws.

use nonos_toolkit::theme::{Category, CATEGORIES};
use nonos_toolkit::widgets::Pill;

use super::agree::assert_agree;
use super::bounds::assert_bounded;
use super::boxes::boxes;
use super::samples::LONG;
use super::serial::serial;

/// Every category, dotted and filled, with and without a count, and a
/// counted pill of each tone.
pub(crate) fn variants() -> Vec<(String, Pill<'static>)> {
    let mut all = Vec::new();
    for label in ["Saved", LONG] {
        for (i, k) in (0..CATEGORIES).map(|i| (i, Category::new(i))) {
            let (dot, fill) = (Pill::new(label).category(k), Pill::new(label).filled(k));
            all.push((format!("category {i} {label:?}"), dot));
            all.push((format!("category {i} {label:?} counted"), dot.count(7)));
            all.push((format!("filled {i} {label:?}"), fill));
            all.push((format!("filled {i} {label:?} counted"), fill.count(1234)));
        }
        for (name, tone) in super::pill::TONES {
            all.push((format!("{name} {label:?} counted"), Pill::new(label).tone(tone).count(42)));
        }
    }
    all
}

#[test]
fn pill_marks_paint_inside_the_box() {
    let _g = serial();
    assert_bounded(&variants());
}

#[test]
fn pill_marks_hit_takes_the_body_it_draws() {
    let _g = serial();
    assert_agree(&variants(), boxes, |p, at, x, y| p.hit(at, x, y));
}
