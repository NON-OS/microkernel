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
 * Lending /proc its view of the family, for the one call that may read it.
 *
 * Only the family knows which processes it holds and the numbers its pid
 * namespace gave them, and /proc must show exactly those and nothing else.
 * Built only for a call that names a path or reads a /proc descriptor, so
 * every other call costs one test.
 */

mod facts;
mod lend;
mod proc;
