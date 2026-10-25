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

//! The preferred mark lands once, on the base block's first timing slot.

use super::plan::plan;
use super::rng::Rng;
use super::widths::DTD;
use super::write::as_dtd;
use crate::build::{base, put};
use crate::edid::Edid;
use crate::source::Source;

#[test]
fn the_same_timing_in_every_slot_is_preferred_once() {
    let mut r = Rng::new(0x9EEF_0001);
    for _ in 0..200 {
        let d = as_dtd(&plan(&mut r, &DTD), &mut r);
        let mut b = base();
        (0..4).for_each(|i| put(&mut b, i, &d));
        let mut marks = Vec::new();
        let e = Edid::parse(&b).unwrap();
        e.for_each_timing(&mut |t| {
            if t.source == Source::Detailed {
                marks.push(t.preferred);
            }
        });
        assert_eq!(marks, [true, false, false, false]);
    }
}
