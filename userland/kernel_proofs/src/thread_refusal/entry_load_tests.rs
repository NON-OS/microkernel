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

//! The guarded store is the only one, and nothing of the thread is loaded
//! before it is asked (L77d): mark_running, mark_ready and mark_failed each
//! go through live_to, the one state store in entered.rs, and no entry path
//! loads a stack, a table, a context or the floating-point state above its
//! ask.

use super::entry_tests::{ENTERED, SW};

// Each with its call syntax, so the `use` lines above the ask never match.
const LOADS: [&str; 14] = [
    "set_kernel_stack(",
    "switch_to_process_address_space(",
    "set_cr3(",
    "CURRENT_PID.store(",
    "restore_fpu_state(",
    "init_fpu(",
    "restore_syscall_user_rsp(",
    "restore_user_context_iretq(",
    "return_to_usermode(",
    "ctx.restore()",
    "swap_address_space(",
    "prepare_incoming(",
    "enter_user(",
    "resume_user(",
];

pub(super) fn body(f: &str) -> String {
    let at = ENTERED.find(f).unwrap_or_else(|| panic!("entered: missing `{f}`"));
    let (head, _) = ENTERED[at..].split_once("\n    }").expect("entered: no body");
    head.lines().skip(1).map(str::trim).collect::<Vec<_>>().join(" ")
}

#[test]
fn the_guarded_store_is_the_only_one_and_nothing_loads_before_the_ask() {
    assert_eq!(body("fn mark_running("), "self.live_to(ProcessState::Running)");
    assert_eq!(body("fn mark_ready("), "let _ = self.live_to(ProcessState::Ready);");
    assert_eq!(body("fn mark_failed("), "let _ = self.live_to(ProcessState::Terminated(-1));");
    assert_eq!(ENTERED.matches("state.lock()").count(), 1, "one store, in live_to");
    for (what, text, _) in SW {
        let asked = text.find("if !pcb.mark_running() {").expect(what);
        for l in LOADS {
            assert!(!text[..asked].contains(l), "{what}: `{l}` before the ask");
        }
    }
}
