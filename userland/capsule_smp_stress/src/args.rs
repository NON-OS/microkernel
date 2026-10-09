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

//! How long to run: the first number on the command line, in seconds. One
//! hour when none is given, the length a release stress run is held to.

pub const DEFAULT_SECONDS: u64 = 3600;

pub fn seconds() -> u64 {
    let mut buf = [0u8; 128];
    let n = nonos_libc::mk_args(buf.as_mut_ptr(), buf.len());
    if n <= 0 {
        return DEFAULT_SECONDS;
    }
    crate::number::first_number(&buf[..(n as usize).min(buf.len())]).unwrap_or(DEFAULT_SECONDS)
}
