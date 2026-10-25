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

//! How far the panic path has gone, so a fault or panic on its way out
//! neither starts it again nor halts short of the wipe. One word holds the
//! stage in its low byte and the owning CPU above it, so taking the path and
//! naming who took it are one compare-and-swap. Pure, so the host checks it.

/// No CPU has taken the path.
pub(super) const IDLE: u64 = 0;
/// The owner is printing, painting and stopping the other CPUs.
pub(super) const STOPPING: u64 = 1;
/// The owner has begun the wipe: nothing is safe to start again.
pub(super) const WIPING: u64 = 2;

/// What a CPU entering the panic path does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Next {
    /// It took the path: print, paint, stop the others, wipe, off.
    Full,
    /// The owner faulted before its wipe: stop the others, wipe, off.
    Wipe,
    /// Another CPU owns the path and stops this one, or the wipe has begun.
    Park,
}

/// The word for `stage` owned by `cpu` (an APIC id plus one, never 0).
pub(super) const fn word(stage: u64, cpu: u64) -> u64 {
    (cpu << 8) | stage
}

/// Given the word a failed claim found, what `me` does.
pub(super) fn next(found: u64, me: u64) -> Next {
    match (found & 0xFF, found >> 8) {
        (IDLE, _) => Next::Full,
        (STOPPING, owner) if owner == me => Next::Wipe,
        _ => Next::Park,
    }
}

/// Whether the panic path has been taken: `terminate` parks if so, rather
/// than start its leave while the owner stops the CPUs.
pub(crate) fn taken(word: u64) -> bool {
    word & 0xFF != IDLE
}
