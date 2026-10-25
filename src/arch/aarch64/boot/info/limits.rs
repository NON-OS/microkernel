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

//! Fixed storage sizes for what `BootInfo` collects before the heap exists.

/// As many regions as the device tree walker reads in one pass.
pub(super) const MAX_MEMORY_REGIONS: usize = 8;

/// As many CPUs as the device tree walker reads in one pass.
pub(super) const MAX_CPUS: usize = 64;
