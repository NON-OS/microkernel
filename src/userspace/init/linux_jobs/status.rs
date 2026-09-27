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

//! Where each asked-for install stands, for the store to show. Keyed by
//! listing; the running stage resolves to the installer's exit code once
//! that process has ended.

use alloc::collections::BTreeMap;
use alloc::string::String;

use spin::Mutex;

use crate::process::core::{ProcessState, PROCESS_TABLE};

/// Enough for every listing a person could ask for in one session.
const CAP: usize = 32;

#[derive(Clone, Copy)]
pub(crate) enum Stage {
    Queued,
    Running(u32),
    Installed,
    /// The installer's exit code: `install::Why` in the personality.
    Failed(i32),
    /// Init would not start it: the market withdrew it, or the spawn gate refused.
    Refused,
}

static STAGES: Mutex<BTreeMap<String, Stage>> = Mutex::new(BTreeMap::new());

pub(crate) fn set(listing: &str, stage: Stage) {
    let mut s = STAGES.lock();
    if s.len() >= CAP && !s.contains_key(listing) {
        return;
    }
    s.insert(String::from(listing), stage);
}

/// The stage, with a finished installer's result read in and kept.
pub(crate) fn get(listing: &str) -> Option<Stage> {
    let mut s = STAGES.lock();
    let stage = *s.get(listing)?;
    let Stage::Running(pid) = stage else { return Some(stage) };
    let ended = match PROCESS_TABLE.find_by_pid(pid) {
        Some(pcb) => match *pcb.state.lock() {
            ProcessState::Zombie(code) | ProcessState::Terminated(code) => Some(code),
            _ => None,
        },
        None => Some(crate::process::exit::peek_exit_status(pid).unwrap_or(-1)),
    };
    let now = match ended {
        None => stage,
        Some(0) => Stage::Installed,
        Some(code) => Stage::Failed(code),
    };
    s.insert(String::from(listing), now);
    Some(now)
}
