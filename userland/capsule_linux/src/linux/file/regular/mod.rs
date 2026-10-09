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
 * Opening a regular file, and creating one that is not there yet.
 *
 * A write-only or truncating open needs no stream from the store: nothing
 * will be read from what is there. A write goes to the family's copy of
 * the file (held/cache/), which is taken when the first write needs it.
 */

mod create;
mod open;

pub use create::create;
pub use open::open;
