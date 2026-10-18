// NONOS Operating System (AGPL-3.0-or-later)
//! The lines a span touches, held to a brute force over every start, length
//! and line size here, for the kernel's flush (`cache_span::lines`) and
//! libc's streaming syncs (`dma_sync_lines_of`), which must agree.

use crate::cache_span::lines;
use crate::dma_lines::{ctr_dminline_bytes, dma_sync_lines_of};

const PAGE: u64 = 4096;

/// The line indices `[start, start + len)` touches, byte by byte.
fn touched(start: u64, len: u64, line: u64) -> Vec<u64> {
    let mut out: Vec<u64> = (start..start + len).map(|b| b / line).collect();
    out.dedup();
    out
}

#[test]
fn every_line_touched_and_no_other() {
    for line in [16u64, 32, 64, 128] {
        for start in 0..300u64 {
            for len in 0..300u64 {
                let want = touched(start, len, line);
                let got = lines(start, len, line).map(|(first, n)| (first / line, n));
                match want.first() {
                    None => assert_eq!(got, None, "empty {start}+{len}"),
                    Some(&f) => {
                        assert_eq!(got, Some((f, want.len() as u64)), "{start}+{len}/{line}")
                    }
                }
                let (first, n) = dma_sync_lines_of(start, len, line);
                assert_eq!(n, want.len() as u64, "libc {start}+{len}/{line}");
                if n != 0 {
                    assert_eq!(first / line, want[0]);
                }
            }
        }
    }
}

#[test]
fn wraps_and_bad_line_sizes_touch_nothing() {
    assert_eq!(lines(u64::MAX - 10, 20, 64), None);
    assert_eq!(lines(u64::MAX - 63, 64, 64), None, "the last line's end wraps");
    assert_eq!(lines(0, 64, 48), None);
    assert_eq!(lines(0, 64, 0), None);
    assert_eq!(dma_sync_lines_of(u64::MAX - 10, 20, 64).1, 0);
}

/// A grant is whole pages, so every line it touches lies inside it: no line
/// is shared with memory the grant does not own, which is what makes clean
/// and invalidate safe on it.
#[test]
fn a_grant_owns_every_line_it_touches() {
    for line in [16u64, 32, 64, 128, 256] {
        for pages in 1..8u64 {
            let start = 0x4000_0000 + pages * PAGE;
            let (first, n) = lines(start, pages * PAGE, line).unwrap();
            assert_eq!((first, first + n * line), (start, start + pages * PAGE));
        }
    }
}

#[test]
fn the_line_size_is_ctr_el0_dminline_in_words() {
    for log2_words in 0..16u64 {
        assert_eq!(ctr_dminline_bytes(log2_words << 16 | 0xF), 4 << log2_words);
    }
}
