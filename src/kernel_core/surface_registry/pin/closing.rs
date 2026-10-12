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

use alloc::vec::Vec;

use spin::Mutex;

use crate::kernel_core::surface_registry::table::SLOTS;
use crate::kernel_core::surface_registry::types::{encode_handle, SurfaceHandle};

use super::closing_rule::{admits, close, forget, Closing};

static CLOSING: Mutex<Closing<SurfaceHandle>> = Mutex::new(Vec::new());

/* Under GATE, from keep_at_exit: every slot release_owned_by_pid will drop. */
pub(super) fn close_owned(pid: u32) {
    let handles: Vec<SurfaceHandle> = SLOTS
        .lock()
        .iter()
        .enumerate()
        .filter_map(|(idx, e)| {
            let s = e.as_ref()?;
            (s.owner_pid == pid).then(|| encode_handle(idx as u32, s.epoch))
        })
        .collect();
    close(&mut CLOSING.lock(), pid, &handles);
}

/// Whether an attach of `handle` may go ahead: not while its owner exits.
pub fn attachable(handle: SurfaceHandle) -> bool {
    admits(&CLOSING.lock(), handle)
}

/// After release_owned_by_pid(`pid`): its handles are stale now.
pub fn forget_exited(pid: u32) {
    forget(&mut CLOSING.lock(), pid);
}
