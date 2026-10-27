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

//! Exclusion over one guest's address space.

mod stripe;

use crate::smp::{TicketGuard, TicketMutex};
use stripe::{stripe_of, STRIPES};

// Every peer call that reads or reshapes a guest's address space runs under
// that guest's lock (peer_lock/stripe.rs), taken in turn.
static ADDRESS_SPACES: [TicketMutex<()>; STRIPES] = [const { TicketMutex::new(()) }; STRIPES];

/// Proof that the caller holds a guest's peer lock.
pub(super) type Held = TicketGuard<'static, ()>;

/// `pid`'s address space and the lock over it. Only
/// `peer_guard::supervised_asid` calls this, and it hands the pair back
/// together so the two cannot be separated.
///
/// The holder maps and unmaps in the guest's tables, and each change waits
/// for every CPU running the guest to acknowledge a TLB shootdown. Peer calls
/// arrive as system calls, with interrupts masked, so a CPU waiting here
/// answers shootdowns while it spins or the two would wait on each other.
pub(super) fn hold(pid: u32) -> Option<(u32, Held)> {
    let find = || crate::memory::paging::manager::lookup_asid_for_process(pid);
    stripe::hold(find, |stripe| crate::smp::lock_responsive(&ADDRESS_SPACES[stripe]))
}

/// Run `f`, the release of the address space `asid`, with no peer call on
/// it in progress or able to start.
///
/// A supervisor's copy walks the guest's tables and writes its frames
/// through the kernel's own mapping, from its own CPU, so the exit path's
/// wait for every CPU to leave the guest's tables does not cover it. A guest
/// torn down from another CPU (its terminal ending, say) had its tables and
/// frames freed on a later tick while such a copy could still be writing:
/// into a frame by then another process's. The address space is now
/// released inside this, and the guest's row is gone before, so a call
/// either finished first or finds no guest. Calls on other guests go on.
pub fn without_peer_calls<R>(asid: u32, f: impl FnOnce() -> R) -> R {
    let _held = crate::smp::lock_responsive(&ADDRESS_SPACES[stripe_of(asid)]);
    f()
}
