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

//! The argv a boxed run is started with: its key named first, ahead of the
//! words as typed; every other run's argv as typed.

use super::box_hold::KEY_LEN;
use super::linux_frames::key_of;
use super::tool_argv::argv;

const KEY: [u8; KEY_LEN] = [0xa5; KEY_LEN];

#[test]
fn a_boxed_run_names_its_key_first_and_every_other_run_is_as_typed() {
    let words: [&[u8]; 4] = [b"linux", b"--box", b"strict", b"sh"];
    let named = argv(&words, Some(&KEY));
    let parts: Vec<&[u8]> = named.split(|b| *b == 0).collect();
    assert_eq!(parts[0], b"linux");
    assert_eq!(key_of(parts[1]), Some(KEY));
    assert_eq!(&parts[2..], &words[1..]);
    assert_eq!(argv(&words, None), b"linux\0--box\0strict\0sh");
    assert_eq!(argv(&[b"linux"], None), b"linux");
}
