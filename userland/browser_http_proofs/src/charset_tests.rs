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

//! Every decoder of the Encoding Standard against vectors made by a direct
//! port of the standard's algorithms over its published index files: all
//! 256 bytes of each single-byte encoding; for each multi-byte one, 1,000
//! valid sequences and 200 short, mostly malformed inputs.

use crate::browser::http::response::charset::{encoding, Encoding, WINDOWS_1252};
use crate::vectors::{path, read, records};

#[test]
fn every_encoding_decodes_its_vectors() {
    let mut files: Vec<_> =
        std::fs::read_dir(path("encodings")).unwrap().map(|e| e.unwrap().path()).collect();
    files.sort();
    assert_eq!(files.len(), 28 + 1 + 8, "single-byte, x-user-defined, multi-byte");
    for f in files {
        let label = f.file_stem().unwrap().to_str().unwrap().to_string();
        let enc = encoding(label.as_bytes()).unwrap_or_else(|| panic!("{label}"));
        let data = read(&format!("encodings/{label}.vec"));
        for (k, pair) in records(&data).chunks(2).enumerate() {
            let mut got = String::new();
            enc.decode(pair[0], &mut got);
            assert_eq!(got.as_bytes(), pair[1], "{label} case {k}");
        }
    }
}

#[test]
fn labels_resolve_as_the_standard_lists_them() {
    let e = |l: &str| encoding(l.as_bytes());
    assert_eq!(e(" Latin1\t"), Some(WINDOWS_1252));
    assert_eq!(e("windows-1252"), Some(WINDOWS_1252));
    assert_eq!(e("US-ASCII"), Some(WINDOWS_1252));
    assert_eq!(e("x-sjis"), Some(Encoding::ShiftJis));
    assert_eq!(e("windows-31j"), Some(Encoding::ShiftJis));
    assert_eq!(e("gb2312"), Some(Encoding::Gb18030));
    assert_eq!(e("big5-hkscs"), Some(Encoding::Big5));
    assert_eq!(e("ks_c_5601-1987"), Some(Encoding::EucKr));
    assert_eq!(e("utf-16"), Some(Encoding::Utf16Le));
    assert_eq!(e("iso-2022-kr"), Some(Encoding::Replacement));
    assert_eq!((e("utf-7"), e(""), e("koi8")), (None, None, Some(Encoding::Single(14))));
}

#[test]
fn replacement_and_user_defined_decode_as_specified() {
    let run = |enc: Encoding, b: &[u8]| {
        let mut s = String::new();
        enc.decode(b, &mut s);
        s
    };
    assert_eq!(run(Encoding::Replacement, b"any bytes"), "\u{FFFD}");
    assert_eq!(run(Encoding::Replacement, b""), "");
    assert_eq!(run(Encoding::XUserDefined, b"a\x80\xff"), "a\u{F780}\u{F7FF}");
}
