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

//! A process's death and the calls around it: the queue rule and the table of
//! abandoned calls, included by path, and the order the kernel calls them in.

extern crate alloc;

#[path = "../../../../src/syscall/microkernel/ipc/pending_reply/abandoned.rs"]
mod abandoned;
#[path = "../../../../src/syscall/microkernel/ipc/pending_reply/death.rs"]
mod death;
#[path = "../../../../src/syscall/microkernel/ipc/pending_reply/entry.rs"]
pub(crate) mod entry;

#[cfg(test)]
mod order;
#[cfg(test)]
mod tests;
