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

//! The runner that starts real processes: its own process group, so a limit
//! stops everything it started, and its own scratch space for what cargo
//! builds and Python caches, so nothing left in the tree is run.

use super::runner::{Cmd, Done, Runner};
use super::scratch::tail;
use std::fs::File;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

pub struct System;

impl Runner for System {
    fn run(&self, cmd: &Cmd, log: &Path) -> Result<Done, String> {
        let out = File::create(log).map_err(|e| format!("cannot write {} ({e})", log.display()))?;
        let err = out.try_clone().map_err(|e| e.to_string())?;
        let mut c = Command::new(&cmd.program);
        c.args(&cmd.args).current_dir(&cmd.cwd).envs(cmd.env.iter().map(|(k, v)| (k, v)));
        // Build outputs and bytecode go to this run's own scratch space: the
        // tree's target and __pycache__ directories are never read from.
        let own = log.parent().unwrap_or(log);
        c.env("CARGO_TARGET_DIR", own.join("cargo-target"));
        c.env("PYTHONPYCACHEPREFIX", own.join("pycache")).env("PYTHONDONTWRITEBYTECODE", "1");
        c.process_group(0);
        let spawned = c.stdin(Stdio::null()).stdout(out).stderr(err).spawn();
        let mut child = spawned.map_err(|e| format!("`{}` did not start ({e})", cmd.program))?;
        let started = Instant::now();
        let code = loop {
            match child.try_wait().map_err(|e| e.to_string())? {
                Some(status) => break status.code(),
                None if started.elapsed() > cmd.limit => {
                    // The whole group: whatever the verifier started stops too.
                    unsafe { libc::kill(-(child.id() as i32), libc::SIGKILL) };
                    let _ = child.wait();
                    let mins = cmd.limit.as_secs() / 60;
                    return Err(format!(
                        "`{}` ran past its {mins} minute limit and was stopped",
                        cmd.program
                    ));
                }
                None => std::thread::sleep(Duration::from_millis(100)),
            }
        };
        Ok(Done { code, tail: tail(log) })
    }
}
