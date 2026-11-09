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

//! Pill: a count reads at the label's 4.5:1 on a plain, dotted, toned and
//! category pill in every state, and on a filled pill the label and count
//! read at 4.5:1 on the category's fill, in every category.

use nonos_toolkit::theme::{Category, CATEGORIES};
use nonos_toolkit::widgets::Pill;

use super::hide::Hide;
use super::legible::{assert_legible, Off, Read};
use super::pill::TONES;
use super::serial::serial;
use crate::wcag::BODY;

const LABEL: &str = "Grüße — 日本語";
const COUNT: &str = "1234";

/// A read of `hide` on `on` at the label's floor.
fn read(what: &'static str, on: Pill<'static>, hide: &'static str) -> Read<Pill<'static>> {
    Read { what, on, off: Off::Hide(Hide::Text(hide)), floor: BODY }
}

#[test]
fn pill_count_reads_in_every_tone_and_state() {
    let _g = serial();
    let mut reads = Vec::new();
    for (name, tone) in TONES {
        let pill = Pill::new(LABEL).tone(tone).count(1234);
        reads.push(read(name, pill, COUNT));
        reads.push(read(name, pill.dot(), COUNT));
    }
    reads.push(read("category", Pill::new(LABEL).category(Category::new(0)).count(1234), COUNT));
    assert_legible(&reads);
}

#[test]
fn filled_pill_label_and_count_read_on_the_fill() {
    let _g = serial();
    let mut reads = Vec::new();
    for k in (0..CATEGORIES).map(Category::new) {
        let pill = Pill::new(LABEL).filled(k).count(1234);
        reads.push(read("filled label", pill, LABEL));
        reads.push(read("filled count", pill, COUNT));
    }
    assert_legible(&reads);
}
