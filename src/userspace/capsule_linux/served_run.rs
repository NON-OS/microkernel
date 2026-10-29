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

//! The one served run, kept: one at a time, as there is one run
//! (served_run_rule.rs, run_owner.rs).

use alloc::string::String;
use spin::Mutex;

use super::served_run_rule::{exited, restarting, started, Served};
use crate::process::exit::teardown;
use crate::process::signal::SIGKILL;

static SERVED: Mutex<Option<Served>> = Mutex::new(None);

/// What a run ended with its launcher reads as: killed, as by SIGKILL.
const ENDED: i32 = 128 + SIGKILL as i32;

/// MkAppLaunch queues `package`: as `launcher`'s, a SERVES capsule's process,
/// with the `word` it gave (served_word.rs); or with None, as no one's.
pub fn note_launch(launcher: Option<u32>, package: &str, word: Option<&str>) {
    super::served_word::keep(package, word.filter(|_| launcher.is_some()));
    let package = String::from(package);
    *SERVED.lock() = launcher.map(|l| Served { launcher: Some(l), package, run: None });
}

/// init started `package` as `pid`, in either run role.
pub fn run_started(package: &str, pid: u32) {
    let orphan = started(&mut SERVED.lock(), package, pid);
    if orphan {
        teardown(pid, ENDED, true);
    }
}

/// The run `pid` is queued to start again with the network (MkLinuxRunNet).
pub fn run_restarting(pid: u32) {
    restarting(&mut SERVED.lock(), pid);
}

/// `pid` is exiting (process/exit/teardown.rs): the runs it started from a
/// terminal end with it (terminal/exit.rs), and so does the served run it
/// launched. It is a zombie already, so no run's teardown comes back for it.
pub fn end_runs_of(pid: u32) {
    super::end_terminal_runs_of(pid);
    let run = exited(&mut SERVED.lock(), pid);
    if let Some(run) = run {
        teardown(run, ENDED, true);
    }
}
