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
//! HTTP/1.1 for a client, with no I/O of its own.

//! Chunked bodies decoded as they come, in any split.

use nonos_download::{ChunkError, Chunked};

const WIRE: &[u8] = b"5\r\nhello\r\n7;ext=1\r\n, world\r\n0\r\nX-Trailer: yes\r\n\r\n";

#[test]
fn a_chunked_body_comes_out_whole_however_it_is_split() {
    for split in 1..WIRE.len() {
        let mut c = Chunked::new();
        let mut out = Vec::new();
        for piece in WIRE.chunks(split) {
            c.feed(piece, &mut out).unwrap();
        }
        assert_eq!(out, b"hello, world", "split {split}");
        assert!(c.done(), "split {split}");
    }
}

#[test]
fn a_bad_size_or_a_missing_crlf_is_an_error_not_a_guess() {
    let mut out = Vec::new();
    assert_eq!(Chunked::new().feed(b"zz\r\n", &mut out), Err(ChunkError::BadSize));
    assert_eq!(Chunked::new().feed(b"3\r\nabcXY", &mut out), Err(ChunkError::BadEnd));
    let long = vec![b'f'; 200];
    assert_eq!(Chunked::new().feed(&long, &mut out), Err(ChunkError::BadSize));
}

#[test]
fn a_large_chunk_size_is_carried_across_many_reads() {
    let mut c = Chunked::new();
    let mut out = Vec::new();
    c.feed(b"100000\r\n", &mut out).unwrap();
    for _ in 0..256 {
        c.feed(&[7u8; 4096], &mut out).unwrap();
    }
    c.feed(b"\r\n0\r\n\r\n", &mut out).unwrap();
    assert_eq!(out.len(), 0x100000);
    assert!(c.done());
}
