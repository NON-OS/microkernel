// Temporary instrumentation: names the exact allocation(s) still live after a
// PNG decode. Drop this file into #601's image_paint_proofs/tests/, run in CI
// (linux, not the broken mac SDK), and read the backtrace(s) it prints.
//   cargo test -p image_paint_proofs --test png_leak_trace -- --nocapture
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::collections::BTreeMap;
use std::sync::Mutex;

use nonos_toolkit::image::png::decoder::decode_png_argb8888;

thread_local!(static BUSY: Cell<bool> = const { Cell::new(false) });
static LIVE: Mutex<BTreeMap<usize, (usize, String)>> = Mutex::new(BTreeMap::new());

struct Tracking;
unsafe impl GlobalAlloc for Tracking {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        let p = unsafe { System.alloc(l) };
        // Record from outside the allocator's own bookkeeping (the map/String/
        // backtrace allocate; BUSY makes those passes go untracked).
        let _ = BUSY.try_with(|b| {
            if !b.replace(true) {
                let bt = std::backtrace::Backtrace::force_capture().to_string();
                if let Ok(mut m) = LIVE.lock() {
                    m.insert(p as usize, (l.size(), bt));
                }
                b.set(false);
            }
        });
        p
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        let _ = BUSY.try_with(|b| {
            if !b.replace(true) {
                if let Ok(mut m) = LIVE.lock() {
                    m.remove(&(p as usize));
                }
                b.set(false);
            }
        });
        unsafe { System.dealloc(p, l) }
    }
}

#[global_allocator]
static GLOBAL: Tracking = Tracking;

#[test]
fn name_the_surviving_allocation() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/misc/png_rgba_2000x2000_diagram.png");
    let file = std::fs::read(path).expect("fixture");
    let mut out = vec![0u32; 2000 * 2000];
    // Clear anything from startup so we only see the decode's survivors.
    LIVE.lock().unwrap().clear();
    let size = decode_png_argb8888(&file, &mut out).expect("decodes");
    assert_eq!((size.width, size.height), (2000, 2000));
    drop(out);

    let m = LIVE.lock().unwrap();
    let survivors: Vec<_> = m.values().collect();
    let total: usize = survivors.iter().map(|(s, _)| *s).sum();
    eprintln!("\n==== SURVIVING ALLOCATIONS: {} live, {} bytes ====", survivors.len(), total);
    for (i, (sz, bt)) in survivors.iter().enumerate() {
        eprintln!("\n-- survivor #{i}: {sz} bytes --\n{bt}");
    }
    eprintln!("==== end ====\n");
    // On a clean tree this is 0; on #601 it names the 144-byte call.
    assert_eq!(total, 0, "{total} bytes survived the decode (see backtraces above)");
}
