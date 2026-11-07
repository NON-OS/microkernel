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

//! Ctrl+Alt+T and Ctrl+Alt+Esc draw the shell's chrome again only when a
//! menu, the Launchpad or a lit menu-bar title was up: on a development
//! guest (TCG) every Ctrl+Alt+T composed the whole screen three times, from
//! a repaint that changed nothing, before the raise and the dock showed what
//! did change.

const CHORD: &str =
    include_str!("../../../capsule_desktop_shell/src/server/handlers/escape_chord.rs");

fn body<'a>(head: &str) -> &'a str {
    let at = CHORD.find(head).expect("the function");
    let end = CHORD[at..].find("\n}\n").expect("its end");
    &CHORD[at..at + end]
}

#[test]
fn the_terminal_and_the_process_manager_repaint_only_what_was_up() {
    for f in ["pub fn bring_terminal(", "fn bring(ctx"] {
        let b = body(f);
        assert!(b.contains("if close_chrome(ctx) {\n        repaint(ctx);\n    }"), "{f}");
        assert!(!b.contains("clear_chrome(ctx)"), "{f} draws nothing unasked");
    }
}

#[test]
fn close_chrome_counts_every_thing_it_takes_down() {
    let b = body("pub fn close_chrome(");
    for up in [
        "ctx.launchpad_ui.launchpad",
        "ctx.desk_ui.desktop_menu.is_some()",
        "ctx.menubar.open.is_some()",
        "ctx.menubar.hover.is_some()",
    ] {
        assert!(b.contains(up), "{up}");
    }
    for down in [
        "ctx.desk_ui.desktop_menu = None;",
        "ctx.desk_ui.menu_hover = None;",
        "ctx.desk_ui.menu_target = None;",
        "ctx.menubar.open = None;",
        "ctx.menubar.hover = None;",
    ] {
        assert!(b.contains(down), "{down}");
    }
    assert!(!b.contains("repaint("), "the caller says whether to draw");
}

#[test]
fn clear_chrome_still_draws_for_the_callers_that_show_their_own_state_by_it() {
    let b = body("pub fn clear_chrome(");
    assert!(b.contains("close_chrome(ctx);\n    repaint(ctx);"), "power off and the switcher");
}

const LOCK: &str = include_str!("../../../capsule_desktop_shell/src/server/lock/mod.rs");
const PROMPT: &str = include_str!("../../../capsule_desktop_shell/src/server/lock/prompt.rs");
const INPUT_MENU: &str =
    include_str!("../../../capsule_desktop_shell/src/server/handlers/input_menu.rs");
const ATTENTION: &str =
    include_str!("../../../capsule_desktop_shell/src/server/handlers/attention.rs");
const CHECK: &str = include_str!("../../../capsule_desktop_shell/src/server/lock/check.rs");

/// The callers that repaint whole right after take the chrome down without
/// a repaint of their own: locking went from the desktop to the curtain
/// through a whole frame of the desktop with its menus closed, eight
/// whole-screen composes on a TCG guest where one frame was wanted.
#[test]
fn a_whole_repaint_that_follows_is_the_only_one() {
    assert_eq!(LOCK.matches("escape_chord::close_chrome(ctx);\n").count(), 2, "lock now, idle");
    assert!(!LOCK.contains("clear_chrome"));
    let cover = LOCK.split("pub(crate) fn cover(").nth(1).expect("cover");
    assert!(cover.contains("crate::server::repaint::repaint(ctx);"), "the curtain, whole");
    assert!(PROMPT.contains("close_chrome(ctx);"));
    assert!(!PROMPT.contains("clear_chrome"));
    assert!(INPUT_MENU.contains("    close_chrome(ctx);\n    repaint(ctx);"));
    assert!(ATTENTION.contains("        close_chrome(ctx);\n        ctx.trusted.attention"));
    let attention = ATTENTION.split("if is_secure_attention(").nth(1).expect("the chord");
    assert!(attention
        .split("return Some(true);")
        .next()
        .expect("its arm")
        .contains("repaint(ctx);"));
}

/// A passphrase checked shows "Checking" and, still locked, the note under
/// the field in the band; only the unlock is drawn whole.
#[test]
fn a_check_and_a_wrong_answer_show_the_band_and_the_unlock_the_screen() {
    let submit = CHECK.split("fn submit(").nth(1).expect("submit");
    let submit = submit.split("\n}\n").next().expect("its body");
    assert_eq!(submit.matches("super::band::show(ctx);").count(), 2);
    assert!(!submit.contains("repaint::repaint(ctx)"));
    assert!(submit.contains("if !ctx.lock.screen.covers() {\n        release(ctx);"));
    let release = CHECK.split("fn release(").nth(1).expect("release");
    assert!(release.contains("crate::server::repaint::repaint(ctx);"), "the unlock, whole");
}
