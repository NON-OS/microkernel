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

//! The broker's Bus Master command as bring-up hands it to the queue step,
//! on the host: it notes what the device holds at that moment.

use std::cell::Cell;

/// Note `seen` in `at`, and Bus Master is on.
pub fn noted<T>(at: &Cell<Option<T>>, seen: T) -> Result<(), &'static str> {
    at.set(Some(seen));
    Ok(())
}
