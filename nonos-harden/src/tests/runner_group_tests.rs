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

//! A verifier past its limit is stopped with everything it started, and
//! every verifier builds and caches into the run's own scratch space, never
//! the tree's target or __pycache__ directories.

use crate::tree::runner::{Cmd, Runner, System};
use std::time::Duration;

fn sh(script: &str, limit: Duration) -> Cmd {
    let args = vec!["-c".into(), script.into()];
    Cmd { program: "sh".into(), args, cwd: std::env::temp_dir(), env: vec![], limit }
}

#[test]
fn a_process_the_verifier_started_is_stopped_with_it() {
    let dir = tempfile::tempdir().unwrap();
    let pid = dir.path().join("pid");
    let script = format!("sleep 60 & echo $! > {}; wait", pid.display());
    assert!(System.run(&sh(&script, Duration::from_millis(500)), &dir.path().join("log")).is_err());
    let pid: i32 = std::fs::read_to_string(&pid).unwrap().trim().parse().unwrap();
    std::thread::sleep(Duration::from_millis(200));
    // Gone, or a zombie no one has reaped yet: either way it runs no more.
    let state = std::fs::read_to_string(format!("/proc/{pid}/stat")).unwrap_or_default();
    let alive = state.rsplit(") ").next().is_some_and(|r| !r.starts_with('Z') && !r.is_empty());
    assert!(!alive, "the grandchild {pid} outlived the limit");
}

#[test]
fn cargo_and_python_write_into_the_runs_own_space() {
    let dir = tempfile::tempdir().unwrap();
    let script = "echo \"$CARGO_TARGET_DIR|$PYTHONPYCACHEPREFIX|$PYTHONDONTWRITEBYTECODE\"";
    let done = System.run(&sh(script, Duration::from_secs(10)), &dir.path().join("log")).unwrap();
    let own = dir.path().display().to_string();
    let want = format!("{own}/cargo-target|{own}/pycache|1");
    assert!(done.tail.contains(&want), "{} lacks {want}", done.tail);
}
