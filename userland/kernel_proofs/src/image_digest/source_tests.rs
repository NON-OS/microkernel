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

use super::digest::{in_chunks, DigestError, CHUNK};
use std::cell::{Cell, RefCell};

const UNIT: usize = 64 << 10;

#[test]
fn a_source_that_fails_part_way_gives_no_digest() {
    let len = 3 * CHUNK + 5;
    for fail_at in [0, CHUNK, 2 * CHUNK, 3 * CHUNK] {
        let read = |at: u64, buf: &mut [u8]| -> Result<(), ()> {
            buf.fill(0x5a);
            if at == fail_at as u64 {
                return Err(());
            }
            Ok(())
        };
        let got = in_chunks(len as u64, UNIT, read, || {});
        assert_eq!(got, Err(DigestError::Unreadable), "failed at {fail_at}");
    }
}

#[test]
fn the_source_is_read_in_order_a_chunk_at_most_with_a_serve_before_each() {
    let len = 3 * CHUNK + 777;
    let serves = Cell::new(0usize);
    let reads = RefCell::new(Vec::new());
    let read = |at: u64, buf: &mut [u8]| -> Result<(), ()> {
        let mut reads = reads.borrow_mut();
        let last = reads.last().map_or(0, |&(_, _, served)| served);
        assert!(serves.get() > last, "a read came with no serve before it");
        reads.push((at, buf.len(), serves.get()));
        Ok(())
    };
    in_chunks(len as u64, UNIT, read, || serves.set(serves.get() + 1)).unwrap();
    let reads: Vec<_> = reads.into_inner().into_iter().map(|(at, n, _)| (at, n)).collect();
    let chunk = CHUNK as u64;
    assert_eq!(reads, [(0, CHUNK), (chunk, CHUNK), (2 * chunk, CHUNK), (3 * chunk, 777)]);
    assert!(serves.get() >= len.div_ceil(UNIT), "{} serves for {len} bytes", serves.get());
}
