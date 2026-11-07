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

//! The cover over every output: the curtain is the whole canvas, and each
//! output has its own clock and field inside it, where alone the pointer is
//! drawn while locked.

use crate::shell_lock::geometry::{fields, inside, places, prompt};

/// Outputs as the compositor lays them out extended (OUTPUT_RECTS):
/// left to right, each its own height.
fn extended(sizes: &[(u32, u32)]) -> Vec<(u32, u32, u32, u32)> {
    let mut x = 0;
    sizes
        .iter()
        .map(|&(w, h)| {
            let r = (x, 0, w, h);
            x += w;
            r
        })
        .collect()
}

fn within(inner: (u32, u32, u32, u32), outer: (u32, u32, u32, u32)) -> bool {
    inner.0 >= outer.0
        && inner.1 >= outer.1
        && inner.0 + inner.2 <= outer.0 + outer.2
        && inner.1 + inner.3 <= outer.1 + outer.3
}

#[test]
fn each_output_has_its_field_and_clock_inside_it() {
    for u in [1, 2] {
        let outs = extended(&[(1920, 1080), (2560, 1440), (1280, 1024), (1366, 768)]);
        let canvas = (0, 0, outs.iter().map(|o| o.2).sum::<u32>(), 1440);
        for &o in &outs {
            let p = places(o, u);
            assert!(within(p.field, o), "field {:?} outside {o:?}", p.field);
            assert!(p.time_y >= o.1 && p.time_y < p.date_y && p.date_y < p.field.1);
            assert!(p.note_y <= o.1 + o.3);
            assert!(within(o, canvas), "the curtain, the whole canvas, covers {o:?}");
        }
        let f = fields(&outs, u);
        assert_eq!(f.len(), outs.len(), "one field an output");
        for (field, o) in f.iter().zip(&outs) {
            assert!(within(*field, *o));
        }
    }
}

#[test]
fn mirrored_or_one_output_is_one_field_in_the_middle() {
    let p = places((0, 0, 1920, 1080), 1);
    assert_eq!(p.field, (750, 518, 420, 44));
    assert!(inside(p.field, 960, 540));
    assert!(!inside(p.field, 10, 10));
}

#[test]
fn a_small_output_still_holds_its_field() {
    for o in [(0, 0, 320, 240), (0, 0, 64, 48), (100, 0, 1, 1)] {
        let p = places(o, 2);
        assert!(within(p.field, o), "{:?} in {o:?}", p.field);
    }
}

#[test]
fn the_prompt_sits_inside_the_first_output() {
    for o in [(0, 0, 1920, 1080), (0, 0, 800, 600), (1920, 0, 2560, 1440)] {
        let p = prompt(o, 1);
        assert!(within(p.panel, o));
        for r in [p.field, p.primary, p.secondary] {
            assert!(within(r, p.panel), "{r:?} in {:?}", p.panel);
        }
        assert!(p.primary.0 + p.primary.2 <= p.secondary.0, "the two buttons apart");
    }
}
