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

//! MkMmap's backing loop (syscall/microkernel/memory/va/map_loop.rs), the
//! shipping source driven by a recorder: every frame zeroed before it is
//! mapped, shootdowns served before every page, an interrupt window every
//! WINDOW_PAGES pages and never more than that apart, and every failure
//! undone (cpu-6).

#[path = "../../../../src/syscall/microkernel/memory/va/map_loop.rs"]
mod map_loop;

#[cfg(test)]
mod tests;
