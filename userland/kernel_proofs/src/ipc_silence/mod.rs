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

//! A server silent to a caller across late calls is reported unanswered,
//! from a fixed table whose eviction can only make a report earlier.

#[path = "../../../../src/syscall/microkernel/ipc/call/silence.rs"]
mod silence;

#[cfg(test)]
mod model;
#[cfg(test)]
mod polls;
#[cfg(test)]
mod replies;
#[cfg(test)]
mod tests;
