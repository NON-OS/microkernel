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

//! A change of the lit frame control presents the controls put out and lit,
//! never the window whole (app_skeleton runner/hover_part.rs): on a
//! maximized window each control the pointer crossed composed the whole
//! window and, folded with the menu bar, the whole 1920x1080 screen.

use nonos_app_skeleton::runner::hover_part::{hover_part, union, Painted};
use nonos_toolkit::decorations::{control_rect_at, DecorationHit as Hit, FrameState, CONTROLS};

const ONE: u32 = 4;
/// A maximized window on a 1920x1080 screen under its menu bar.
const MAX: (u32, u32) = (1920, 1022);

fn painted(hover: Hit, maximized: bool) -> Painted {
    (MAX.0, MAX.1, FrameState { hover, focused: true }, maximized)
}

fn control(hit: Hit) -> nonos_toolkit::decorations::Rect {
    let i = CONTROLS.iter().position(|c| *c == hit).expect("a control") as u32;
    control_rect_at(i, MAX.0, MAX.1, true, ONE)
}

#[test]
fn lighting_a_control_presents_that_control_alone() {
    let part = hover_part(painted(Hit::None, true), painted(Hit::MaximizeButton, true), ONE);
    assert_eq!(part, Some(Some(control(Hit::MaximizeButton))));
}

#[test]
fn moving_between_controls_presents_the_box_round_both() {
    let was = painted(Hit::MaximizeButton, true);
    let part = hover_part(was, painted(Hit::CloseButton, true), ONE).flatten().expect("a part");
    assert_eq!(part, union(control(Hit::MaximizeButton), control(Hit::CloseButton)));
    let window = u64::from(MAX.0) * u64::from(MAX.1);
    let area = u64::from(part.w) * u64::from(part.h);
    assert!(area * 100 < window, "{area} px of the window's {window}");
}

#[test]
fn putting_a_control_out_presents_it_and_the_title_bar_changes_nothing() {
    let out = hover_part(painted(Hit::MinimizeButton, true), painted(Hit::Titlebar, true), ONE);
    assert_eq!(out, Some(Some(control(Hit::MinimizeButton))));
    let bar = hover_part(painted(Hit::Titlebar, true), painted(Hit::None, true), ONE);
    assert_eq!(bar, Some(None), "neither is a control: nothing to present");
}

#[test]
fn anything_besides_the_hover_draws_the_window_as_before() {
    let was = painted(Hit::None, true);
    assert_eq!(hover_part(was, was, ONE), None, "nothing changed");
    assert_eq!(hover_part(was, painted(Hit::CloseButton, false), ONE), None, "restored");
    let (w, h, s, m) = painted(Hit::CloseButton, true);
    assert_eq!(hover_part(was, (w - 10, h, s, m), ONE), None, "resized");
    let blurred = FrameState { focused: false, ..s };
    assert_eq!(hover_part(was, (w, h, blurred, m), ONE), None, "focus lost");
}

#[test]
fn a_hover_drain_takes_the_hover_path_and_any_other_ask_repaints() {
    const INPUT: &str = include_str!("../../../../app_skeleton/src/runner/inbox/input.rs");
    assert!(INPUT.contains("PointerAction::HoverChanged => out.hover = true,"));
    const APPLY: &str = include_str!("../../../../app_skeleton/src/runner/apply_drain.rs");
    assert!(APPLY.contains(
        "    if result.repaint && paints(booted.minimized) {\n        repaint(booted, peers, request_id);\n    } else if result.hover && paints(booted.minimized) {\n        repaint_hover(booted, peers, request_id);"
    ));
    const HOVER: &str = include_str!("../../../../app_skeleton/src/runner/paint_hover.rs");
    assert!(HOVER.contains("draw_whole(&mut booted.app"), "the buffer holds the whole picture");
    assert!(HOVER.contains("let d = Damage { x, y, width: w, height: h };"), "the part presented");
}
