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
//! Finding the xHCI controllers. A laptop often has more than one: the
//! chipset's (Intel PCH, AMD FCH), a Thunderbolt or USB4 controller beside
//! it, and AMD Ryzen APUs carry two of their own. Every one found is
//! returned, the chipset's first, so it is the primary and keeps root port
//! numbers from 1: the ports a boot stick, keyboard and mouse sit on.
mod find;
mod found;
mod thunderbolt;
pub use find::find_all_xhci;
pub use found::Found;
