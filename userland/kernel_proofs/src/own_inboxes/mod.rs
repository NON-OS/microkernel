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
//! A process's output never lands where its requests are read. MkDebug,
//! MkStdoutWrite and MkPrivateWrite write `stdout.<pid>`, MkProcOutput
//! drains it, and `proc.<pid>` holds requests and replies alone, so a
//! server never receives its own lines as requests from itself and a parent
//! draining a child's output never takes the child's requests.

#[path = "../../../../src/ipc/nonos_inbox/own_names.rs"]
mod own_names;
#[path = "../../../../src/syscall/microkernel/ipc/sender_pid.rs"]
mod sender_pid;

#[cfg(test)]
mod tests;
