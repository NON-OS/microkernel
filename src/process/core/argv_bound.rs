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

//! How long an argv may be, as `MkArgs` hands it back: the words joined with
//! a NUL between each. Every place that sets a process's argv holds to it
//! (`argv::set_argv`), and userland reads argv into a buffer of the same size
//! (`nonos_libc::ARGV_BYTES_MAX`, pinned equal by scripts/check_argv_bound.py).
//! An install's argv carries its policy as 640 hex since I-C5, about 720 bytes;
//! read into 256 it came back empty and the install ran in the run heap. Pure,
//! so the host proofs hold it.

use alloc::string::String;

/// The most bytes an argv may join to.
pub const ARGV_BYTES_MAX: usize = 4096;

/// The bytes `MkArgs` would return for `words`.
pub fn joined_len(words: &[String]) -> usize {
    words.iter().map(String::len).sum::<usize>() + words.len().saturating_sub(1)
}

/// Whether `words` fit the bound.
pub fn fits(words: &[String]) -> bool {
    joined_len(words) <= ARGV_BYTES_MAX
}
