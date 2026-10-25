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


//! One completion entry, read while the controller may still be writing it.
//! The phase tag sits in the status half-word, so that is read first and the
//! rest only after a read barrier. Read the other way round, the command id
//! from the slot's last pass can pair with this pass's phase tag: the wait
//! then takes its own command's completion for another's, consumes it, and
//! waits out its whole budget for an entry already gone.

use super::completion::Completion;

/// The entry made of `status`, read first, and `rest`, read after `barrier`.
/// The status half-word `rest` carries is not used: only the first read's
/// phase tag says whether the controller had written the rest.
pub fn read_entry<S, B, R>(status: S, barrier: B, rest: R) -> Completion
where
    S: FnOnce() -> u16,
    B: FnOnce(),
    R: FnOnce() -> Completion,
{
    let status = status();
    barrier();
    Completion { status, ..rest() }
}
