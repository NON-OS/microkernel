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

//! The argv a tool is started with.

use alloc::vec::Vec;

use crate::jobs::box_hold::KEY_LEN;

/// argv as the tool sees it: argv[0] is the command name, then the key a
/// boxed run is given, then each argument, NUL-separated, which is what the
/// kernel splits into the process argv.
pub fn argv(args: &[&[u8]], key: Option<&[u8; KEY_LEN]>) -> Vec<u8> {
    let named = key.map(key_arg);
    let words =
        args.iter().take(1).copied().chain(named.as_deref()).chain(args.iter().skip(1).copied());
    let mut blob = Vec::new();
    for (i, a) in words.enumerate() {
        if i > 0 {
            blob.push(0);
        }
        blob.extend_from_slice(a);
    }
    blob
}

/// The word that names `key` to the run, ahead of `--box`.
pub fn key_arg(key: &[u8; KEY_LEN]) -> Vec<u8> {
    let mut word = b"--box-key=".to_vec();
    for b in key {
        word.push(b"0123456789abcdef"[usize::from(b >> 4)]);
        word.push(b"0123456789abcdef"[usize::from(b & 15)]);
    }
    word
}
