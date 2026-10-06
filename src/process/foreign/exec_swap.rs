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

//! Replacing the context an exec leaves behind, and the thread pointer that
//! went with it.

pub(super) type Saved = Option<crate::arch::context::SavedUser>;

/// Put a context in place and hand back the one it displaced.
pub(super) fn swap(pid: u32, ctx: Saved) -> Option<Saved> {
    crate::process::with_process(pid, |p| {
        core::mem::replace(&mut *p.saved_user_context.lock(), ctx)
    })
}

/// Forget the thread pointer the replaced runtime set: the scheduler writes
/// the control block's base on every switch, so leaving it would put the new
/// image back on the old TLS the first time it is preempted.
pub(super) fn drop_tls(pid: u32) {
    crate::process::with_process(pid, |pcb| pcb.set_tls_base(0));
}
