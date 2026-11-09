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

//! An nsh session runs the shared NONOS shell, as nsh.front wires it: each
//! line is recorded as a command, run by nonos_shell's session with its
//! control flow and filters, and its output sent sealed as one OUTPUT. The
//! client receives exactly the shell's lines, and the record holds each
//! command before its output.

use super::good::{kinds, received};
use crate::kit::run::Run;
use nonos_shell::host::{Host, Mem, Proc};
use nonos_shell::out::Crlf;
use nonos_shell::session::{Flow, Session, Who};
use nsh_client::Event as C;
use nsh_server::Event as S;
use nsh_wire::EntryBody;

/// A machine that answers nothing, as a capsule with no grant beyond the
/// session's would.
struct Bare;

impl Host for Bare {
    fn random(&mut self, _: &mut [u8]) -> bool {
        false
    }
    fn wall_ms(&mut self) -> Option<u64> {
        None
    }
    fn uptime_ms(&mut self) -> Option<u64> {
        None
    }
    fn mem(&mut self) -> Option<Mem> {
        None
    }
    fn procs(&mut self) -> Option<Vec<Proc>> {
        None
    }
    fn read_file(&mut self, _: &[u8], _: usize) -> Option<Vec<u8>> {
        None
    }
}

/// One typed line through the session, as nsh.front's shell runs it.
fn line(run: &mut Run, sh: &mut Session, text: &[u8]) -> Flow {
    run.host(|s, now, r| s.command(text, now, r)).expect("command");
    let mut out = Crlf::default();
    let flow = sh.run_line(text, &mut out, &mut Bare);
    if !out.0.is_empty() {
        run.host(|s, now, r| s.output(&out.0, now, r)).expect("output");
    }
    flow
}

#[test]
fn the_session_runs_the_shared_shell_end_to_end() {
    let mut run = Run::logged_in();
    let mut sh = Session::default();
    assert_eq!(line(&mut run, &mut sh, b"for i in 1 2; do echo nsh-$i; done"), Flow::Next);
    assert_eq!(
        line(&mut run, &mut sh, b"if test 6 -lt 7; then seq 40 | tail -n 1; fi"),
        Flow::Next
    );
    assert_eq!(line(&mut run, &mut sh, b"date || echo no clock here"), Flow::Next);
    assert_eq!(line(&mut run, &mut sh, b"exit"), Flow::Exit(0));
    run.host(|s, now, r| s.exit(0, now, r)).expect("exit");
    let got: Vec<u8> = run
        .c_events
        .iter()
        .filter_map(|e| match e {
            C::Output(b) => Some(b.clone()),
            _ => None,
        })
        .flatten()
        .collect();
    assert_eq!(got, b"nsh-1\r\nnsh-2\r\n40\r\ndate: clock unavailable\r\nno clock here\r\n");
    let rec = received(&run);
    let k = kinds(&rec);
    assert_eq!(k[0], EntryBody::Command(b"for i in 1 2; do echo nsh-$i; done"));
    assert!(matches!(k[1], EntryBody::Output { bytes: 14, .. }));
    assert_eq!(k.last(), Some(&EntryBody::Exit(0)));
}

#[test]
fn whoami_is_the_login_the_server_admitted() {
    let mut run = Run::logged_in();
    let a = run
        .s_events
        .iter()
        .find_map(|e| match e {
            S::Admit(a) => Some(a.clone()),
            _ => None,
        })
        .expect("admitted");
    let mut sh = Session {
        who: Some(Who {
            user: a.user.clone(),
            role: a.role.clone(),
            subject: a.subject,
            caps: a.caps.names(),
        }),
        ..Session::default()
    };
    assert_eq!(line(&mut run, &mut sh, b"whoami"), Flow::Next);
    let got: Vec<u8> = run
        .c_events
        .iter()
        .filter_map(|e| match e {
            C::Output(b) => Some(b.clone()),
            _ => None,
        })
        .flatten()
        .collect();
    let hex: String = a.subject.iter().map(|b| format!("{b:02x}")).collect();
    let user = String::from_utf8(a.user).expect("user");
    let role = String::from_utf8(a.role).expect("role");
    assert_eq!(
        String::from_utf8(got).expect("text"),
        format!("user {user}\r\nrole {role}\r\nkey sha512 {hex}\r\n")
    );
}
