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

//! LPIT's low-power idle states and their residency counters, as the x86
//! parser keeps them, through the broker's one platform shim so the
//! syscall layer names no arch path.

use super::kernel::platform::fw;

pub use fw::parser::other::lpit_entry::LpiState;
pub use fw::parser::other::{lpit_residency, lpit_state, CounterError};
