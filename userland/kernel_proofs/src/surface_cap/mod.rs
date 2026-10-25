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

//! H-06(b): how many bytes one surface may span, by the boot framebuffer.
//! The real rule is included by path.

#[path = "../../../../src/syscall/dispatch/router/surface_handlers/cap.rs"]
pub mod cap;

#[cfg(test)]
mod glue_tests;
#[cfg(test)]
mod slices_tests;
#[cfg(test)]
mod tests;
