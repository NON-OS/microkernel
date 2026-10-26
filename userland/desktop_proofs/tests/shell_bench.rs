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
//! A line too long for its width is cut with an ellipsis on a character
//! The shell's start on the serial line (capsule_desktop_shell bench.rs and
//! bench_line.rs): its steps in the kernel's own `[BENCH] <ms> <what>` form,
//! and the first requests it answers with how long after it began. No line
//! names a process.

#[path = "../../capsule_desktop_shell/src/bench_line.rs"]
mod bench_line;

use bench_line::{answered_line, mark_line};

const BENCH: &str = include_str!("../../capsule_desktop_shell/src/bench.rs");

fn text(f: impl FnOnce(&mut [u8; 96]) -> usize) -> String {
    let mut out = [0u8; 96];
    let n = f(&mut out);
    String::from_utf8(out[..n].to_vec()).unwrap()
}

#[test]
fn shell_bench_marks_read_as_the_kernels() {
    assert_eq!(text(|o| mark_line(122095, b"serving", o)), "[BENCH] 122095 shell_serving\n");
    assert_eq!(text(|o| mark_line(0, b"start", o)), "[BENCH] 0 shell_start\n");
}

#[test]
fn shell_bench_answers_say_the_op_and_the_wait() {
    let line = text(|o| answered_line(0x0007, 133000, 10905, o));
    assert_eq!(line, "[SHELL] answered op 0x0007 at 133000 ms, 10905 ms after the shell began\n");
    assert!(text(|o| answered_line(0xBEEF, 1, 1, o)).contains("op 0xbeef"));
}

#[test]
fn shell_bench_names_the_first_four_and_no_process() {
    assert!(BENCH.contains("const NAMED: u32 = 4;"));
    assert!(BENCH.contains("if ANSWERED.fetch_add(1, Relaxed) >= NAMED {"));
    assert!(!BENCH.contains("pid") && !BENCH.contains("sender"), "no process named");
}
