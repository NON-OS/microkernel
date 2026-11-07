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
//! A window hidden to the dock and shown again (app_skeleton
//! runner/hide_steps.rs): hidden only once the window manager took the
//! minimize, shown only once its layer is back, hidden again in the window
//! manager when it is not, and raised once its layer is in the scene.

use super::super::hide_steps::{
    hide, hover_after_hide, paints, show, waits_for_frame, HideSteps, ShowSteps,
};

#[derive(Default)]
struct Log {
    calls: Vec<&'static str>,
    wm_refuses: bool,
    compositor_refuses: bool,
}

impl HideSteps for Log {
    fn wm_minimize(&mut self) -> bool {
        self.calls.push("wm_minimize");
        !self.wm_refuses
    }

    fn scene_remove(&mut self) -> bool {
        self.calls.push("scene_remove");
        !self.compositor_refuses
    }
}

impl ShowSteps for Log {
    fn scene_submit(&mut self) -> bool {
        self.calls.push("scene_submit");
        !self.compositor_refuses
    }

    fn wm_raise(&mut self) {
        self.calls.push("wm_raise");
    }

    fn wm_minimize(&mut self) -> bool {
        HideSteps::wm_minimize(self)
    }

    fn repaint(&mut self) {
        self.calls.push("repaint");
    }
}

#[test]
fn a_minimize_the_window_manager_refuses_leaves_the_window_drawn() {
    let mut log = Log { wm_refuses: true, ..Log::default() };
    assert!(!hide(&mut log), "not minimized");
    assert_eq!(log.calls, ["wm_minimize"], "its layer kept, where the window manager has it");
}

#[test]
fn a_minimize_takes_the_layer_once_the_window_manager_took_it() {
    let mut log = Log::default();
    assert!(hide(&mut log));
    assert_eq!(log.calls, ["wm_minimize", "scene_remove"]);
    // A layer the compositor kept: minimized all the same, and the restore
    // submits it in its place.
    let mut log = Log { compositor_refuses: true, ..Log::default() };
    assert!(hide(&mut log));
}

#[test]
fn a_restore_raises_the_window_once_its_layer_is_back() {
    let mut log = Log::default();
    assert!(show(&mut log));
    assert_eq!(log.calls, ["scene_submit", "wm_raise", "repaint"]);
}

#[test]
fn a_restore_the_compositor_refuses_hides_the_window_again() {
    let mut log = Log { compositor_refuses: true, ..Log::default() };
    assert!(!show(&mut log), "still minimized, so the next dock click tries again");
    assert_eq!(log.calls, ["scene_submit", "wm_minimize"], "no press goes to a window not drawn");
}

/// The restore keeps the window on a display that shrank while it was on
/// the dock, as a move does; a layer past the edge is refused.
#[test]
fn a_restore_keeps_the_window_on_the_display() {
    const RESTORE: &str = include_str!("../../../../app_skeleton/src/runner/window_ops/restore.rs");
    assert!(RESTORE.contains("b.x.min(di.width.saturating_sub(w))"));
    assert!(RESTORE.contains("b.y.min(di.height.saturating_sub(h))"));
    assert!(RESTORE.contains("wm::window_move("), "the window manager told the kept place");
    const DRAIN: &str = include_str!("../../../../app_skeleton/src/runner/apply_drain.rs");
    assert!(DRAIN.contains("minimize(booted, peers, request_id);"));
    assert!(!DRAIN.contains("scene_remove"), "the layer goes only by hide_steps");
}

/// A window on the dock has no layer, so a present of it is refused. A
/// repaint an input asked for there marked its frame unshown, and the frame
/// loop, waiting for a shown frame before it read its inbox, never read the
/// dock's click that restores it: the window stayed on the dock for good.
#[test]
fn a_window_on_the_dock_is_not_painted_and_its_inbox_is_still_read() {
    assert!(!paints(true), "no repaint while on the dock");
    assert!(paints(false));
    assert!(!waits_for_frame(false, true), "an unshown frame on the dock holds nothing up");
    assert!(waits_for_frame(false, false), "a shown window is painted whole first");
    assert!(!waits_for_frame(true, false) && !waits_for_frame(true, true));
}

/// The frame loop asks both rules where it paints and where it waits.
#[test]
fn the_frame_loop_keeps_to_the_dock_rules() {
    const DRAIN: &str = include_str!("../../../../app_skeleton/src/runner/apply_drain.rs");
    assert!(DRAIN.contains("if result.repaint && paints(booted.minimized) {"));
    const PRIMED: &str = include_str!("../../../../app_skeleton/src/runner/ensure_primed.rs");
    assert!(PRIMED.contains("if !waits_for_frame(booted.primed, booted.minimized) {"));
    const FRAME: &str = include_str!("../../../../app_skeleton/src/runner/frame_loop.rs");
    assert!(
        FRAME.contains("booted.app.on_tick() && !booted.minimized"),
        "a tick paints nothing there"
    );
}

const RESTORE: &str = include_str!("../../../../app_skeleton/src/runner/window_ops/restore.rs");

#[test]
fn a_hidden_window_shows_no_control_lit_when_it_comes_back() {
    assert_eq!(hover_after_hide(true, "minimize", "none"), "none");
    assert_eq!(hover_after_hide(false, "minimize", "none"), "minimize", "a refused hide keeps it");
    let hide_at = RESTORE.find("let hidden = hide(").expect("minimize hides");
    let cleared = RESTORE
        .find(
            "booted.drag.hover = hover_after_hide(hidden, booted.drag.hover, DecorationHit::None);",
        )
        .expect("the hover is put out by the rule");
    let marked = RESTORE.find("booted.minimized = true;").expect("marked minimized");
    assert!(hide_at < cleared && cleared < marked, "after the hide answers, before it counts");
}

#[test]
fn a_restore_puts_the_hover_out_before_the_window_is_painted_again() {
    let restore = RESTORE.find("pub(in crate::runner) fn restore<A: App>(").expect("restore");
    let body = &RESTORE[restore..];
    let cleared = body.find("booted.drag.hover = DecorationHit::None;").expect("put out");
    let shown = body.find("if show(&mut Calls").expect("shown");
    assert!(cleared < shown, "before show, whose repaint draws the frame");
}
