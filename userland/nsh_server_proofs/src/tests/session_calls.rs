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

//! The session: the client's frames reach the host as events, each held to
//! its body's rules; the host's output, commands and refused calls reach
//! the record; and the shell is stopped for exactly the reasons the record
//! can name.

use super::good::{kinds, received};
use crate::kit::roles::Table;
use crate::kit::roles_build::grant::GrantB;
use crate::kit::roles_build::revoke::RevB;
use crate::kit::roles_build::table::TableB;
use crate::kit::run::Run;
use nsh_server::CallError;
use nsh_wire::EntryBody;

#[test]
fn commands_and_refused_calls_are_recorded() {
    let mut run = Run::logged_in();
    run.host(|s, now, r| s.command(b"ls -l", now, r)).expect("command");
    run.host(|s, now, r| s.command(b"", now, r)).expect("an empty line is not a command");
    let long = vec![b'x'; 4097];
    run.host(|s, now, r| s.command(&long, now, r)).expect("command");
    run.host(|s, now, r| s.refused_call(0x4D55_4F50, b"UserFiles", now, r)).expect("refused call");
    assert_eq!(
        run.host(|s, now, r| s.refused_call(1, b"not a cap", now, r)),
        Err(CallError::Malformed)
    );
    run.host(|s, now, r| s.exit(0, now, r)).expect("exit");
    let rec = received(&run);
    let digest = nonos_hash::sha256(&long);
    let want = [
        EntryBody::Command(b"ls -l"),
        EntryBody::CommandDigest(&digest),
        EntryBody::RefusedCall { tag: 0x4D55_4F50, cap: b"UserFiles" },
        EntryBody::Exit(0),
    ];
    assert_eq!(kinds(&rec), want);
}

#[test]
fn a_role_that_keeps_digests_keeps_no_command_text() {
    let mut tb = TableB::default();
    tb.role("operator").flags = 1;
    let mut run = Run::new(&GrantB::default(), Table::new(&tb, &RevB::default()));
    run.pump();
    run.host(|s, now, r| s.command(b"cat /secret", now, r)).expect("command");
    run.host(|s, now, r| s.exit(0, now, r)).expect("exit");
    let rec = received(&run);
    assert_eq!(kinds(&rec)[0], EntryBody::CommandDigest(&nonos_hash::sha256(b"cat /secret")));
}
