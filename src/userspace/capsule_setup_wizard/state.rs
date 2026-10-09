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

use core::sync::atomic::{AtomicU32, Ordering};

use crate::services::lifecycle::CapsuleState;

static STATE: CapsuleState = CapsuleState::new();

/// The pid setup ran as. The lifecycle tick clears the pid in `STATE` once
/// the process is gone, and its exit status is read after that.
static SPAWNED: AtomicU32 = AtomicU32::new(0);

pub(super) fn set_alive(pid: u32) {
    SPAWNED.store(pid, Ordering::SeqCst);
    STATE.set_alive(pid);
}

#[cfg(feature = "microkernel-setup-wizard")]
pub(super) fn spawned_pid() -> u32 {
    SPAWNED.load(Ordering::SeqCst)
}

pub fn shared_state() -> &'static CapsuleState {
    &STATE
}
