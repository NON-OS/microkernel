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

//! Serve points: where a loop that runs with interrupts masked answers the
//! TLB shootdowns it would otherwise not hear.

use crate::memory::paging::manager::{handle_shootdown_ipi, shootdown_in_flight};
use crate::smp::cpus_online;

/// Answer a TLB shootdown aimed at this CPU, if one is waiting.
///
/// Every system call runs with interrupts masked (SFMASK clears IF), so a
/// call that does a long stretch of work without taking a lock never hears
/// the shootdown vector either. A full-screen present is one: a 1920x1080
/// blit moves 8 MiB with interrupts off, which under emulation outlasts the
/// shootdown budget, and the originator halted the machine while the
/// presenting CPU was still copying. Such loops call this between units of
/// work. Answering early is always safe: the flush only drops translations,
/// and the pending flag makes a later delivery of the vector a no-op.
#[inline]
pub fn serve_shootdowns() {
    /*
     * Nothing can be pending on a uniprocessor, and resolving the current
     * CPU costs an interrupt-controller read, so the common case pays only
     * the compare. On more than one CPU the same read is skipped while no
     * round is in flight, which is nearly always, so a loop can afford to
     * call this once per page.
     */
    if cpus_online() > 1 && shootdown_in_flight() {
        handle_shootdown_ipi();
    }
}

/// The most work, in bytes, a masked loop does between two serve points.
pub const SERVE_UNIT: usize = 64 * 1024;

/// Hand `data` to `step` in pieces of at most [`SERVE_UNIT`] bytes, answering
/// any TLB shootdown between pieces. For hashing and other pure computation
/// over kernel buffers: nothing is held across the serve point but `data`,
/// which is kernel memory and is not touched by a user page-table change.
pub fn in_serve_units(data: &[u8], mut step: impl FnMut(&[u8])) {
    for (i, piece) in data.chunks(SERVE_UNIT).enumerate() {
        if i != 0 {
            serve_shootdowns();
        }
        step(piece);
    }
}
