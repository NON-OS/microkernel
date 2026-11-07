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

//! Host doubles for the broker: objects in a map that can be cut at any
//! write, a host whose clients and clock the test sets, and a transport
//! that carries a client's requests straight into `serve`; and the Linux
//! personality's user-files rules, mounted from capsule_linux (`linux`).

extern crate alloc;

pub mod doubles;
pub mod fuzz_input;
pub mod linux;
pub mod rng;

#[cfg(test)]
mod tests;
