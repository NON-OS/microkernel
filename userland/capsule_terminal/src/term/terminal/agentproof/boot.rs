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

//! A headless AgentProof run: no window. Once vfs answers, it reads the job
//! the image holds, runs it as the Terminal runs a typed line, copies what
//! the screen showed to the serial log, and exits. The box's own `[BOX]` or
//! `[BOX-MISSING]` line is written by the Terminal's code, as at any run.
//! Built only with `agentproof-headless`, which only a Gauntlet image turns
//! on (scripts/check_copy_only_features.py).

use nonos_app_skeleton::clients::vfs::read_file;
use nonos_libc::{heap_init, mk_exit, mk_uptime_ms, mk_yield, HeapError};

use super::dump::{dump, say};
use super::job::{line_of, JOB_MAX, JOB_PATH};
use crate::jobs::{self, JobState};
use crate::term::state::State;

/// How long vfs may take to answer, and the run to end, in milliseconds.
const READY_MS: i64 = 300_000;
const RUN_MS: i64 = 1_800_000;

pub fn main() -> ! {
    if let Err(e) = heap_init() {
        if e != HeapError::AlreadyInitialized {
            done(b"[AGENTPROOF] v=1 status=error why=heap\n", 2);
        }
    }
    let mut state = State::new();
    let start = mk_uptime_ms();
    let job = loop {
        match read_file(state.owner_pid, JOB_PATH, JOB_MAX as u32 + 1) {
            Ok(job) => break job,
            Err(_) if mk_uptime_ms() - start < READY_MS => {
                let _ = mk_yield();
            }
            Err(_) => done(b"[AGENTPROOF] v=1 status=error why=no-job\n", 2),
        }
    };
    let line = match line_of(&job) {
        Ok(line) => line,
        Err(why) => {
            say(&alloc::format!("[AGENTPROOF] v=1 status=refused why={why}\n"));
            mk_exit(2)
        }
    };
    say(&alloc::format!("[AGENTPROOF] v=1 status=running line={}\n", lossy(&line)));
    state.line.replace(&line);
    let _ = crate::event::on_enter(&mut state);
    let started = mk_uptime_ms();
    while state.jobs.iter().any(|j| j.state == JobState::Running) {
        if mk_uptime_ms() - started > RUN_MS {
            dump(&mut state);
            done(b"[AGENTPROOF] v=1 status=error why=timeout\n", 3);
        }
        jobs::pump(&mut state);
        let _ = mk_yield();
    }
    dump(&mut state);
    let line = alloc::format!("[AGENTPROOF] v=1 status=done exit={}\n", state.last_status);
    say(&line);
    mk_exit(0)
}

fn lossy(b: &[u8]) -> alloc::string::String {
    alloc::string::String::from_utf8_lossy(b).into_owned()
}

fn done(line: &[u8], code: i32) -> ! {
    say(&lossy(line));
    mk_exit(code)
}
