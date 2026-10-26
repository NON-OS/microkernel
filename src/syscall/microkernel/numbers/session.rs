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

//! Instances, tools, capability reads and the terminal.

use crate::syscall::abi::tag4;

/// Spawn another window of an embedded, attested app (terminal or browser); needs SpawnWindow.
pub const SYS_SPAWN_INSTANCE: u64 = tag4(b"MSPI");
/// Run a baked, attested tool by name as the caller's child, to drive its stdio; needs IPC.
pub const SYS_TOOL_RUN: u64 = tag4(b"MTRN");
pub const SYS_CAPS_OF: u64 = tag4(b"MCOF");
pub const SYS_TTY_SET: u64 = tag4(b"MTTY");
pub const SYS_TTY_QUERY: u64 = tag4(b"MTTQ");
