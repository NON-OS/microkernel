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

//! Running the tree's own verifiers. Each runs with no input, its output to a
//! log, and a time limit: a verifier that never ends is stopped and counted as
//! not having run, so a hung tool cannot hold the verdict back or pass it.

use super::scratch::tail;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

pub struct Cmd {
    pub program: String,
    pub args: Vec<String>,
    pub cwd: PathBuf,
    pub env: Vec<(String, String)>,
    pub limit: Duration,
}

/// How a command ended: its exit code (none if a signal or the limit ended
/// it) and the last of what it printed.
pub struct Done {
    pub code: Option<i32>,
    pub tail: String,
}

pub trait Runner {
    fn run(&self, cmd: &Cmd, log: &Path) -> Result<Done, String>;
}

pub struct System;

impl Runner for System {
    fn run(&self, cmd: &Cmd, log: &Path) -> Result<Done, String> {
        let out = File::create(log).map_err(|e| format!("cannot write {} ({e})", log.display()))?;
        let err = out.try_clone().map_err(|e| e.to_string())?;
        let mut c = Command::new(&cmd.program);
        c.args(&cmd.args).current_dir(&cmd.cwd).envs(cmd.env.iter().map(|(k, v)| (k, v)));
        let spawned = c.stdin(Stdio::null()).stdout(out).stderr(err).spawn();
        let mut child = spawned.map_err(|e| format!("`{}` did not start ({e})", cmd.program))?;
        let started = Instant::now();
        let code = loop {
            match child.try_wait().map_err(|e| e.to_string())? {
                Some(status) => break status.code(),
                None if started.elapsed() > cmd.limit => {
                    let _ = child.kill();
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
