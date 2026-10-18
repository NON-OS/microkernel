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
//! E-51: the core against a chip that lies at its receive boundary; E-53:
//! one that restarts, or loses the keys it was given.

mod held_key_tests;
mod junk;
mod keys_tests;
mod liar;
mod look_tests;
mod out_tests;
pub mod restart;
pub mod restart_rig;
mod restart_tests;
mod rig;
mod rx_tests;
