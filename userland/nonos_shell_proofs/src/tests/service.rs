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

//! `service` in a session: each name's port and pid from the registry, a
//! name the registry does not hold said and failed, a name that is not a
//! service name refused before the host is asked, and a plain refusal when
//! the host cannot read the registry.

use crate::kit::{Lines, Script};
use nonos_shell::builtin::service::{BAD_NAME, NAME_MAX, UNANSWERED, USAGE};
use nonos_shell::session::Session;
use std::collections::HashMap;

fn text(b: &[u8]) -> &str {
    std::str::from_utf8(b).unwrap()
}

fn run(host: &mut Script, line: &[u8]) -> Vec<String> {
    let mut out = Lines::default();
    Session::default().run_line(line, &mut out, host);
    out.text()
}

fn machine() -> Script {
    let mut reg = HashMap::new();
    reg.insert(b"nsh.attest".to_vec(), (4486, 15));
    reg.insert(b"net.sockets".to_vec(), (4300, 9));
    Script { services: Some(reg), ..Script::default() }
}

#[test]
fn service_prints_each_port_and_pid() {
    assert_eq!(
        run(&mut machine(), b"service nsh.attest net.sockets; echo $?"),
        ["nsh.attest  port=4486  pid=15", "net.sockets  port=4300  pid=9", "0"]
    );
}

#[test]
fn svc_is_the_same_command() {
    assert_eq!(run(&mut machine(), b"svc nsh.attest"), ["nsh.attest  port=4486  pid=15"]);
}

#[test]
fn a_name_not_registered_is_said_and_fails() {
    assert_eq!(
        run(&mut machine(), b"service nsh.attest no.such; echo $?"),
        ["nsh.attest  port=4486  pid=15", "no.such: not registered", "1"]
    );
}

#[test]
fn no_name_is_usage() {
    assert_eq!(run(&mut machine(), b"service; echo $?"), [text(USAGE), "1"]);
}

#[test]
fn a_name_the_registry_cannot_hold_is_refused_before_the_host_is_asked() {
    let long = "a".repeat(NAME_MAX + 1);
    for bad in [long.as_bytes(), b"\x1b[2J\x07", b"caf\xc3\xa9", b"del\x7f"] {
        let mut host = machine();
        let line = [b"service nsh.attest ".as_slice(), bad].concat();
        assert_eq!(run(&mut host, &line), [text(BAD_NAME)]);
        assert!(host.asked.is_empty());
    }
    let mut host = machine();
    let longest = "a".repeat(NAME_MAX);
    assert_eq!(run(&mut host, format!("service {longest}").as_bytes()).len(), 1);
    assert_eq!(host.asked, [longest.into_bytes()]);
}

#[test]
fn a_host_without_the_registry_says_so() {
    let mut host = Script::default();
    assert_eq!(run(&mut host, b"service nsh.attest; echo $?"), [text(UNANSWERED), "1"]);
}

#[test]
fn service_feeds_a_filter() {
    assert_eq!(
        run(&mut machine(), b"service nsh.attest net.sockets | grep sockets"),
        ["net.sockets  port=4300  pid=9"]
    );
}
