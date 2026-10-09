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

//! The object behind an eventfd.
//!
//! It is the counter, not the descriptor: dup, fork and every thread reach
//! one counter through their own descriptors, so it is kept with the pipes
//! as the family's, and a descriptor names it by index.

#[derive(Clone, Copy)]
pub struct Event {
    pub count: u64,
    /// EFD_SEMAPHORE: a read takes one from the count rather than all of it.
    pub semaphore: bool,
}
