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


use super::map_loop::{map_loop, MapOps, WINDOW_PAGES};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Step {
    Serve,
    Window,
    Alloc(u32),
    Zero(u32),
    Map(usize, u32),
    Free(u32),
    Rollback(usize),
}

#[derive(Default)]
struct Rec {
    steps: Vec<Step>,
    next: u32,
    /// The allocation, or the map, that fails: by page index.
    fail_alloc: Option<usize>,
    fail_map: Option<usize>,
    allocs: usize,
}

impl MapOps for Rec {
    type Frame = u32;
    fn serve_shootdowns(&mut self) {
        self.steps.push(Step::Serve);
    }
    fn window(&mut self) {
        self.steps.push(Step::Window);
    }
    fn allocate(&mut self) -> Option<u32> {
        let i = self.allocs;
        self.allocs += 1;
        if self.fail_alloc == Some(i) {
            return None;
        }
        self.next += 1;
        self.steps.push(Step::Alloc(self.next));
        Some(self.next)
    }
    fn zero(&mut self, f: u32) {
        self.steps.push(Step::Zero(f));
    }
    fn map(&mut self, i: usize, f: u32) -> bool {
        self.steps.push(Step::Map(i, f));
        self.fail_map != Some(i)
    }
    fn free(&mut self, f: u32) {
        self.steps.push(Step::Free(f));
    }
    fn rollback(&mut self, mapped: usize) {
        self.steps.push(Step::Rollback(mapped));
    }
}

fn run(pages: usize, rec: &mut Rec) -> bool {
    map_loop(rec, pages)
}

#[test]
fn every_frame_is_zeroed_before_it_is_mapped_and_mapped_once() {
    let mut r = Rec::default();
    assert!(run(100, &mut r));
    for (i, s) in r.steps.iter().enumerate() {
        if let Step::Map(page, f) = *s {
            let zeroed = r.steps[..i].iter().rposition(|x| *x == Step::Zero(f));
            let allocated = r.steps[..i].iter().rposition(|x| *x == Step::Alloc(f));
            assert!(allocated < zeroed && zeroed.is_some(), "page {page}: frame {f} mapped before it was zeroed");
        }
    }
    let maps: Vec<usize> = r.steps.iter().filter_map(|s| if let Step::Map(p, _) = s { Some(*p) } else { None }).collect();
    assert_eq!(maps, (0..100).collect::<Vec<_>>());
}

#[test]
fn shootdowns_are_served_before_every_page() {
    let mut r = Rec::default();
    run(40, &mut r);
    for (i, s) in r.steps.iter().enumerate() {
        if let Step::Alloc(_) = s {
            let before = r.steps[..i].iter().rev().find(|x| !matches!(x, Step::Window));
            assert_eq!(before.copied().map(|b| b == Step::Serve || matches!(b, Step::Map(..))), Some(true));
            assert_eq!(r.steps[i - 1], Step::Serve, "step {i}");
        }
    }
}

#[test]
fn interrupts_are_let_in_every_window_pages_and_never_further_apart() {
    for pages in [0, 1, 15, 16, 17, 32, 33, 1000, 262_144] {
        let mut r = Rec::default();
        assert!(run(pages, &mut r));
        let windows = r.steps.iter().filter(|s| **s == Step::Window).count();
        assert_eq!(windows, pages.saturating_sub(1) / WINDOW_PAGES, "{pages} pages");
        // Between two windows, or a window and either end, at most
        // WINDOW_PAGES pages are mapped.
        let mut since = 0;
        for s in &r.steps {
            match s {
                Step::Window => {
                    assert_eq!(since, WINDOW_PAGES, "{pages} pages: a window early");
                    since = 0;
                }
                Step::Map(..) => since += 1,
                _ => {}
            }
            assert!(since <= WINDOW_PAGES, "{pages} pages: {since} mapped with no window");
        }
        assert!(r.steps.first() != Some(&Step::Window), "no window before any work");
    }
    assert_eq!(WINDOW_PAGES, 16, "64 KiB, the masked loop's serve unit");
}

#[test]
fn a_frame_that_cannot_be_had_undoes_every_page_before_it() {
    for at in [0, 1, 16, 17, 99] {
        let mut r = Rec { fail_alloc: Some(at), ..Rec::default() };
        assert!(!run(100, &mut r));
        assert_eq!(r.steps.last(), Some(&Step::Rollback(at)), "fail at {at}");
        let maps = r.steps.iter().filter(|s| matches!(s, Step::Map(..))).count();
        assert_eq!(maps, at);
    }
}

#[test]
fn a_page_that_cannot_be_mapped_frees_its_frame_then_undoes_the_rest() {
    for at in [0, 5, 16, 63] {
        let mut r = Rec { fail_map: Some(at), ..Rec::default() };
        assert!(!run(64, &mut r));
        let n = r.steps.len();
        assert_eq!(r.steps[n - 3], Step::Map(at, at as u32 + 1));
        assert_eq!((r.steps[n - 2], r.steps[n - 1]), (Step::Free(at as u32 + 1), Step::Rollback(at)));
    }
}

/// map_fresh.rs and map_kernel.rs without whitespace: the kernel's half,
/// read as text.
fn kernel_side() -> String {
    ["map_fresh.rs", "map_kernel.rs"]
        .iter()
        .map(|f| {
            let path = format!("{}/../../src/syscall/microkernel/memory/va/{f}", env!("CARGO_MANIFEST_DIR"));
            std::fs::read_to_string(path).unwrap()
        })
        .collect::<String>()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect()
}

#[test]
fn the_kernels_window_opens_only_a_masked_cpu_and_masks_it_again() {
    let k = kernel_side();
    let at = k.find("fnwindow(&mutself){").expect("the kernel's window");
    let body = &k[at..at + k[at..].find("fnallocate").unwrap()];
    assert!(
        body.contains("ifcrate::arch::cpu::interrupts_enabled(){return;}"),
        "a CPU already taking interrupts is left as it is"
    );
    let open = body.find("crate::arch::enable_interrupts();").expect("opened");
    let shadow = body.find("core::hint::spin_loop();").expect("one instruction in STI's shadow");
    let close = body.find("crate::arch::disable_interrupts();").expect("masked again");
    assert!(open < shadow && shadow < close);
}

#[test]
fn mkmmap_backs_its_range_through_the_loop_with_the_kernels_own_steps() {
    let k = kernel_side();
    assert!(k.contains("map_loop(&mutKernel{base,perms},pagesasusize)"), "map_fresh goes through map_loop");
    assert!(k.contains("crate::memory::frame_alloc::zero_frame(frame);"), "zero is the allocator's zeroing");
    assert!(k.contains("map_page(va,frame,self.perms).is_ok()"));
    assert!(k.contains("crate::smp::serve_shootdowns();"));
    assert!(k.contains("rollback_mapped_pages(self.base,mapped);"));
    assert!(k.contains("letlen=pages*PAGE_SIZEasu64;let_busy=crate::kernel_core::surface_registry::pin::busy::begin(base,len);"));
}
