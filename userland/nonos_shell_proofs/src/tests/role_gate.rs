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

//! A session runs inside nsh.front, so the shell holds a login to its
//! role's capabilities itself: `service` needs IPC and `ping` and
//! `nslookup` need Network in the role, whatever the host holds, and a
//! refused command asks the host nothing. A session nothing admitted, the
//! GUI terminal's, runs under its host's own grant.

use crate::kit::{Lines, Script};
use nonos_shell::host::{Echo, Resolve};
use nonos_shell::session::{Session, Who};
use std::collections::HashMap;

fn login(caps: &[&'static str]) -> Session {
    let mut s = Session::default();
    s.who = Some(Who {
        user: b"alice".to_vec(),
        role: b"operator".to_vec(),
        subject: [0; 64],
        caps: caps.to_vec(),
    });
    s
}

fn host() -> Script {
    let mut reg = HashMap::new();
    reg.insert(b"nsh.attest".to_vec(), (4486, 15));
    Script {
        services: Some(reg),
        resolve: Some(Resolve::Addr([10, 0, 2, 2])),
        echoes: vec![Echo::Reply(1)],
        ..Script::default()
    }
}

fn run(s: &mut Session, h: &mut Script, line: &str) -> Vec<String> {
    let mut out = Lines::default();
    s.run_line(line.as_bytes(), &mut out, h);
    out.text()
}

const OPERATOR: &[&str] =
    &["AttestRead", "CoreExec", "FileSystem", "IPC", "Memory", "ProcessControl"];

#[test]
fn a_role_without_network_cannot_ping_or_look_up() {
    for line in ["ping 10.0.2.2", "nslookup gw.lan"] {
        let mut h = host();
        let got = run(&mut login(OPERATOR), &mut h, &format!("{line}; echo $?"));
        let cmd = line.split(' ').next().unwrap();
        assert_eq!(
            got,
            [
                format!("{cmd}: the role operator does not hold Network, which this command needs"),
                "1".into()
            ]
        );
        assert!(h.resolved.is_empty() && h.echoed.is_empty());
    }
}

#[test]
fn a_role_without_ipc_cannot_read_the_registry() {
    for cmd in ["service", "svc"] {
        let mut h = host();
        let got = run(&mut login(&["CoreExec", "Memory"]), &mut h, &format!("{cmd} nsh.attest"));
        assert_eq!(
            got,
            [format!("{cmd}: the role operator does not hold IPC, which this command needs")]
        );
        assert!(h.asked.is_empty());
    }
}

#[test]
fn an_alias_does_not_pass_the_gate() {
    let mut s = login(OPERATOR);
    let mut h = host();
    assert!(run(&mut s, &mut h, "alias p ping").is_empty());
    let got = run(&mut s, &mut h, "p 10.0.2.2");
    assert_eq!(got, ["ping: the role operator does not hold Network, which this command needs"]);
    assert!(h.resolved.is_empty() && h.echoed.is_empty());
}

#[test]
fn a_role_that_holds_the_capability_runs_the_command() {
    let mut h = host();
    let mut s = login(&["IPC", "Network"]);
    assert_eq!(run(&mut s, &mut h, "service nsh.attest"), ["nsh.attest  port=4486  pid=15"]);
    assert_eq!(run(&mut s, &mut h, "ping 10.0.2.2")[1], "reply from 10.0.2.2: seq=1 time=1 ms");
}

#[test]
fn the_terminal_runs_under_its_own_grant() {
    let mut h = host();
    let mut s = Session::default();
    assert_eq!(run(&mut s, &mut h, "service nsh.attest"), ["nsh.attest  port=4486  pid=15"]);
    assert_eq!(run(&mut s, &mut h, "nslookup gw.lan"), ["gw.lan -> 10.0.2.2"]);
}

#[test]
fn the_gate_leaves_every_other_command_alone() {
    let mut h = host();
    assert_eq!(run(&mut login(&[]), &mut h, "echo hi; true; echo $?"), ["hi", "0"]);
}
