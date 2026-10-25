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

//! The two kinds of shared pages read and write the same bytes, and the
//! mapped kind refuses an access past its end.

use std::vec;

use crate::mem::{Buffer, Pages, Shared};

fn both(f: impl Fn(&mut dyn Shared)) {
    let mut backing = vec![0u8; 64];
    // SAFETY: `backing` outlives `p` and nothing else touches it meanwhile.
    let mut p = unsafe { Pages::new(backing.as_mut_ptr(), backing.len()) };
    f(&mut p);
    let mut b = Buffer(vec![0u8; 64]);
    f(&mut b);
    assert_eq!(backing, b.0, "the same bytes either way");
}

#[test]
fn bytes_and_indexes_land_where_they_are_put() {
    both(|m| {
        assert_eq!((m.len(), m.is_empty()), (64, false));
        m.write(10, &[1, 2, 3]);
        let mut out = [0u8; 4];
        m.read(9, &mut out);
        assert_eq!(out, [0, 1, 2, 3]);
        m.store32(20, 0xA1B2_C3D4);
        assert_eq!(m.load32(20), 0xA1B2_C3D4);
        let mut le = [0u8; 4];
        m.read(20, &mut le);
        assert_eq!(
            le,
            [0xD4, 0xC3, 0xB2, 0xA1],
            "little endian, as the host reads it"
        );
        m.barrier();
    });
}

#[test]
fn no_pages_are_empty() {
    assert!(Buffer(vec![]).is_empty());
    let mut none = [0u8; 0];
    // SAFETY: zero bytes.
    assert!(unsafe { Pages::new(none.as_mut_ptr(), 0) }.is_empty());
}

fn pages(len: usize) -> (std::vec::Vec<u8>, Pages) {
    let mut v = vec![0u8; 64];
    // SAFETY: the vector is returned beside the pages and outlives their use.
    let p = unsafe { Pages::new(v.as_mut_ptr(), len) };
    (v, p)
}

#[test]
#[should_panic]
fn a_read_past_the_end_panics() {
    let (_v, p) = pages(16);
    p.read(13, &mut [0u8; 4]);
}

#[test]
#[should_panic]
fn a_write_past_the_end_panics() {
    let (_v, mut p) = pages(16);
    p.write(13, &[0u8; 4]);
}

#[test]
#[should_panic]
fn an_index_past_the_end_panics() {
    let (_v, p) = pages(16);
    p.load32(16);
}

#[test]
#[should_panic]
fn an_index_stored_past_the_end_panics() {
    let (_v, mut p) = pages(16);
    p.store32(16, 1);
}
