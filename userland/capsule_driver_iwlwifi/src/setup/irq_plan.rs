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

//! What a refused interrupt bind means for the attempt. The bind tries legacy
//! INTx when firmware routed a line, then one MSI-X vector, then runs polled:
//! every wait in this driver is a bounded register poll, so no interrupt grant
//! is needed to make progress. A refusal only ends the attempt when it says
//! the claim itself is gone, because then nothing further can be bound and the
//! grants already held must be given back.

/// The broker's errno for a claim epoch that is no longer current (ESTALE).
const ERRNO_STALE: i64 = -116;
/// EPERM: for MSI-X, the device is not claimed by this capsule. For INTx the
/// broker also uses it for a line the kernel reserves, which MSI-X sidesteps.
const ERRNO_PERM: i64 = -1;

/// What to do after a bind was refused with an errno.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Refusal {
    /// The claim is gone: give everything back and end the attempt.
    ClaimLost,
    /// Try the next interrupt kind, or run polled after the last.
    TryNext,
}

/// After an INTx bind was refused with `rc`.
pub fn after_intx(rc: i64) -> Refusal {
    if rc == ERRNO_STALE {
        Refusal::ClaimLost
    } else {
        Refusal::TryNext
    }
}

/// After an MSI-X bind was refused with `rc`.
pub fn after_msix(rc: i64) -> Refusal {
    if rc == ERRNO_STALE || rc == ERRNO_PERM {
        Refusal::ClaimLost
    } else {
        Refusal::TryNext
    }
}
