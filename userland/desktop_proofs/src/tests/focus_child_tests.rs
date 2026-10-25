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

//! A GUI app the Terminal loads opens its window: the shell gives the focus
//! frame to the asker's own child and to no one else, and the Terminal asks,
//! once after the load and only for a program that draws, and reports it.

use crate::shell_apps::focus_rule::may_focus;

const HANDLER: &str =
    include_str!("../../../capsule_desktop_shell/src/server/handlers/focus_child.rs");
const DISPATCH: &str = include_str!("../../../capsule_desktop_shell/src/server/dispatch.rs");
const DRAIN: &str =
    include_str!("../../../capsule_terminal/src/command/builtin/nox/install/job_drain.rs");
const ASK: &str =
    include_str!("../../../capsule_terminal/src/command/builtin/nox/install/window.rs");

fn flat(s: &str) -> String {
    s.split_whitespace().collect()
}

#[test]
fn only_the_askers_own_child_is_focused() {
    assert!(may_focus(40, 40));
    assert!(!may_focus(40, 41), "another's child");
    assert!(!may_focus(0, 0), "the kernel's pid is no asker");
}

#[test]
fn the_terminal_asks_only_for_a_program_that_draws() {
    let code = flat(ASK);
    assert!(code.contains("constDRAWS:u64=1<<12;"), "GraphicsSurfaceCreate");
    let draws = "letdraws=table().iter().any(|e|e.pid==pid&&e.caps&DRAWS!=0);";
    let at = code.find(draws).expect("read from the process table");
    assert!(at < code.find("call_status(").unwrap(), "before it asks");
    assert!(code.contains(".filter(|_|draws)"));
}

#[test]
fn the_shell_wakes_an_app_only_where_the_rule_allows() {
    let code = flat(HANDLER);
    let arm = "Some(parent)ifmay_focus(sender_pid,parent)=>{boot(pid);0}";
    assert!(code.contains(arm));
    assert_eq!(code.matches("boot(").count(), 1, "no other wake");
    assert!(flat(DISPATCH).contains("OP_FOCUS_CHILD=>handlers::focus_child::handle("));
}

#[test]
fn the_terminal_asks_once_and_a_window_is_success() {
    let code = flat(DRAIN);
    let ask =
        "if!core::mem::replace(&mutself.asked_window,true)&&super::super::window::open(self.pid)";
    let at = code.find(ask).expect("asked once, after the load");
    let done = code[at..].find("returnJobProgress::Done(0);").expect("success");
    assert!(done < code[at..].find("mk_proc_output").unwrap_or(usize::MAX));
    assert!(flat(ASK).contains("constOP_FOCUS_CHILD:u16=0x000F;"));
}
