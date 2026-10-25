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
//! frame to the asker's own child that draws and to no one else, and the
//! Terminal asks once after the load and reports the app as running.

use crate::shell_apps::focus_rule::{may_focus, DRAWS};

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
fn only_the_askers_own_child_that_draws_is_focused() {
    assert_eq!(DRAWS, 1 << 12, "GraphicsSurfaceCreate");
    assert!(may_focus(40, 40, 0x1819));
    assert!(!may_focus(40, 41, 0x1819), "another's child");
    assert!(!may_focus(40, 40, 0x159), "a program with no window");
    assert!(!may_focus(0, 0, 0x1819), "the kernel's pid is no asker");
}

#[test]
fn the_shell_wakes_an_app_only_where_the_rule_allows() {
    let code = flat(HANDLER);
    let arm = "Some((parent,caps))ifmay_focus(sender_pid,parent,caps)=>{boot(pid);0}";
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
