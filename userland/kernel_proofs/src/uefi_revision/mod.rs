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
 * UEFI revisions use the specification's packing.
 *
 * The kernel's revision constants and firmware record come from the UEFI
 * modules the crate already includes by path. The constants packed the minor
 * version into the upper byte of the lower half, so 2.8 read back as minor
 * 2048, while 2.3.1 alone used the specification's tens-and-units form and so
 * sorted below 2.1. The checks below fail against that code.
 */

mod tests;
