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

//! A kernel-mode fault may come while its CPU holds the heap's, the logger's
//! or the port's lock. So between the fatal line and the panic path the
//! kernel branch of each handler allocates nothing and logs nothing.

const EXC: &str = "../../../../src/interrupts/handlers/exceptions/";
const GPF: &str = include_str!("../../../../src/interrupts/handlers/exceptions/gpf.rs");
const UD: &str = include_str!("../../../../src/interrupts/handlers/exceptions/opcode.rs");
const PF: &str = include_str!("../../../../src/interrupts/handlers/exceptions/page_fault.rs");
const DF: &str = include_str!("../../../../src/interrupts/handlers/exceptions/double_fault.rs");

/// The text from `from` up to the first `to` after it.
fn between<'a>(text: &'a str, from: &str, to: &str) -> &'a str {
    let start = text.find(from).unwrap_or_else(|| panic!("{EXC}: no {from}"));
    let rest = &text[start..];
    &rest[..rest.find(to).unwrap_or_else(|| panic!("{EXC}: no {to} after {from}"))]
}

fn quiet(name: &str, body: &str) {
    for loud in ["log_", "format!", "alloc::", "println!", "with_serial_lock"] {
        assert!(!body.contains(loud), "{name}: {loud} before the panic path");
    }
}

#[test]
fn gpf_and_ud_go_from_the_notice_straight_to_the_panic_path() {
    for (name, src) in [("gpf", GPF), ("opcode", UD)] {
        let kernel = between(src, "if !ctx.is_user_mode() {", "crate::boot::panic::fatal();");
        assert!(kernel.contains("emit_fatal_notice("), "{name}: the notice comes first");
        quiet(name, kernel);
    }
}

#[test]
fn a_kernel_page_fault_logs_nothing_on_its_way_to_the_panic_path() {
    let handle = between(PF, "pub fn handle(", "\n}\n");
    assert!(handle.contains("if exception.is_user_mode() {\n        log_page_fault(&ctx);"));
    assert_eq!(handle.matches("log_page_fault(").count(), 1, "the user branch's only");
    let guard = between(PF, "check_guard_page_access(", "return false;");
    assert!(guard.contains(".end_fatal();"));
    quiet("page_fault guard", guard);
    let panic = between(PF, "fn kernel_panic(", "crate::boot::panic::fatal();");
    assert!(panic.contains(".end_fatal();"));
    quiet("page_fault", panic);
}

#[test]
fn a_double_fault_says_one_fatal_line_and_stops() {
    let handle = between(DF, "pub fn handle(", "crate::boot::panic::fatal();");
    assert!(handle.contains("emit_fatal_notice_nolock("));
    assert!(handle.contains("l.end_fatal();"));
    quiet("double_fault", handle);
}
