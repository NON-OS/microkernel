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

//! Exclusion over a guest's address space.

use spin::{Mutex, MutexGuard};

// Every peer call that reads or reshapes a guest's address space runs under
// this.
static ADDRESS_SPACE: Mutex<()> = Mutex::new(());

/// Proof that the caller holds the peer lock.
pub(super) type Held = MutexGuard<'static, ()>;

/// Take it. Only `peer_guard::supervised_asid` calls this, and it hands
/// the result back beside the asid so the two cannot be separated.
///
/// The holder maps and unmaps in the guest's tables, and each change waits
/// for every CPU running the guest to acknowledge a TLB shootdown. Peer calls
/// arrive as system calls, with interrupts masked, so a CPU waiting here
/// answers shootdowns while it spins or the two would wait on each other.
pub(super) fn take() -> Held {
    crate::smp::lock_responsive(&ADDRESS_SPACE)
}
