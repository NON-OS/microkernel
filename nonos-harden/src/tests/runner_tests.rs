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

//! The runner the tree's verifiers go through: an exit code and the tail of
//! what was printed come back; a command past its limit is stopped and is an
//! error, as is one that cannot start, so neither can pass a check.

use crate::tree::runner::{Cmd, Runner, System};
use std::time::{Duration, Instant};

fn sh(script: &str, limit: Duration) -> Cmd {
    let args = vec!["-c".into(), script.into()];
    Cmd { program: "sh".into(), args, cwd: std::env::temp_dir(), env: vec![], limit }
}

#[test]
fn an_ended_command_gives_its_code_and_what_it_printed() {
    let dir = tempfile::tempdir().unwrap();
    let cmd = sh("echo first; echo last >&2; exit 3", Duration::from_secs(30));
    let done = System.run(&cmd, &dir.path().join("log")).unwrap();
    assert_eq!(done.code, Some(3));
    assert!(done.tail.contains("first") && done.tail.contains("last"), "{}", done.tail);
}

#[test]
fn a_command_past_its_limit_is_stopped_and_is_an_error() {
    let dir = tempfile::tempdir().unwrap();
    let started = Instant::now();
    let got = System.run(&sh("sleep 30", Duration::from_millis(300)), &dir.path().join("log"));
    assert!(got.is_err_and(|e| e.contains("limit")));
    assert!(started.elapsed() < Duration::from_secs(10), "it was not stopped");
}

#[test]
fn a_command_that_cannot_start_is_an_error() {
    let dir = tempfile::tempdir().unwrap();
    let mut cmd = sh("true", Duration::from_secs(5));
    cmd.program = "nonos-harden-no-such-program".into();
    assert!(System.run(&cmd, &dir.path().join("log")).is_err());
}
