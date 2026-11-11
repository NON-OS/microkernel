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

//! Button, on look: a selected plain or quiet toggle draws a raised body
//! inked in the accent, or `accent_text` where the accent does not read on
//! it; disabled takes over, and hover and press still move the body.

use nonos_toolkit::theme::derive::mix;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{Kind, State};

use super::button_drawn::{body, each_theme, inked};
use super::serial::serial;
use crate::wcag::{ratio, BODY};

const TOGGLES: [Kind; 2] = [Kind::Plain, Kind::Quiet];

const ON: State =
    State { selected: true, hovered: false, focused: false, pressed: false, disabled: false };

#[test]
fn button_on_inks_in_the_accent_where_it_reads() {
    let _g = serial();
    each_theme(|| {
        let r = roles();
        let (fill, wash) = (r.raised.argb(), r.hover_wash.argb());
        let grounds = [fill, mix(fill, wash, (wash >> 24) as u8)];
        let reads = grounds.iter().all(|&g| ratio(r.accent.argb(), g) >= BODY);
        let want = if reads { r.accent } else { r.accent_text };
        let bad = TOGGLES.into_iter().find(|&k| !inked(k, ON, want));
        bad.map(|k| format!("{k:?} on is not in {want:?} (accent reads: {reads})"))
    });
}

#[test]
fn button_on_disabled_takes_the_disabled_look() {
    let _g = serial();
    let dead = State { disabled: true, ..ON };
    each_theme(|| {
        let ink = roles().disabled;
        let bad = TOGGLES.into_iter().find(|&k| !inked(k, dead, ink));
        bad.map(|k| format!("{k:?} on and disabled is not in disabled ink"))
    });
}

#[test]
fn button_on_hover_and_press_move_the_body() {
    let _g = serial();
    each_theme(|| {
        let bad = TOGGLES.into_iter().find_map(|k| {
            let (a, h) = (body(k, ON), body(k, State { hovered: true, ..ON }));
            let p = body(k, State { pressed: true, ..ON });
            let washes = roles().hover_wash.argb() >> 24 != 0;
            (a == h && washes || a == p).then(|| format!("{k:?} {a:08x} {h:08x} {p:08x}"))
        });
        bad.or_else(|| {
            let (on, off) = (body(Kind::Quiet, ON), body(Kind::Quiet, State::default()));
            (on == off).then(|| format!("on body {on:08x} is the off body"))
        })
    });
}
