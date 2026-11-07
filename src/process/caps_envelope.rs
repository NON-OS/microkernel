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

//! What a grant may ever give a process. `sys_cap_grant` lets an Admin
//! holder add bits to another process's word; without a bound that is any
//! bit the granter holds, whatever the target's manifest said. The envelope
//! is recorded once, when the spawn path installs the manifest word, and
//! never changes: a grant outside it is refused. A process with no
//! recorded envelope has none, so nothing can be granted to it.

use core::sync::atomic::Ordering;

use super::api::{with_process, with_process_mut};

/// Records `envelope` for `pid`. The first record stands; a second returns
/// None and changes nothing.
pub fn record(pid: u32, envelope: u64) -> Option<()> {
    with_process_mut(pid, |pcb| {
        pcb.caps_envelope_set
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .ok()
            .map(|_| pcb.caps_envelope.store(envelope, Ordering::SeqCst))
    })
    .flatten()
}

/// The envelope of a live process; zero when none was recorded, None when
/// there is no such process.
pub fn envelope(pid: u32) -> Option<u64> {
    with_process(pid, |pcb| {
        if pcb.caps_envelope_set.load(Ordering::Acquire) {
            pcb.caps_envelope.load(Ordering::Acquire)
        } else {
            0
        }
    })
}
