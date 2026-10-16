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
 * The load average's weights and fold (L56), included by path: each weight
 * is ONE * e^(-sample / window) to the nearest unit, worked here in floating
 * point from the definition; a steady load reads back exactly; an idle
 * machine's average falls to nothing it would print.
 */

#[path = "../../../../src/fs/procfs/loadavg_math.rs"]
mod loadavg_math;
mod periodic_tests;
mod tests;
