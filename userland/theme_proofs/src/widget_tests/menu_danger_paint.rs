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

//! Menu: the pixels a danger row changes all lie in that row, and there are
//! some, in every scheme and high contrast; every other row is drawn as it
//! was, pixel for pixel, and a menu with no danger row shows no `danger` ink. A disabled danger row draws as any disabled row.

use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{Item, Menu, Rect};

use super::boxes::boxes;
use super::canvas::Canvas;
use super::menu::MENUS;
use super::menu_danger::with_danger;
use super::serial::{per_theme, serial};

fn painted(m: Menu, at: Rect) -> Canvas {
    Canvas::new(at, roles().bg.argb()).painted(|fb| m.draw(fb, at))
}

/// A line if making live row `i` of `items` a danger row changes no pixel,
/// or one outside that row, or if the menu with no danger row already shows
/// the `danger` ink.
fn strays(name: &str, items: &[Item<'static>], i: usize) -> Option<String> {
    let (m, danger) = (Menu::new(items), with_danger(items, i));
    let at = boxes(m.size())[0];
    let plain = painted(m, at);
    let diff = plain.diff(&painted(Menu::new(&danger), at));
    let off = diff.iter().find(|&&(x, y)| m.hit(at, x as i32, y as i32) != Some(i));
    let red = plain.px.contains(&roles().danger.argb());
    let bad = diff.is_empty() || off.is_some() || red;
    bad.then(|| format!("{name} row {i}: {off:?} of {}, danger ink {red}", diff.len()))
}

#[test]
fn menu_danger_row_repaints_only_itself() {
    let _g = serial();
    let mut fails = Vec::new();
    per_theme(|theme| {
        for (name, items) in MENUS {
            let live = (0..items.len()).filter(|&i| items[i].live());
            fails.extend(
                live.filter_map(|i| strays(name, items, i)).map(|e| format!("{theme}: {e}")),
            );
        }
        let dead = [Item::new("Delete").disabled()];
        let at = boxes(Menu::new(&dead).size())[0];
        if painted(Menu::new(&dead), at).px != painted(Menu::new(&with_danger(&dead, 0)), at).px {
            fails.push(format!("{theme}: a disabled danger row draws apart"));
        }
    });
    assert!(fails.is_empty(), "{} cases\n{}", fails.len(), fails.join("\n"));
}
