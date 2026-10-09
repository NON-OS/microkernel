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
 * /proc/<pid>/fd and fdinfo: each descriptor, named as Linux names it.
 *
 * A file or directory is its path. What has no path is named by kind and
 * inode as Linux does: a pipe `pipe:[n]`, a socket `socket:[n]`, and the
 * objects with no file behind them `anon_inode:[kind]`. The console is a
 * stream with no terminal behind it, as a pipe is, so it is shown as one.
 */

mod info;
mod list;

pub use info::{info, target_of};
pub use list::{open_fds, CONSOLE_IN, CONSOLE_OUT, PIPES, SOCKETS};
