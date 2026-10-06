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
 * sendfile and copy_file_range: bytes from one descriptor to another
 * without passing through the guest.
 *
 * Both read a file through the same path read(2) does. sendfile writes to
 * a file or to the console; to a pipe or a socket it answers EINVAL, as
 * Linux answers for an output it cannot splice into, and every caller
 * then falls back to read and write, which reach those. copy_file_range
 * is between two files, as on Linux.
 */

mod bytes;
mod copy;
mod offset;
mod send;

pub use copy::copy_file_range;
pub use send::sendfile;
