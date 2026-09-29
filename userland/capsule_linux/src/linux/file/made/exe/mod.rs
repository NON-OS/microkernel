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
 * What each program of the family was started as: the file, its name, and
 * where its arguments and environment lie in its own memory.
 *
 * Recorded when an image is built, for both a first start and an exec,
 * and kept by kernel pid for the life of the personality, which is the life
 * of the family. A forked child has no record of its own until it execs:
 * it runs its parent's image, so /proc answers for it from the parent's.
 */

mod shape;
mod table;

pub use shape::Exe;
pub use table::{comm_of, of, record};
