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

use super::evict::victim;

/// Bounds the table. When it is full, the finished install recorded
/// longest ago gives way (`evict`); one still moving never does.
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
    /// The personality is taking the package away.
    Removing(u32),
    Removed,
}

impl Stage {
    /// Nothing more will happen to it without a new request.
    fn finished(self) -> bool {
        matches!(self, Stage::Installed | Stage::Removed | Stage::Failed(_) | Stage::Refused)
    }
}

#[derive(Clone, Copy)]
struct Kept {
    stage: Stage,
    /// When it was last set, in the order of sets: the age eviction reads.
    at: u64,
}

struct Table {
    kept: BTreeMap<String, Kept>,
    next: u64,
}

static STAGES: Mutex<Table> = Mutex::new(Table { kept: BTreeMap::new(), next: 0 });

/// Record `stage` for `listing`. False only when the table is full of
/// installs still moving and `listing` is not among them, so nothing could
/// make room; the caller refuses the request as busy rather than run an
/// install nobody could see.
pub(crate) fn set(listing: &str, stage: Stage) -> bool {
    let mut t = STAGES.lock();
    if !room(&mut t, listing) {
        return false;
    }
    let at = t.next;
    t.next = t.next.wrapping_add(1);
    t.kept.insert(String::from(listing), Kept { stage, at });
    true
}

/// Whether `listing` can be recorded, making room for it if a finished
/// install can give way.
pub(crate) fn room_for(listing: &str) -> bool {
    room(&mut STAGES.lock(), listing)
}

fn room(t: &mut Table, listing: &str) -> bool {
    if t.kept.len() < CAP || t.kept.contains_key(listing) {
        return true;
    }
    let entries = t.kept.iter().map(|(k, v)| (k, v.stage.finished(), v.at));
    let Some(oldest) = victim(entries).cloned() else { return false };
    t.kept.remove(&oldest);
    true
}

/// Whether an install or an uninstall is still moving. The personality runs
/// one installer at a time (its endpoint, `app.linux.install`, has one
/// owner), so the next has to wait for it, not be refused.
pub(crate) fn one_moving() -> bool {
    let moving: alloc::vec::Vec<String> = STAGES
        .lock()
        .kept
        .iter()
        .filter(|(_, k)| matches!(k.stage, Stage::Running(_) | Stage::Removing(_)))
        .map(|(name, _)| name.clone())
        .collect();
    // `get` reads an ended installer's result in, so one that has just
    // finished stops counting here on the same pass.
    moving.iter().any(|name| matches!(get(name), Some(Stage::Running(_) | Stage::Removing(_))))
}

/// The stage, with a finished installer's result read in and kept.
pub(crate) fn get(listing: &str) -> Option<Stage> {
    let mut t = STAGES.lock();
    let kept = *t.kept.get(listing)?;
    let (pid, removing) = match kept.stage {
        Stage::Running(pid) => (pid, false),
        Stage::Removing(pid) => (pid, true),
        _ => return Some(kept.stage),
    };
    let ended = match PROCESS_TABLE.find_by_pid(pid) {
        Some(pcb) => match *pcb.state.lock() {
            ProcessState::Zombie(code) | ProcessState::Terminated(code) => Some(code),
            _ => None,
        },
        None => Some(crate::process::exit::peek_exit_status(pid).unwrap_or(-1)),
    };
    let now = match ended {
        None => kept.stage,
        Some(0) if removing => Stage::Removed,
        Some(0) => Stage::Installed,
        Some(code) => Stage::Failed(code),
    };
    // Its age stays when it was asked for, not when it was read.
    t.kept.insert(String::from(listing), Kept { stage: now, at: kept.at });
    Some(now)
}
