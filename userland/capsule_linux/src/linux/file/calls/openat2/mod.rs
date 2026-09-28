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
 * openat2: openat with a struct open_how, and RESOLVE_ flags that limit
 * how the path may be walked.
 *
 * Served: NO_XDEV, NO_MAGICLINKS, NO_SYMLINKS, BENEATH and IN_ROOT, each
 * checked on the same walk open makes (walk/), and CACHED, which Linux
 * may always answer with EAGAIN and so does here.
 */

mod check;
mod how;
mod open;
mod rooted;
mod walked;

pub use open::openat2;
