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
//! The names apart, and every writer, reader, spawn and teardown on them.

use super::own_names::{requests, stdin, stdout};
use super::sender_pid::from_envelope;

macro_rules! kernel {
    ($p:literal) => {
        include_str!(concat!("../../../../src/", $p))
    };
}

/// The body of the function `sig` opens in `text`, its braces alone.
fn body<'a>(text: &'a str, sig: &str) -> &'a str {
    let from = &text[text.find(sig).expect(sig)..];
    let from = &from[from.find('{').unwrap()..];
    &from[..from.find("\n}\n").unwrap()]
}

#[test]
fn the_three_inboxes_of_a_pid_are_three_and_no_other_pids() {
    for pid in (0..2_000u32).chain([u32::MAX - 1, u32::MAX]) {
        let (r, i, o) = (requests(pid), stdin(pid), stdout(pid));
        assert!(r != i && i != o && r != o, "pid {pid}");
        assert_eq!(r, format!("proc.{pid}"), "requests stay where senders address them");
        assert_ne!(o, requests(pid.wrapping_add(1)));
        // A line, were it ever routed, would claim no capsule as its sender.
        assert_eq!(from_envelope(&o), 0);
    }
}

#[test]
fn every_writer_and_the_drain_name_stdout_and_never_proc() {
    for (file, text) in [
        ("debug/proc_mirror.rs", kernel!("syscall/microkernel/debug/proc_mirror.rs")),
        ("private_write.rs", kernel!("syscall/microkernel/private_write.rs")),
        ("proc_output.rs", kernel!("syscall/microkernel/proc_output.rs")),
    ] {
        assert!(text.contains("own_names::stdout("), "{file} names stdout");
        assert!(!text.contains("format!(\"proc."), "{file} builds no proc.<pid>");
        assert!(!text.contains("own_names::requests("), "{file} names no request inbox");
    }
}

#[test]
fn a_capsule_is_spawned_with_stdout_and_its_teardown_keeps_only_that() {
    let spawn = kernel!("kernel_core/process_spawn/capsule_spawn/runner/install/own_inboxes.rs");
    for name in ["requests(pid)", "stdout(pid)", "stdin(pid)"] {
        assert!(spawn.contains(&format!("own_names::{name}")), "registered: {name}");
    }
    let drop = kernel!("ipc/nonos_inbox/drop_pid.rs");
    let all = body(drop, "fn unregister_for_pid");
    assert!(
        all.contains("own_names::stdout(pid)") && all.contains("unregister_all_but_stdout(pid)")
    );
    let kept = body(drop, "fn unregister_all_but_stdout");
    assert!(kept.contains("own_names::stdin(pid)") && kept.contains("own_names::requests(pid)"));
    assert!(!kept.contains("stdout"), "the output stays for the parent");
    let finalize = kernel!("process/exit/finalize.rs");
    assert!(finalize.contains("unregister_all_but_stdout(pid)"), "a retained exit keeps stdout");
}
