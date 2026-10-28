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

//! The ids the driver gives its requests.

use crate::constants::vstor::RESET_ID;

/// The id after `id`: 1 upward, wrapping short of the start's and the
/// reset's ids; 0 is the host's own.
pub fn next_id(id: u64) -> u64 {
    if id >= RESET_ID - 1 {
        1
    } else {
        id + 1
    }
}
