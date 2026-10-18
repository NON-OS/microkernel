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

//! Deaths and failures of the display engine and of the driver at every
//! step of flip, mode set and cursor update (G-72), held against the
//! model's watch (`model/watch.rs`); and the owner's silence and a
//! restart's leftovers, the two ends of a death seen from outside.

pub(crate) mod flip_deaths;
pub mod kit;
pub mod kit_held;
mod leftover_tests;
pub(crate) mod mode_deaths;
mod silence_tests;
