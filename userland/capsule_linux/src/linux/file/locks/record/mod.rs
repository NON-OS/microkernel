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
 * fcntl's record locks: F_GETLK, F_SETLK, F_SETLKW and their OFD forms.
 *
 * struct flock on x86_64: l_type and l_whence as shorts, then l_start,
 * l_len as 64-bit offsets, then l_pid; 32 bytes.
 */

mod cmds;
mod fcntl;
mod range;
mod want;

pub use cmds::is_lock;
pub use fcntl::fcntl_lock;
