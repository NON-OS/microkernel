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

//! Where a new thread may start, with no kernel dependencies so a host proof
//! can run it. The thread calls ask this before anything is built, so a start
//! the user entry builder would refuse is an EINVAL and not a thread that was
//! published and then could not run.

/// The last address of the user half: the bound the x86_64 user entry builder
/// holds a first instruction and a stack to, and the one user copies use.
pub(crate) const USER_VA_MAX: u64 = 0x0000_7FFF_FFFF_FFFF;

/// Whether a thread may start at `entry` on the stack whose top is `stack`:
/// both non-zero and inside the user half.
pub(crate) fn start_in_user_half(entry: u64, stack: u64) -> bool {
    entry != 0 && entry <= USER_VA_MAX && stack != 0 && stack <= USER_VA_MAX
}
