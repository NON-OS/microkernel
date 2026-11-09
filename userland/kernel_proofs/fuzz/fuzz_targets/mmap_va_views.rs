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

//! Any run of reserves and releases on a process's mmap allocator, the
//! kernel's own src/process/mmap_va.rs mounted unchanged, held to a model of
//! the ranges handed out. A surface view's release (`release_guarded`) gives
//! back a range only when it is a live guarded range of exactly that base
//! and length; a plain release only an exact live range of either kind. No
//! range handed out, guards included, ever overlaps another, and once every
//! range is back the next reserve starts at the allocator's first address.

#![no_main]

use libfuzzer_sys::fuzz_target;

#[allow(clippy::new_without_default)]
#[path = "../../../../src/process/mmap_va.rs"]
mod mmap_va;
use mmap_va::MmapVa;

const PAGE: u64 = 4096;

#[derive(Clone, Copy, Debug)]
struct Live {
    base: u64,
    pages: u64,
    guarded: bool,
}

impl Live {
    /// The addresses it takes, its guard pages included.
    fn span(&self) -> (u64, u64) {
        let g = u64::from(self.guarded);
        (self.base - g * PAGE, self.base + (self.pages + g) * PAGE)
    }
}

fn disjoint(live: &[Live]) -> bool {
    let mut spans: Vec<(u64, u64)> = live.iter().map(Live::span).collect();
    spans.sort_unstable();
    spans.windows(2).all(|w| w[0].1 <= w[1].0)
}

fuzz_target!(|data: &[u8]| {
    let mut va = MmapVa::new();
    let mut live: Vec<Live> = Vec::new();
    let mut first = None;
    for op in data.chunks_exact(3).take(512) {
        let pages = u64::from(op[1] % 16);
        let pick = usize::from(op[2]);
        match op[0] % 6 {
            0 | 1 => {
                let guarded = op[0] % 6 == 1;
                let got = if guarded { va.reserve_guarded(pages) } else { va.reserve(pages) };
                assert_eq!(got.is_some(), pages != 0, "reserve of {pages} pages");
                if let Some(base) = got {
                    assert_eq!(base % PAGE, 0);
                    first.get_or_insert(base - u64::from(guarded) * PAGE);
                    live.push(Live { base, pages, guarded });
                }
            }
            2 | 3 => {
                // A release of a live range, its length right or off by some.
                let Some(&r) = live.get(pick % live.len().max(1)) else { continue };
                let pages = if op[0] % 6 == 2 { r.pages } else { r.pages + 1 + pages };
                let view = op[1] & 0x80 != 0;
                let want = pages == r.pages && (!view || r.guarded);
                let got = if view { va.release_guarded(r.base, pages) } else { va.release(r.base, pages) };
                assert_eq!(got, want, "{r:?} released as {pages} pages, view {view}");
                if got {
                    live.retain(|l| l.base != r.base);
                }
            }
            4 => {
                // A guard page, a page inside a range or one past it is never a base.
                let Some(&r) = live.get(pick % live.len().max(1)) else { continue };
                let off = [r.base.wrapping_sub(PAGE), r.base + PAGE, r.base + r.pages * PAGE];
                let at = off[usize::from(op[1]) % 3];
                if live.iter().any(|l| l.base == at) {
                    continue;
                }
                assert!(!va.release_guarded(at, r.pages), "{r:?} given back at {at:#x}");
                assert!(!va.release(at, r.pages), "{r:?} given back at {at:#x}");
            }
            _ => {
                // An address never handed out.
                let at = u64::from(op[1]) * PAGE + u64::from(op[2]);
                assert!(!va.release_guarded(at, pages.max(1)));
                assert!(!va.release(at, pages.max(1)));
            }
        }
        assert!(disjoint(&live), "{live:?}");
    }
    for r in live.drain(..) {
        assert!(va.release(r.base, r.pages), "{r:?} not given back at the end");
    }
    if let Some(first) = first {
        assert_eq!(va.reserve(1), Some(first), "the allocator kept a range");
    }
});
