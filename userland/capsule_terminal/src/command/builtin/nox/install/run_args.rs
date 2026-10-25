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

//! The capsule name `install` takes, and the argv it hands the loaded program.

pub(super) fn valid_name(stem: &[u8]) -> bool {
    !stem.is_empty()
        && stem.len() <= 64
        && stem.iter().all(|&b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
}

// argv[0] is the program name (the capsule stem), matching the POSIX
// convention upstream CLI parsers expect; the user args follow.
pub(super) fn argv_blob(name: &[u8], args: &[&[u8]]) -> alloc::vec::Vec<u8> {
    let mut out = alloc::vec::Vec::new();
    out.extend_from_slice(name);
    for arg in args {
        out.push(0);
        out.extend_from_slice(arg);
    }
    out
}
