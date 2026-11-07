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

//! A wl_shm pool's pixels: found where its memfd is mapped now, from the
//! file offset it was mapped at, and read only at the buffer's own shape
//! inside the pool (guest/memfd_spans.rs, wayland/shm_fit.rs).

use super::super::random::Regs;
use crate::memfd_spans::{cut, grown, moved, reach, record, resolve, MemfdSpan};
use crate::shm_fit::{fits, read_len, MAX_SIDE};

const PAGE: u64 = 4096;

fn span(id: u32, at: u64, len: u64, off: u64) -> MemfdSpan {
    MemfdSpan { id, at, len, off }
}

/// A pool whose memfd the client mapped from a file offset: its buffers are
/// read from where those bytes landed, and nothing before or past them.
#[test]
fn a_pool_mapped_at_an_offset_is_found_from_that_offset() {
    let mut s = Vec::new();
    assert!(record(&mut s, span(7, 0x10_0000, 4 * PAGE, 2 * PAGE)));
    assert_eq!(resolve(&s, 7, 2 * PAGE, PAGE), Some(0x10_0000));
    assert_eq!(resolve(&s, 7, 3 * PAGE, 3 * PAGE), Some(0x10_1000));
    assert_eq!(resolve(&s, 7, 0, PAGE), None, "before the mapping");
    assert_eq!(resolve(&s, 7, 5 * PAGE, 2 * PAGE), None, "past its end");
    assert_eq!(reach(&s, 7), Some(6 * PAGE));
    assert_eq!(resolve(&s, 8, 2 * PAGE, PAGE), None, "another memfd");
}

/// Qt replacing a buffer as a window is resized: its mapping is gone before
/// the bridge reads the create_pool. Nothing is found, so nothing is read,
/// and the pool is made all the same (shm.rs).
#[test]
fn a_memfd_unmapped_before_its_pool_is_read_is_found_nowhere() {
    let mut s = Vec::new();
    record(&mut s, span(3, 0x40_0000, 16 * PAGE, 0));
    s = cut(&s, 0x40_0000, 16 * PAGE);
    assert_eq!(resolve(&s, 3, 0, PAGE), None);
    assert_eq!(reach(&s, 3), None);
}

/// The address an old pool was read at, mapped again for a wider one: the
/// old buffer no longer finds it, so it is never read at its own stride over
/// the new pool's rows.
#[test]
fn an_address_mapped_again_for_another_memfd_is_not_the_old_ones() {
    let (a, len) = (0x80_0000, 64 * PAGE);
    let mut s = Vec::new();
    record(&mut s, span(1, a, len, 0));
    s = cut(&s, a, len);
    record(&mut s, span(2, a, len * 2, 0));
    assert_eq!(resolve(&s, 1, 0, PAGE), None, "the old pool reads nothing");
    assert_eq!(resolve(&s, 2, 0, PAGE), Some(a));
    // MAP_FIXED over it without an unmap first: the record itself replaces.
    let mut s = Vec::new();
    record(&mut s, span(1, a, len, 0));
    record(&mut s, span(2, a, len, 0));
    assert_eq!(resolve(&s, 1, 0, PAGE), None);
    assert_eq!(s.len(), 1);
}

/// An unmap of a span's head, middle or tail leaves the rest naming the file
/// bytes it maps.
#[test]
fn a_split_span_keeps_its_file_offsets() {
    let a = 0x100_0000;
    let mut s = Vec::new();
    record(&mut s, span(9, a, 8 * PAGE, 0));
    s = cut(&s, a + 2 * PAGE, 2 * PAGE);
    assert_eq!(resolve(&s, 9, 0, 2 * PAGE), Some(a));
    assert_eq!(resolve(&s, 9, 4 * PAGE, 4 * PAGE), Some(a + 4 * PAGE));
    assert_eq!(resolve(&s, 9, PAGE, 4 * PAGE), None, "across the hole");
    s = cut(&s, a, PAGE);
    assert_eq!(resolve(&s, 9, PAGE, PAGE), Some(a + PAGE));
    assert_eq!(resolve(&s, 9, 0, PAGE), None);
    s = cut(&s, a + 7 * PAGE, PAGE);
    assert_eq!(resolve(&s, 9, 6 * PAGE, PAGE), Some(a + 6 * PAGE));
    assert_eq!(resolve(&s, 9, 7 * PAGE, PAGE), None);
}

/// libwayland-cursor maps a grown pool again: the latest mapping is read.
#[test]
fn the_latest_mapping_of_a_memfd_is_the_one_read() {
    let mut s = Vec::new();
    record(&mut s, span(4, 0x20_0000, 2 * PAGE, 0));
    record(&mut s, span(4, 0x30_0000, 4 * PAGE, 0));
    assert_eq!(resolve(&s, 4, 0, PAGE), Some(0x30_0000));
    assert_eq!(resolve(&s, 4, 0, 4 * PAGE), Some(0x30_0000));
    assert_eq!(reach(&s, 4), Some(4 * PAGE));
}

/// mremap: grown in place reaches further into the file; moved, the spans
/// go with it and the grown end is the same mapping.
#[test]
fn a_remapped_memfd_is_followed() {
    let a = 0x50_0000;
    let mut s = vec![span(5, a, 2 * PAGE, PAGE)];
    grown(&mut s, a, 2 * PAGE, 2 * PAGE);
    assert_eq!(resolve(&s, 5, PAGE, 4 * PAGE), Some(a));
    let to = 0x90_0000;
    let mut s = vec![span(5, a, 2 * PAGE, PAGE)];
    let at_new = moved(&s, a, 2 * PAGE, to, 3 * PAGE);
    s = cut(&s, a, 2 * PAGE);
    at_new.into_iter().for_each(|x| assert!(record(&mut s, x)));
    assert_eq!(resolve(&s, 5, PAGE, 3 * PAGE), Some(to));
    assert_eq!(resolve(&s, 5, PAGE, PAGE), Some(to));
}

/// Offsets and lengths are the guest's: none that wraps is recorded or found.
#[test]
fn no_span_or_lookup_wraps() {
    let mut s = Vec::new();
    assert!(!record(&mut s, span(1, u64::MAX - PAGE, 2 * PAGE, 0)));
    assert!(!record(&mut s, span(1, 0x1000, 2 * PAGE, u64::MAX - PAGE)));
    assert!(!record(&mut s, span(1, 0x1000, 0, 0)));
    assert!(s.is_empty());
    record(&mut s, span(1, 0x1000, PAGE, 0));
    assert_eq!(resolve(&s, 1, u64::MAX, 2), None);
    assert_eq!(resolve(&s, 1, 1, u64::MAX), None);
}

/// A span grown or moved by mremap keeps what `record` holds: neither its
/// end nor its file end wraps, so a later unmap inside it cuts it in place.
#[test]
fn a_remap_never_makes_a_span_that_wraps() {
    let near = u64::MAX - 0x1fff;
    let mut s = vec![span(1, 0x10_0000, PAGE, near)];
    grown(&mut s, 0x10_0000, PAGE, 3 * PAGE);
    assert_eq!(s, vec![span(1, 0x10_0000, PAGE, near)]);
    let mut s = vec![span(1, u64::MAX - 2 * PAGE, PAGE, 0)];
    grown(&mut s, u64::MAX - 2 * PAGE, PAGE, 4 * PAGE);
    assert_eq!(s[0].len, PAGE);
    let s = vec![span(1, 0x10_0000, PAGE, near)];
    assert!(moved(&s, 0x10_0000, PAGE, 0x20_0000, 4 * PAGE).is_empty());
    assert!(moved(&s, 0x10_0000, PAGE, u64::MAX - PAGE / 2, PAGE).is_empty());
    let s = vec![span(1, 0x10_0000, PAGE, near)];
    assert_eq!(moved(&s, 0x10_0000, PAGE, 0x20_0000, PAGE), vec![span(1, 0x20_0000, PAGE, near)]);
    for x in moved(&s, 0x10_0000, PAGE, 0x20_0000, PAGE) {
        assert!(x.at.checked_add(x.len).is_some() && x.off.checked_add(x.len).is_some());
        let _ = cut(&[x], x.at + 1, 1);
    }
}

/// Over any record and cut: spans never overlap, and an address found lies
/// inside a span of that memfd, at the file offset asked for, the whole
/// length within it.
#[test]
fn a_found_address_is_always_the_memfds_own() {
    let mut r = Regs::new(0x5A4D_F00D);
    for _ in 0..2_000 {
        let mut s: Vec<MemfdSpan> = Vec::new();
        for _ in 0..24 {
            let at = r.small(64) * PAGE;
            let len = (r.small(8) + 1) * PAGE;
            match r.small(3) {
                0 => s = cut(&s, at, len),
                _ => {
                    let id = r.small(4) as u32;
                    let _ = record(&mut s, span(id, at, len, r.small(8) * PAGE));
                }
            }
            for (i, x) in s.iter().enumerate() {
                for y in &s[i + 1..] {
                    assert!(x.at + x.len <= y.at || y.at + y.len <= x.at, "{x:?} {y:?}");
                }
            }
        }
        for _ in 0..32 {
            let (id, off, len) = (r.small(4) as u32, r.small(16) * 512, r.small(4) * 1000 + 1);
            if let Some(at) = resolve(&s, id, off, len) {
                let hit = s.iter().any(|x| {
                    x.id == id
                        && x.at <= at
                        && at + len <= x.at + x.len
                        && x.off + (at - x.at) == off
                });
                assert!(hit, "{id} {off} {len} at {at:#x} in {s:?}");
            }
        }
    }
}

/// The frame Telegram showed: a buffer whose rows are read at a stride under
/// its width, every row shifted from the one above. Never read.
#[test]
fn a_sheared_buffer_is_not_read() {
    let size = 800 * 600 * 4;
    assert_eq!(read_len(0, 800, 600, 3200, size), Some(size));
    assert_eq!(read_len(0, 801, 600, 3200, size), None, "stride under its width");
    assert_eq!(read_len(0, 800, 600, 3199, size), None);
    assert_eq!(read_len(0, 800, 600, 0, size), None);
}

/// A buffer past its pool, made so or left so by a pool that shrank, and
/// one longer than MAX_SIDE: never read.
#[test]
fn an_oversized_buffer_is_not_read() {
    let size = 800 * 600 * 4;
    assert_eq!(read_len(4, 800, 600, 3200, size), None, "four bytes past");
    assert_eq!(read_len(0, 800, 601, 3200, size), None);
    assert_eq!(read_len(0, 800, 600, 3200, size - 1), None, "its pool shrank");
    assert_eq!(read_len(u64::MAX - 2, 1, 1, 4, u64::MAX), None, "an offset that wraps");
    let side = MAX_SIDE as u32 + 1;
    assert_eq!(read_len(0, side, 1, side * 4, u64::MAX), None);
    assert_eq!(read_len(0, 1, side, 4, u64::MAX), None);
    assert_eq!(read_len(0, u32::MAX, u32::MAX, u32::MAX, u64::MAX), None);
}

/// Over any buffer and pool: a read is the buffer's own stride times its
/// height, its rows as wide as its width at least, inside the pool.
#[test]
fn every_read_is_the_buffers_own_shape_inside_its_pool() {
    let mut r = Regs::new(0x00C0_FFEE);
    for _ in 0..200_000 {
        let (w, h) = (r.small(20_000) as u32, r.small(20_000) as u32);
        let stride = match r.small(2) {
            0 => w.wrapping_mul(4).wrapping_add(r.small(64) as u32),
            _ => r.arg() as u32,
        };
        let (offset, size) = (r.small(1 << 20), r.small(1 << 32));
        if let Some(n) = read_len(offset, w, h, stride, size) {
            assert!(w > 0 && h > 0 && u64::from(w) <= MAX_SIDE && u64::from(h) <= MAX_SIDE);
            assert!(u64::from(stride) >= u64::from(w) * 4, "{w} {stride}");
            assert_eq!(n, u64::from(stride) * u64::from(h));
            assert!(offset + n <= size);
            assert!(fits(offset, w.into(), h.into(), stride.into(), size));
        }
    }
}
