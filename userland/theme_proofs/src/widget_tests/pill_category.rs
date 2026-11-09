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

//! Pill: a category dot is drawn in the category's own colour, the colour
//! most of its pixels take, in every category, scheme and look while live.

use std::collections::HashMap;

use nonos_toolkit::theme::{Category, CATEGORIES};
use nonos_toolkit::widgets::Pill;

use super::canvas::Canvas;
use super::hide::Hide;
use super::mask::Masked;
use super::serial::per_theme;
use super::states::looks;
use super::widget::Widget;

/// The colour most of the dot's pixels take when `p` is drawn.
fn dot_colour(p: Pill<'static>) -> Option<u32> {
    let at = p.read_box();
    let lit = Canvas::new(at, 0).painted(|fb| p.draw(fb, at));
    let hide = Hide::Dots;
    let off = Canvas::new(at, 0).painted(|fb| p.draw(&mut Masked { fb, hide, rounds: 0 }, at));
    let mut seen: HashMap<u32, u32> = HashMap::new();
    for (x, y) in lit.diff(&off) {
        *seen.entry(lit.get(x, y)).or_default() += 1;
    }
    seen.into_iter().max_by_key(|&(c, n)| (n, c)).map(|(c, _)| c)
}

#[test]
fn pill_category_dot_is_the_category_colour() {
    let _g = super::serial::serial();
    let mut fails = Vec::new();
    per_theme(|theme| {
        for k in (0..CATEGORIES).map(Category::new) {
            for st in looks().filter(|s| !s.disabled) {
                let got = dot_colour(Pill::new("Saved").category(k).with(st));
                if got != Some(k.colour().argb()) {
                    fails.push(format!("{theme}: {k:?} {st:?} drew {got:x?}"));
                }
            }
        }
    });
    assert!(fails.is_empty(), "{} cases\n{}", fails.len(), fails.join("\n"));
}
