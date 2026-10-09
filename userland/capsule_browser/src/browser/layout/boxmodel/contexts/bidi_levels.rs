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

use alloc::vec::Vec;

use super::super::inline_items::InlineItem;
use super::bidi::{class, Class};

/* Embedding levels for a line in a paragraph of level `para` (0 or 1): R
 * is 1, L is even, a number keeps left-to-right order inside a
 * right-to-left run (level 2 after R), and a neutral takes the direction
 * of the strong text on both sides when they agree, else the paragraph's.
 * Two passes, so a long line costs linear time. */
pub(in super::super) fn levels(items: &[(i32, InlineItem)], para: u8) -> Vec<u8> {
    let base = if para == 1 { Class::R } else { Class::L };
    let mut prev = base;
    let dirs: Vec<(Class, Class)> = items
        .iter()
        .map(|(_, it)| {
            let c = class(it);
            let d = match c {
                Class::Num if prev == Class::R => Class::R,
                Class::Num => Class::L,
                s => s,
            };
            prev = if d == Class::N { prev } else { d };
            (c, d)
        })
        .collect();
    let mut after = alloc::vec![base; dirs.len()];
    let mut seen = base;
    for i in (0..dirs.len()).rev() {
        after[i] = seen;
        seen = if dirs[i].1 == Class::N { seen } else { dirs[i].1 };
    }
    let mut before = base;
    let level = |(c, d): (Class, Class), next: Class, before: &mut Class| {
        let d = match d {
            Class::N if *before == next => next,
            Class::N => base,
            s => {
                *before = s;
                s
            }
        };
        match (c, d) {
            (Class::Num, Class::R) => 2,
            (_, Class::R) => 1,
            _ => para * 2,
        }
    };
    dirs.iter().zip(after).map(|(&cd, next)| level(cd, next, &mut before)).collect()
}
