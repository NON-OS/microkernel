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

//! Every ratio identity.md's colour tables give, from the role values in
//! design::roles, so the page and the code cannot part.

use crate::design::roles::{Roles, HIGH_CONTRAST, INK, PAPER};
use crate::wcag::{ratio, BODY, LARGE};

/// `fg` on each of `grounds` at `min` or more.
fn on(name: &str, fg: u32, grounds: &[u32], min: f64) {
    for g in grounds {
        let r = ratio(fg, *g);
        assert!(r >= min, "{name}: {fg:08X} on {g:08X} is {r:.2}:1, under {min}");
    }
}

/// The floors for a scheme: body text and the semantic colours at `text`,
/// metadata at `quiet`, the accent as text on ground and surface.
fn holds(r: &Roles, text: f64, quiet: f64, edge: f64) {
    let all = [r.ground, r.surface, r.raised];
    on("text", r.text, &all, text);
    on("text.2", r.text2, &all, text);
    on("text.3", r.text3, &all, quiet);
    on("edge", r.edge, &all, edge);
    on("accent", r.accent, &[r.ground, r.surface], quiet);
    on("on.accent", r.on_accent, &[r.accent], quiet);
    for (n, c) in [("ok", r.ok), ("warn", r.warn), ("bad", r.bad)] {
        on(n, c, &all, quiet);
    }
    on("on.bad", r.on_bad, &[r.bad], quiet);
    on("text on select", r.text, &[r.select], text);
    on("text.2 on select", r.text2, &[r.select], quiet);
}

#[test]
fn ink_and_paper_hold_their_tables() {
    holds(&INK, 7.0, BODY, LARGE);
    holds(&PAPER, 7.0, BODY, LARGE);
}

#[test]
fn high_contrast_holds_seven_to_one_everywhere() {
    holds(&HIGH_CONTRAST, 7.0, 7.0, 7.0);
}

#[test]
fn a_rule_is_decoration_and_the_brand_cyan_is_never_paper_text() {
    for r in [INK, PAPER] {
        assert!(ratio(r.rule, r.ground) < LARGE, "a rule divides; it is not an edge");
    }
    assert!(ratio(0xFF66_FFFF, PAPER.ground) < LARGE && PAPER.accent != 0xFF66_FFFF);
}
