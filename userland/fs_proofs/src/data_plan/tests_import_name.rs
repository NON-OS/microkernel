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
 * Which names a write from outside the import path may never touch, and
 * where it looks for the record that makes a file the import's.
 */

use super::import_name::{is_kept, record_of, trimmed};

#[test]
fn records_and_marks_are_kept_however_they_are_spelled() {
    for name in [
        &b"/qwen.gguf.sha256"[..],
        b"/qwen.gguf.partial",
        b"qwen.gguf.sha256",
        b"//qwen.gguf.sha256//",
        b"/dir/qwen.gguf.partial/",
        b".sha256",
    ] {
        assert!(is_kept(name), "{:?}", core::str::from_utf8(name));
    }
}

#[test]
fn ordinary_names_are_not_kept() {
    for name in
        [&b"/qwen.gguf"[..], b"/qwen.sha256x", b"/sha256", b"/qwen.partial.gguf", b"", b"///"]
    {
        assert!(!is_kept(name), "{:?}", core::str::from_utf8(name));
    }
}

#[test]
fn the_record_sits_beside_the_name_the_volume_resolves() {
    assert_eq!(record_of(b"/qwen.gguf"), b"/qwen.gguf.sha256");
    assert_eq!(record_of(b"/qwen.gguf//"), b"/qwen.gguf.sha256");
    assert_eq!(record_of(b"//qwen.gguf"), b"//qwen.gguf.sha256");
    assert_eq!(record_of(b"dir/qwen.gguf/"), b"dir/qwen.gguf.sha256");
}

#[test]
fn trailing_slashes_are_dropped_and_nothing_else() {
    assert_eq!(trimmed(b"/a/b//"), b"/a/b");
    assert_eq!(trimmed(b"//a"), b"//a");
    assert_eq!(trimmed(b"///"), b"");
    assert_eq!(trimmed(b""), b"");
}
