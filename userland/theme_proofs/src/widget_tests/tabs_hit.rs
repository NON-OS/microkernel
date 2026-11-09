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

//! Tabs: the index `hit` returns is the tab whose segment was painted, for
//! the hover wash and for the selected look alike, with leads or without.

use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::widgets::{Rect, Tabs};

use super::boxes::boxes;
use super::canvas::Canvas;
use super::serial::{serial, SCALES};
use super::tabs::{BUSY, ICONS, LABELS, LONGS};

/// A line for the first pixel that differs between `t` and `plain` which
/// `hit` does not give as tab `i`.
fn misnamed(t: Tabs, plain: Tabs, at: Rect, i: usize) -> Option<String> {
    let draw = |t: Tabs| Canvas::new(at, 0).painted(|fb| t.draw(fb, at));
    let lit = draw(t).diff(&draw(plain));
    let wrong = lit.into_iter().find(|&(x, y)| t.hit(at, x as i32, y as i32) != Some(i));
    wrong.map(|(x, y)| {
        format!("tab {i} in {at:?}: ({x}, {y}) hits {:?}", t.hit(at, x as i32, y as i32))
    })
}

#[test]
fn tabs_hit_names_the_tab_that_was_painted() {
    let _g = serial();
    let mut fails = Vec::new();
    for q in SCALES {
        set_scale(q);
        for labels in [&LABELS[..], &LONGS[..]] {
            for lead in [false, true] {
                let strip = |sel| match lead {
                    false => Tabs::new(labels, sel),
                    true => Tabs::new(labels, sel).icons(&ICONS).busy(&BUSY),
                };
                let plain = strip(usize::MAX);
                for at in boxes(plain.size()) {
                    for i in 0..labels.len() {
                        fails.extend(misnamed(plain.hovered(Some(i)), plain, at, i));
                        fails.extend(misnamed(strip(i), plain, at, i));
                    }
                }
            }
        }
    }
    set_scale(SCALES[0]);
    assert!(fails.is_empty(), "{} cases\n{}", fails.len(), fails.join("\n"));
}

#[test]
fn tabs_tab_is_the_box_hit_names() {
    let _g = serial();
    for q in SCALES {
        set_scale(q);
        let t = Tabs::new(&LABELS, 0);
        for at in boxes(t.size()) {
            for i in 0..LABELS.len() {
                let b = t.tab(at, i).unwrap();
                let inside =
                    |x: u32, y: u32| x >= b.x && x < b.x + b.w && y >= b.y && y < b.y + b.h;
                for (x, y) in
                    (at.x..at.x + at.w).flat_map(|x| (at.y..at.y + at.h).map(move |y| (x, y)))
                {
                    let named = t.hit(at, x as i32, y as i32) == Some(i);
                    assert_eq!(named, inside(x, y), "tab {i} in {at:?} at ({x}, {y})");
                }
            }
        }
    }
    set_scale(SCALES[0]);
    assert!(Tabs::new(&LABELS, 0).tab(Rect { x: 0, y: 0, w: 99, h: 40 }, LABELS.len()).is_none());
}
