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

//! The one lock between a port access and a device leaving its holder
//! (K10c). An access holds it shared from the grant check to the `in` or
//! `out`; a retire takes it alone once, after the claim is gone, so it
//! returns only when no access that passed the check is still running, and
//! no later one can pass. No port reaches a function in reset.

use spin::RwLock;

static IN_FLIGHT: RwLock<()> = RwLock::new(());

/// Run one access, check and instruction, under the shared lock.
pub(super) fn during<R>(access: impl FnOnce() -> R) -> R {
    let _held = IN_FLIGHT.read();
    access()
}

/// Wait until every access already past its check has finished.
pub(crate) fn quiesce() {
    drop(IN_FLIGHT.write());
}
