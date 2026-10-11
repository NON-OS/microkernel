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

//! A send addressed to another capsule's endpoint (plan/54 K7, C6).
//!
//! The kernel delivers every endpoint a capsule owns into its one inbox,
//! proc.<owner> (ipc/kernel_ipc.rs), so a send by any of its names lands
//! where all of them are read: a send by name is a send by pid. Checking
//! only the name used let a capsule holding IPC reach each held driver by
//! its reply endpoint, and every net.* service by its reply endpoint
//! without Network. So such a send passes the gate of every endpoint the
//! owner serves, the rule MkIpcSendToPid keeps (held.rs inbox_admits). A
//! kernel-owned endpoint is read from its own inbox, and the caller's own
//! endpoints are its own: neither changes.
//!
//! Pure, and held in kernel_proofs (ipc_held_tests::addressed).

/// Whether an addressed send passes, beyond the gates of the name used.
/// `inbox_ok` is evaluated only when the owner is another capsule.
pub(crate) fn addressed_admits(
    kernel_owned: bool,
    caller_owns: bool,
    inbox_ok: impl FnOnce() -> bool,
) -> bool {
    kernel_owned || caller_owns || inbox_ok()
}
