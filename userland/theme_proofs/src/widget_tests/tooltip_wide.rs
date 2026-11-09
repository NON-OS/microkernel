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

//! Tooltip: `max_width` moves the cap. Asking for `TOOLTIP_MAX_W` draws
//! what an unasked tooltip draws; a wider cap shows a long text whole and a
//! narrower one cuts it with `…` at exactly that width.

use nonos_toolkit::design::scale::{scale, set_scale};
use nonos_toolkit::design::size::TOOLTIP_MAX_W;
use nonos_toolkit::widgets::{Rect, Tooltip};

use super::canvas::Canvas;
use super::samples::LONG;
use super::serial::{serial, SCALES};
use super::toast_lines::Runs;
use super::tooltip::TEXTS;

/// The runs `t` draws in a box its own size.
fn drawn(t: Tooltip) -> Vec<String> {
    let mut runs = Runs(Vec::new());
    t.draw(&mut runs, Rect { x: 24, y: 24, w: t.size().0, h: t.size().1 });
    runs.0
}

#[test]
fn tooltip_max_width_moves_only_the_cap() {
    let _g = serial();
    let mut fails = Vec::new();
    for q in SCALES {
        set_scale(q);
        for text in TEXTS {
            let (t, same) = (Tooltip::new(text), Tooltip::new(text).max_width(TOOLTIP_MAX_W));
            let at = Rect { x: 24, y: 24, w: t.size().0, h: t.size().1 };
            let paint = |t: Tooltip| Canvas::new(at, 0).painted(|fb| t.draw(fb, at)).px;
            if t.size() != same.size() || paint(t) != paint(same) {
                fails.push(format!("{text:?} at {q}/4 differs when asked for the default"));
            }
        }
        let (wide, narrow) = (Tooltip::new(LONG).max_width(900), Tooltip::new(LONG).max_width(99));
        let whole = drawn(wide) == [LONG] && wide.size().0 < scale().px(900);
        let cut = drawn(narrow)[0].ends_with('…') && narrow.size().0 == scale().px(99);
        if !whole || !cut {
            fails.push(format!("at {q}/4: {:?} {:?}", drawn(wide), drawn(narrow)));
        }
    }
    set_scale(SCALES[0]);
    assert!(fails.is_empty(), "{} cases\n{}", fails.len(), fails.join("\n"));
}
