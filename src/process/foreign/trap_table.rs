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

//! The parked guests: one entry per thread waiting inside a syscall this
//! kernel refused, holding its frame until its supervisor answers.

use alloc::vec::Vec;

use spin::Mutex;

use super::frame::ForeignFrame;
use super::registry;

pub(super) struct Parked {
    pub frame: ForeignFrame,
    /// Set by the supervisor's reply, read by the guest on wake.
    pub answer: Option<u64>,
    /// Taken by the first supervisor wait that claims it.
    pub claimed: bool,
}

pub(super) static PARKED: Mutex<Vec<Parked>> = Mutex::new(Vec::new());

pub(super) fn park(frame: ForeignFrame) {
    PARKED.lock().push(Parked { frame, answer: None, claimed: false });
}

/// The answer for `pid`, removing the entry once it is taken.
pub(super) fn take_answer(pid: u32) -> Option<u64> {
    let mut parked = PARKED.lock();
    let at = parked.iter().position(|p| p.frame.pid == pid)?;
    let value = parked[at].answer?;
    parked.remove(at);
    Some(value)
}

/// The next unclaimed frame for `supervisor`, if one is waiting.
pub(super) fn claim_next(supervisor: u32) -> Option<ForeignFrame> {
    let mut parked = PARKED.lock();
    for entry in parked.iter_mut() {
        if entry.claimed || entry.answer.is_some() {
            continue;
        }
        if registry::supervisor_of(entry.frame.pid) == Some(supervisor) {
            entry.claimed = true;
            return Some(entry.frame);
        }
    }
    None
}
