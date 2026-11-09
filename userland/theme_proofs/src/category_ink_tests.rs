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

//! Text on a category's fill, and an index past the eight.

use crate::roles_tests::every_theme;
use crate::theme::category::{Category, CATEGORIES};
use crate::wcag::{ratio, BODY};

/* A letter tile or a group's name on its chip is body text, so 4.5:1. */
#[test]
fn category_ink_reads_on_its_fill() {
    let mut fails = Vec::new();
    for (name, r) in every_theme() {
        for i in 0..CATEGORIES {
            let (k, fill) = (Category::new(i), Category::new(i).colour_in(&r).argb());
            let q = ratio(k.ink_in(&r).argb(), fill);
            if q < BODY {
                fails.push(format!("{name}: ink on category {i} {fill:08X} is {q:.3}:1"));
            }
        }
    }
    assert!(fails.is_empty(), "\n{}", fails.join("\n"));
}

#[test]
fn an_index_wraps_to_one_of_the_eight() {
    for (_, r) in every_theme() {
        for i in 0..3 * CATEGORIES {
            let (k, home) = (Category::new(i), Category::new(i % CATEGORIES));
            assert_eq!(k, home);
            assert!(k.index() < CATEGORIES);
            assert_eq!(k.colour_in(&r), home.colour_in(&r));
        }
    }
    assert_eq!(Category::new(usize::MAX).index(), usize::MAX % CATEGORIES);
}
