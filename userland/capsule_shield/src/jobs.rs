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

//! The worker: one job at a time, off the IPC thread, so a sync or a proof
//! never keeps the wallet window waiting for an answer.

use std::sync::Mutex;

mod phase;
mod result;
mod run;
mod start;

pub use result::result;
pub use start::start;

pub enum State {
    Running,
    Done(String),
    Failed(String),
}

struct Job {
    name: &'static str,
    state: State,
}

static SLOT: Mutex<Option<Job>> = Mutex::new(None);

/// The last job's thread. A thread whose handle is dropped never has its stack freed on
/// NONOS, so a sync every few minutes would leak 16 MiB each time until the service died: the
/// finished thread is joined, which frees it, before the next job starts.
static LAST: Mutex<Option<std::thread::JoinHandle<()>>> = Mutex::new(None);

/// The name of the job running now, if one is.
pub fn running() -> Option<&'static str> {
    let slot = SLOT.lock().ok()?;
    match slot.as_ref() {
        Some(Job { name, state: State::Running }) => Some(name),
        _ => None,
    }
}
