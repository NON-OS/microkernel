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

//! An install's argv is about 720 bytes since I-C5 put its policy in it, and
//! mk_args copies nothing into a buffer shorter than the argv: an install
//! read through a 256-byte buffer ran in the 64 MiB run heap, and
//! vim-common's unpack died there (booted, code 134).

use crate::heap_choice::{first_word, is_install, ARGS_MAX};

fn argv(words: &[&[u8]]) -> Vec<u8> {
    words.join(&0u8)
}

#[test]
fn an_install_argv_of_its_real_length_picks_the_install_heap() {
    let m = [b'a'; 640];
    let pin = [b'0'; 64];
    let a = argv(&[b"install", b"vim", &pin, &m, b"direct"]);
    assert!(a.len() > 256, "the length that broke it: {}", a.len());
    assert!(ARGS_MAX >= 4 * a.len(), "heap.rs reads the argv into ARGS_MAX bytes");
    let mut buf = [0u8; ARGS_MAX];
    buf[..a.len()].copy_from_slice(&a);
    assert!(is_install(&buf, a.len() as i64));
    assert_eq!(first_word(&buf, a.len() as i64), Some(&b"install"[..]));
}

#[test]
fn what_mk_args_did_not_copy_is_no_install() {
    let mut small = [0u8; 256];
    /* mk_args returned the length and copied nothing: the buffer is zeros. */
    assert!(!is_install(&small, 720));
    assert_eq!(first_word(&small, 720), None);
    assert_eq!(first_word(&small, -14), None, "an error copies nothing");
    small[..3].copy_from_slice(b"run");
    assert!(!is_install(&small, 3), "a run is not an install");
    assert!(!is_install(b"installer\0x", 11), "the whole first word");
}
