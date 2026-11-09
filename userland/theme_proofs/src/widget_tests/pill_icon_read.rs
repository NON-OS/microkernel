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

//! Pill: an icon reads at 3:1 in the label's ink in every tone, category,
//! scheme and state, stays in the box, and the hit test takes the body.

use nonos_toolkit::icons::IconId;
use nonos_toolkit::theme::{Category, CATEGORIES};
use nonos_toolkit::widgets::Pill;

use super::agree::assert_agree;
use super::bounds::assert_bounded;
use super::boxes::boxes;
use super::hide::Hide;
use super::legible::{assert_legible, Off, Read};
use super::pill::TONES;
use super::samples::LONG;
use super::serial::serial;
use crate::wcag::LARGE;

const LABEL: &str = "History clears at shutdown";

fn variants() -> Vec<(String, Pill<'static>)> {
    let mut all = Vec::new();
    for (name, tone) in TONES {
        for label in [LABEL, LONG] {
            let p = Pill::new(label).tone(tone).icon(IconId::Shield);
            all.push((format!("{name} {label:?}"), p));
            all.push((format!("{name} {label:?} with dot"), p.dot().count(12)));
        }
    }
    let p = Pill::new(LABEL).icon(IconId::Shield);
    for k in (0..CATEGORIES).map(Category::new) {
        all.extend([(format!("cat {k:?}"), p.category(k)), (format!("fill {k:?}"), p.filled(k))]);
    }
    all
}

#[test]
fn pill_icon_reads_in_every_tone_and_state() {
    let _g = serial();
    let read = |what, on| Read { what, on, off: Off::Hide(Hide::Icons), floor: LARGE };
    let p = Pill::new(LABEL).icon(IconId::Shield);
    let mut reads: Vec<_> = TONES.into_iter().map(|(n, t)| read(n, p.tone(t))).collect();
    reads.extend(TONES.into_iter().map(|(_, t)| read("dotted", p.tone(t).dot())));
    for k in (0..CATEGORIES).map(Category::new) {
        reads.extend([read("category", p.category(k)), read("filled", p.filled(k))]);
    }
    assert_legible(&reads);
}

#[test]
fn pill_icon_paints_inside_its_box() {
    let _g = serial();
    assert_bounded(&variants());
}

#[test]
fn pill_icon_hit_takes_the_body_it_draws() {
    let _g = serial();
    assert_agree(&variants(), boxes, |p, at, x, y| p.hit(at, x, y));
}
