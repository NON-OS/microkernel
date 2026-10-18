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

//! What a change on the displays' rows is kept as, and that the shell puts
//! back the very body Settings sent live: the same field ids, values it
//! accepts, and the same bytes for the same display and scale.

use super::shell_keep::display_keep::{body, valid, FIELDS};
use crate::settings::schema::access_knob::Knob;
use crate::settings::screens::arrange::{ScreenPage, Shown, OUTPUTS, SCALES};
use crate::settings::screens::keep::kept;
use crate::settings::screens::wire::{layout, scale};

fn page(extend: bool, output: u32, q: u32) -> ScreenPage {
    let mut chosen = [0; 4];
    chosen[output as usize] = q;
    let shown = Shown { outputs: 4, extend, extended: extend, scale_q: 4, chosen };
    ScreenPage { shown, output, read: true, levels: [None; 4] }
}

#[test]
fn the_arrangement_is_kept_and_replayed_as_sent() {
    for extend in [false, true] {
        let (field, v) = kept(&page(extend, 0, 0), Knob::Arrange).unwrap();
        assert_eq!(field, FIELDS[0]);
        let (b, len) = body(0, v).unwrap();
        assert_eq!(&b[..len], &layout(extend), "replayed as Settings sent it");
    }
}

#[test]
fn each_displays_scale_is_kept_and_replayed_as_sent() {
    for o in 0..OUTPUTS {
        for q in SCALES {
            let (field, v) = kept(&page(false, o, q), Knob::Scale).unwrap();
            let i = FIELDS.iter().position(|&f| f == field).unwrap();
            assert!(valid(i, v), "display {o} at {q}");
            let (b, len) = body(i, v).unwrap();
            assert_eq!(&b[..len], &scale(o, q).unwrap(), "display {o} at {q}");
        }
    }
}

#[test]
fn nothing_else_is_kept() {
    assert_eq!(kept(&page(true, 1, 8), Knob::Screen), None, "choosing the display");
    assert_eq!(kept(&page(false, 2, 7), Knob::Scale), None, "a scale it would not take");
    assert_eq!(FIELDS, [0x0126, 0x0127, 0x0128, 0x0129, 0x012A], "the ids approved");
}
