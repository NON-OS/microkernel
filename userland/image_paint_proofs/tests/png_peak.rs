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

//! Heap bound of the PNG decoder, in its own test binary so the counting
//! allocator sees this one decode and nothing else. The 2000x2000 RGBA file
//! (4,000,000 pixels, the browser's raster ceiling) used to stage 12 bytes a
//! pixel in a 48 MiB heap; beyond the caller's 4 B/px output it may now hold
//! two scanlines, the 32 KiB inflate window and small tables.
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

use nonos_toolkit::image::png::decoder::decode_png_argb8888;

struct Counting;
static LIVE: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        let now = LIVE.fetch_add(l.size(), Relaxed) + l.size();
        PEAK.fetch_max(now, Relaxed);
        unsafe { System.alloc(l) }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        LIVE.fetch_sub(l.size(), Relaxed);
        unsafe { System.dealloc(p, l) }
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

#[test]
fn a_4_megapixel_png_decodes_in_its_output_plus_under_128_kib() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/misc/png_rgba_2000x2000_diagram.png");
    let file = std::fs::read(path).expect("fixture");
    let mut out = vec![0u32; 2000 * 2000];
    let base = LIVE.load(Relaxed);
    PEAK.store(base, Relaxed);
    let size = decode_png_argb8888(&file, &mut out).expect("decodes");
    let extra = PEAK.load(Relaxed) - base;
    assert_eq!((size.width, size.height), (2000, 2000));
    assert!(extra < 128 * 1024, "decoder working set {extra} bytes");
    assert_eq!(LIVE.load(Relaxed), base, "everything the decoder took is returned");
}
