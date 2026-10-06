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

//! data: URIs: the payload is percent-decoded before a ;base64 payload is
//! read as forgiving base64, and a path that never advanced (the SVG
//! closepath the URI fuzzer found) no longer stalls the decoder.
use std::time::Instant;

use crate::browser::image::{ingest, Store};
use crate::fixtures::read;
use crate::shim::data_uri;

#[test]
fn percent_encoded_base64_decodes_like_the_plain_form() {
    let uri = std::string::String::from_utf8(read("misc/base64_percent_encoded.uri")).unwrap();
    assert!(uri.contains("%2B") || uri.contains("%2F"));
    let plain = uri.replace("%2B", "+").replace("%2F", "/").replace("%3D", "=");
    let bytes = data_uri(uri.trim()).expect("percent-encoded base64 decodes");
    assert_eq!(Some(bytes.clone()), data_uri(plain.trim()));
    assert!(bytes.starts_with(&[0x89, b'P', b'N', b'G']));
}

#[test]
fn forgiving_base64_skips_whitespace_and_padding() {
    let want = Some(b"foobar".to_vec());
    assert_eq!(data_uri("data:text/plain;base64,Zm9v\nYmFy"), want);
    assert_eq!(data_uri("data:;BASE64,Zm9v\u{c}Ym%46y"), want);
    assert_eq!(data_uri("data:;base64,Zm9vYg%3D%3D"), Some(b"foob".to_vec()));
    assert_eq!(data_uri("data:;base64,Zm9vYg"), Some(b"foob".to_vec()));
    assert_eq!(data_uri("data:;base64,Zm9v%FF"), None);
    assert_eq!(data_uri("data:image/svg+xml,%3Csvg%3E"), Some(b"<svg>".to_vec()));
}

#[test]
fn an_svg_path_that_never_advanced_finishes() {
    let uri =
        std::string::String::from_utf8_lossy(&read("misc/svg_closepath_hang.uri")).into_owned();
    let t = Instant::now();
    let bytes = data_uri(&uri).expect("plain payload");
    let mut store = Store::new();
    ingest(&mut store, "data:hang", &bytes);
    let tail = "data:image/svg+xml,<svg width='4' height='4'><path d='M0 0h4v4z 9 9'/></svg>";
    ingest(&mut store, "data:tail", &data_uri(tail).unwrap());
    assert!(t.elapsed().as_millis() < 500, "took {:?}", t.elapsed());
    assert!(store.ready("data:tail").is_some());
}
