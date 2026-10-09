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
 * Which SD host controllers start the storage capsule as an eMMC disk. The
 * inventory's list is included by path. Before it an Atom-class laptop whose
 * only disk is soldered eMMC started no storage driver at all.
 */

#[path = "../../../../src/hardware/inventory/emmc.rs"]
pub mod emmc;
mod tests;
