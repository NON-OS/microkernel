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

//! The categorical colours: each reads, and each is told from the rest.

use crate::oklab::distance;
use crate::roles_tests::every_theme;
use crate::theme::category::{Category, CATEGORIES};
use crate::wcag::{ratio, LARGE};

/*
 * Five just-noticeable differences. Oklab's is about 0.02, so 0.10 is a step a
 * reader sees at a glance rather than finds by comparing, between every pair and
 * not only neighbours, because a group's colour is matched against a legend.
 */
const APART: f64 = 0.10;

/* A dot or a tab group's chip is a graphic that carries meaning, so WCAG 2.1
 * success criterion 1.4.11 asks 3:1 of it against whatever is beside it. */
#[test]
fn every_category_reads_as_a_graphic_on_every_ground() {
    let mut fails = Vec::new();
    for (name, r) in every_theme() {
        for i in 0..CATEGORIES {
            let c = Category::new(i).colour_in(&r).argb();
            for (g, ground) in [("bg", r.bg), ("sunken", r.sunken), ("surface", r.surface)]
                .into_iter()
                .chain([("raised", r.raised)])
            {
                let q = ratio(c, ground.argb());
                if q < LARGE {
                    fails.push(format!("{name}: category {i} {c:08X} on {g} is {q:.3}:1"));
                }
            }
        }
    }
    assert!(fails.is_empty(), "\n{}", fails.join("\n"));
}

#[test]
fn every_pair_of_categories_is_told_apart() {
    let mut fails = Vec::new();
    for (name, r) in every_theme() {
        let c: Vec<u32> = (0..CATEGORIES).map(|i| Category::new(i).colour_in(&r).argb()).collect();
        for i in 0..CATEGORIES {
            for j in i + 1..CATEGORIES {
                let d = distance(c[i], c[j]);
                if d < APART {
                    fails.push(format!("{name}: categories {i} and {j} are {d:.4} apart"));
                }
            }
        }
    }
    assert!(fails.is_empty(), "\n{}", fails.join("\n"));
}
