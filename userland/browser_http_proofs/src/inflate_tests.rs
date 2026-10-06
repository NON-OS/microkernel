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

//! nonos_inflate's partial entry points: output so far, how it ended, and
//! how much input it read, on a real page and on hand-built streams.

use nonos_inflate::{gunzip, gunzip_partial, raw_partial, zlib, zlib_partial, End};

use crate::vectors::{read, zeros_deflate, Bits};

#[test]
fn a_real_page_inflates_whole_and_every_prefix_is_a_prefix() {
    let (gz, page) = (read("kernel.gz"), read("kernel.out"));
    assert_eq!(gunzip(&gz).as_deref(), Some(&page[..]));
    let full = gunzip_partial(&gz, usize::MAX);
    assert_eq!((full.end, full.used, full.out == page), (End::Complete, gz.len(), true));
    for cut in (0..gz.len()).step_by(7) {
        let p = gunzip_partial(&gz[..cut], usize::MAX);
        assert_eq!(p.end, End::Truncated, "cut at {cut}");
        assert!(page.starts_with(&p.out) && p.used <= cut, "cut at {cut}");
    }
    let capped = gunzip_partial(&gz, 1000);
    assert_eq!((capped.end, &capped.out[..]), (End::Capped, &page[..1000]));
}

#[test]
fn zlib_checks_its_adler32() {
    let mut z = vec![0x78, 0x01, 1, 3, 0, 0xfc, 0xff, b'a', b'b', b'c'];
    assert_eq!(zlib_partial(&z, 99).end, End::Truncated, "no checksum yet");
    z.extend_from_slice(&0x024d_0127u32.to_be_bytes());
    assert_eq!(zlib(&z).as_deref(), Some(&b"abc"[..]));
    *z.last_mut().unwrap() ^= 1;
    assert_eq!(zlib_partial(&z, 99).end, End::Corrupt);
}

#[test]
fn malformed_codes_and_distances_are_corrupt() {
    let mut over = Bits::default();
    over.put(1, 1);
    over.put(2, 2);
    over.put(0, 10);
    over.put(15, 4);
    for _ in 0..19 {
        over.put(1, 3);
    }
    assert_eq!(raw_partial(&over.out, 99).end, End::Corrupt, "over-subscribed code");
    let mut far = Bits::default();
    far.put(3, 3);
    far.put(0xC5u32.reverse_bits() >> 24, 8);
    far.put(0, 5);
    assert_eq!(raw_partial(&far.out, 99).end, End::Corrupt, "distance before the start");
}

#[test]
fn the_cap_stops_a_bomb_early_and_says_how_little_was_read() {
    let bomb = zeros_deflate(400_000);
    let r = raw_partial(&bomb, 4 << 20);
    assert_eq!((r.end, r.out.len()), (End::Capped, 4 << 20));
    assert!(r.used * 16 < bomb.len(), "read {} of {}", r.used, bomb.len());
    assert!(nonos_inflate::inflate(&bomb).is_none(), "the whole-stream API refuses it");
}
