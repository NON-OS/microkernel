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


//! Where the terminal opens and where its prompt sits: centred on the
//! 1920x1080 screen with room for a real session, whole on the small modes,
//! and the prompt under the last line of output rather than at the foot.

use demo_screen_proofs::layout::{follow, Layout, Rect};
use demo_screen_proofs::term::Terminal;
use nonos_app_skeleton::runner::chrome::{dock_for, menubar_for};
use nonos_app_skeleton::runner::fit_rule::fit_manifest;
use nonos_app_skeleton::App;

#[test]
fn it_opens_centred_and_roomy_on_a_1080p_screen() {
    let (w, h) = (1920, 1080);
    let at = fit_manifest(Terminal::new().manifest(), w, h);
    let box_ = (at.initial_x, at.initial_y, at.width, at.height);
    let right = w - (at.initial_x + at.width);
    let top = at.initial_y - menubar_for(w, h);
    let bottom = h - dock_for(w, h) - (at.initial_y + at.height);
    assert!(at.initial_x.abs_diff(right) <= 2, "{box_:?}: not centred across");
    assert!(top.abs_diff(bottom) <= 2, "{box_:?}: not centred between bar and dock");
    assert!(at.width * 100 >= w * 60, "{}: under 60% of the width", at.width);
    assert!(at.height >= 720, "{}: too short for 30 rows", at.height);
}

#[test]
fn it_is_centred_where_the_fit_rule_has_to_shrink_it() {
    for (w, h) in [(1024, 768), (1280, 720)] {
        let at = fit_manifest(Terminal::new().manifest(), w, h);
        let right = w - (at.initial_x + at.width);
        assert!(at.initial_x.abs_diff(right) <= 2, "{w}x{h} at {}", at.initial_x);
    }
}

fn laid(body_h: u32) -> Layout {
    let r = |y, h| Rect { x: 0, y, w: 800, h };
    Layout { left_rail: r(40, 0), body: r(40, body_h), input: r(40 + body_h, 20), footer: r(700, 20) }
}

#[test]
fn the_prompt_follows_short_output() {
    let l = follow(laid(600), 3, 20);
    assert_eq!((l.body.h, l.input.y), (60, 100), "the row under the third line");
    let empty = follow(laid(600), 0, 20);
    assert_eq!(empty.input.y, 40, "nothing printed: the prompt at the top");
}

#[test]
fn a_full_body_keeps_the_prompt_at_the_foot() {
    for used in [30, 31, 500] {
        let l = follow(laid(600), used, 20);
        assert_eq!((l.body.h, l.input.y), (600, 640), "{used} rows");
    }
    assert_eq!(follow(laid(600), 29, 20).input.y, 620, "one row short");
}
