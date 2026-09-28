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
 * Served: NO_XDEV, NO_MAGICLINKS, NO_SYMLINKS and BENEATH, which a walk of
 * the family's tree can check; CACHED, which Linux may always answer with
 * EAGAIN and so does here. IN_ROOT would re-root every link's target at
 * the directory, which this resolver does not do: it is refused with
 * EINVAL, as a kernel refuses a flag it does not know.
 */

mod check;
mod how;
mod open;
mod walked;

pub use open::openat2;
