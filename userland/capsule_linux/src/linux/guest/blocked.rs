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

//! A call waiting on descriptors, left parked in its trap until it can
//! complete or its deadline passes.

#[derive(Clone, Copy)]
pub struct Blocked {
    pub tid: u32,
    /// The call and its arguments as the guest gave them, so the family can
    /// try it again whenever something may have changed.
    pub nr: u64,
    pub args: [u64; 6],
    /// Monotonic milliseconds after which it is answered with nothing ready.
    /// None waits for as long as it takes.
    pub deadline: Option<u64>,
    /// Bytes of a write already put in: Linux answers a write to a blocking
    /// pipe only once the whole of it is, so it waits on for the rest.
    pub done: u64,
}
