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

//! Button: the tonal body is its own and follows hover, press and disabled.

use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{Button, Kind, State};

use super::boxes::boxes;
use super::canvas::Canvas;
use super::serial::{per_theme, serial};
use super::widget::Widget;

fn drawn(kind: Kind, label: &'static str, st: State) -> Canvas {
    let b = Button::new(label).kind(kind).state(st);
    let at = boxes(b.pref())[0];
    Canvas::new(at, roles().bg.argb()).painted(|fb| b.draw(fb, at))
}

fn body(kind: Kind, st: State) -> u32 {
    let at = boxes(Button::new("").pref())[0];
    drawn(kind, "", st).get(at.x + at.w / 2, at.y + at.h / 2)
}

fn each_theme(mut bad: impl FnMut() -> Option<String>) {
    let mut fails = Vec::new();
    per_theme(|theme| fails.extend(bad().map(|e| format!("{theme}: {e}"))));
    assert!(fails.is_empty(), "{} cases\n{}", fails.len(), fails.join("\n"));
}

#[test]
fn tonal_fill_differs_from_secondary_and_primary() {
    let _g = serial();
    each_theme(|| {
        let at = |k| body(k, State::default());
        let (t, s, p) = (at(Kind::Tonal), at(Kind::Secondary), at(Kind::Primary));
        (t == s || t == p).then(|| format!("tonal {t:08x} secondary {s:08x} primary {p:08x}"))
    });
}

#[test]
fn disabled_tonal_takes_the_disabled_look() {
    let _g = serial();
    let dead = State { disabled: true, ..State::default() };
    each_theme(|| {
        let same = drawn(Kind::Tonal, "Save", dead).px == drawn(Kind::Secondary, "Save", dead).px;
        (!same).then(|| "disabled tonal differs from disabled secondary".to_string())
    });
}

#[test]
fn tonal_hover_and_press_move_the_fill() {
    let _g = serial();
    let rest = State::default();
    each_theme(|| {
        let at = |s| body(Kind::Tonal, s);
        let (a, h) = (at(rest), at(State { hovered: true, ..rest }));
        let p = at(State { pressed: true, ..rest });
        let washes = roles().hover_wash.argb() >> 24 != 0;
        (a == h && washes || a == p).then(|| format!("rest {a:08x} hover {h:08x} press {p:08x}"))
    });
}
