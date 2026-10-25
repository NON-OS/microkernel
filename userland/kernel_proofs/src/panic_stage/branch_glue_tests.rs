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

//! zerostate's `terminate` and the exception handlers' user branches, read
//! from the source: `terminate` parks once the panic path is taken, before
//! its own once-guard, and a fault from user mode still ends only that
//! process, never the machine.

use std::path::Path;

const TERMINATE: &str = include_str!("../../../../src/security/zerostate/terminate.rs");
const END: &str = include_str!("../../../../src/boot/panic/end.rs");

#[test]
fn terminate_reads_the_panic_stage_before_its_once_guard() {
    assert!(TERMINATE.contains(
        "if crate::boot::panic::panic_stage_taken() || !first(&ENDING) {\n        park()"
    ));
    assert!(END.contains("super::stage::taken(STAGE.load(Ordering::Acquire))"));
}

#[test]
fn a_user_fault_ends_its_process_before_any_fatal_exit() {
    let dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../src/interrupts/handlers/exceptions");
    let mut seen = 0;
    let mut stack = vec![dir];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(d).unwrap().flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
                continue;
            }
            let text = std::fs::read_to_string(&p).unwrap();
            let Some(fatal) = text.find("crate::boot::panic::fatal();") else { continue };
            let name = p.file_name().unwrap().to_string_lossy().into_owned();
            // A double fault, a machine check and a bad TSS are the machine's,
            // from any mode: they have no user branch.
            if ["double_fault.rs", "machine_check.rs", "tss.rs"].contains(&name.as_str()) {
                continue;
            }
            let user =
                text.find("exit_and_yield(").unwrap_or_else(|| panic!("{name}: no user branch"));
            seen += 1;
            // The user branch's exit is written before the kernel's fatal exit,
            // and the user branch is chosen by the frame's privilege level.
            assert!(user < fatal, "{name}");
            assert!(text.contains("is_user_mode()"), "{name}");
        }
    }
    assert_eq!(seen, 14, "the handlers with both branches: {seen}");
}
