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
//! `history` numbers the lines a session has run, held to the session's
//! bounds; `clear` sends the terminal at the other end home and erased.

use crate::kit::{Lines, Script};
use nonos_shell::session::{Session, CLEAR, MAX_HISTORY, MAX_HISTORY_LINE};

fn run(s: &mut Session, line: &str) -> Vec<String> {
    let mut out = Lines::default();
    s.run_line(line.as_bytes(), &mut out, &mut Script::default());
    out.text()
}

#[test]
fn history_numbers_each_line_run_including_its_own() {
    let mut s = Session::default();
    run(&mut s, "echo a");
    run(&mut s, "   ");
    run(&mut s, "nope; echo b");
    assert_eq!(run(&mut s, "history"), ["0: echo a", "1: nope; echo b", "2: history"]);
    assert_eq!(run(&mut s, "history x; echo $?"), ["usage: history", "1"]);
}

#[test]
fn history_keeps_the_newest_lines_and_the_head_of_each() {
    let mut s = Session::default();
    for i in 0..MAX_HISTORY + 10 {
        run(&mut s, &format!("echo {i}"));
    }
    assert_eq!(s.history.len(), MAX_HISTORY);
    assert_eq!(s.history.first().map(Vec::as_slice), Some(&b"echo 10"[..]));
    let long = format!("echo {}", "x".repeat(MAX_HISTORY_LINE));
    run(&mut s, &long);
    assert_eq!(s.history.len(), MAX_HISTORY);
    assert_eq!(s.history.last().map(Vec::as_slice), Some(&long.as_bytes()[..MAX_HISTORY_LINE]));
    let out = run(&mut s, "history");
    assert_eq!(out.len(), MAX_HISTORY);
    assert_eq!(out.last().map(String::as_str), Some("63: history"));
}

#[test]
fn clear_sends_home_and_erase() {
    let mut s = Session::default();
    let mut out = Lines::default();
    s.run_line(b"clear; echo $?", &mut out, &mut Script::default());
    assert_eq!(out.0, [CLEAR.to_vec(), b"0".to_vec()]);
}
