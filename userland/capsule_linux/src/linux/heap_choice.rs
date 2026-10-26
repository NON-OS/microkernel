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
//! Which heap a process's arguments ask for, from what `mk_args` gave back.
//! An install's argv carries its market policy, 640 hex since I-C5, so it is
//! longer than the first buffer this read into. `mk_args` then copies
//! nothing and returns the length, so an install that read only that
//! length ran in the run heap. Pure, so the host proofs hold it.

/// Room for any argv init writes: an install's is about 720 bytes.
pub const ARGS_MAX: usize = 4096;

/// The first word of the arguments, when `n` bytes of them were copied
/// into `buf`; None when they were not (too long, or an error).
pub fn first_word(buf: &[u8], n: i64) -> Option<&[u8]> {
    let n = usize::try_from(n).ok().filter(|&n| n <= buf.len())?;
    buf[..n].split(|&b| b == 0).next()
}

/// Whether the arguments are an install's.
pub fn is_install(buf: &[u8], n: i64) -> bool {
    first_word(buf, n) == Some(b"install".as_slice())
}
