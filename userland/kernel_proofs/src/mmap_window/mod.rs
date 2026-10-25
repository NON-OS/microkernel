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

/*
 * Where the kernel reads the loader's memory map: through the direct map,
 * and only a window the direct map holds whole, with the entry count bound.
 * It was read at its physical address with the count as given. The kernel's
 * pure rule is included by path.
 */

#[path = "../../../../src/boot/handoff/types/map_window.rs"]
pub mod map_window;
mod tests;
