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

//! `ps` in a session: the table the host hands over, in the terminal's
//! layout, no process 0, no control byte from a name, and a plain refusal
//! when the host has no table.

use crate::kit::{Lines, Script};
use nonos_shell::builtin::ps::{HEADER, UNAVAILABLE};
use nonos_shell::host::Proc;
use nonos_shell::session::Session;

fn proc(pid: u32, ppid: u32, state: u8, name: &[u8]) -> Proc {
    Proc { pid, ppid, state, name: name.to_vec() }
}

fn run(host: &mut Script, line: &str) -> Vec<String> {
    let mut out = Lines::default();
    Session::default().run_line(line.as_bytes(), &mut out, host);
    out.text()
}

fn machine() -> Script {
    let procs = vec![
        proc(0, 0, 2, b"kernel"),
        proc(1, 0, 3, b"init"),
        proc(14, 1, 2, b"nsh.front"),
        proc(15, 1, 1, b"nsh.attest"),
        proc(120345, 14, 9, b"long-gone"),
    ];
    Script { procs: Some(procs), ..Script::default() }
}

#[test]
fn ps_prints_the_table_as_the_terminal_does() {
    assert_eq!(
        run(&mut machine(), "ps; echo $?"),
        [
            std::str::from_utf8(HEADER).unwrap(),
            "     1     0  idle      init",
            "    14     1  running   nsh.front",
            "    15     1  ready     nsh.attest",
            "120345    14  exited    long-gone",
            "0",
        ]
    );
}

#[test]
fn ps_feeds_a_filter() {
    assert_eq!(run(&mut machine(), "ps | grep nsh | wc -l"), ["lines 2"]);
}

#[test]
fn a_name_reaches_the_client_as_printable_bytes_only() {
    let mut host = Script {
        procs: Some(vec![proc(7, 1, 2, b"\x1b]0;owned\x07x\x7f\xc3\xa9")]),
        ..Script::default()
    };
    let got = run(&mut host, "ps");
    assert_eq!(got[1], "     7     1  running   ?]0;owned?x???");
    let mut out = Lines::default();
    Session::default().run_line(b"ps", &mut out, &mut host);
    assert!(out.0.iter().flatten().all(|&c| (0x20..0x7f).contains(&c)));
}

#[test]
fn no_table_is_said_and_fails() {
    let unavailable = std::str::from_utf8(UNAVAILABLE).unwrap();
    assert_eq!(run(&mut Script::default(), "ps; echo $?"), [unavailable, "1"]);
    let mut empty = Script { procs: Some(Vec::new()), ..Script::default() };
    assert_eq!(run(&mut empty, "ps; echo $?"), [unavailable, "1"]);
}

#[test]
fn ps_takes_no_argument() {
    assert_eq!(run(&mut machine(), "ps -ef; echo $?"), ["usage: ps", "1"]);
}

#[test]
fn ps_is_listed_and_built_in() {
    let mut host = machine();
    assert!(run(&mut host, "help")[0].split(' ').any(|n| n == "ps"));
    assert_eq!(run(&mut host, "type ps"), ["ps is built into the shell"]);
}
