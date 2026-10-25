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

#[allow(dead_code)]
#[path = "../../../../../src/syscall/microkernel/errnos.rs"]
pub mod errnos;
#[allow(dead_code)]
#[path = "../../../../../src/syscall/microkernel/acpi/owner_gate.rs"]
pub mod owner_gate;
#[cfg(test)]
mod owner_gate_names_tests;
#[cfg(test)]
mod owner_gate_tests;
#[path = "../../../../../src/syscall/microkernel/retired.rs"]
pub mod retired;
#[cfg(test)]
mod retired_tests;

pub mod caps_of;
pub mod enqueued_at;
pub mod reply_dest;
pub mod wait;
