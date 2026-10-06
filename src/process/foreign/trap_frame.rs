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

//! The register state of a guest parked inside a syscall, kept for a fork.
//!
//! It is not the scheduler's resume slot. A guest that yields while parked
//! and is switched back without a kernel context would be resumed from that
//! slot, returning to user mode with its syscall number in rax as if that
//! were the answer. So the frame lives here, where only fork reads it.

use alloc::collections::BTreeMap;

use spin::Mutex;

use crate::arch::context::SavedUser;

static FRAMES: Mutex<BTreeMap<u32, SavedUser>> = Mutex::new(BTreeMap::new());

pub(super) fn keep(pid: u32, frame: SavedUser) {
    FRAMES.lock().insert(pid, frame);
}

/// The frame of a guest still parked, or `None` once it has its answer.
pub(super) fn parked_frame(pid: u32) -> Option<SavedUser> {
    FRAMES.lock().get(&pid).copied()
}

pub(super) fn drop_frame(pid: u32) {
    FRAMES.lock().remove(&pid);
}
