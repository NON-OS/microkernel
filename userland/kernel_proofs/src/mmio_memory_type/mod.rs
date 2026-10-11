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
 * The memory type a capsule's MMIO mapping gets: uncached unless it asks for
 * write-combining on a prefetchable BAR, and always the kernel's own type
 * where the kernel maps the range. The kernel's pure rule is included by
 * path.
 */

#[path = "../../../../src/hardware/broker/mmio/memory_type.rs"]
pub mod memory_type;
mod tests;
