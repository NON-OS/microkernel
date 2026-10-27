// NONOS Operating System (AGPL-3.0-or-later)

//! DMA grants as host memory. Each map is page-aligned zeroed memory that
//! lives to the end of the process, handed out under a device address
//! below 4 GiB; the modelled chip turns a device address back into host
//! memory with `dma_at`, as an IOMMU-less bus would reach it.

use std::alloc::{alloc_zeroed, Layout};
use std::cell::{Cell, RefCell};

use crate::types::dma::DmaMapOut;

pub const MK_DMA_MAP_DMA32: u32 = 1 << 1;
pub const MK_DMA_MAP_COHERENT: u32 = 1 << 2;

/// How the broker answers the next maps.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DmaMode {
    Grant,
    Refuse,
    /// Grants placed past 4 GiB, as a broker without DMA32 would.
    High,
}

thread_local! {
    static REGIONS: RefCell<Vec<(u64, usize, usize)>> = const { RefCell::new(Vec::new()) };
    static NEXT: Cell<u64> = const { Cell::new(0x1000_0000) };
    static MODE: Cell<DmaMode> = const { Cell::new(DmaMode::Grant) };
}

pub fn set_dma_mode(m: DmaMode) {
    MODE.with(|c| c.set(m));
}

pub fn mk_dma_map(_d: u64, _e: u64, len: u64, _flags: u32, out: &mut DmaMapOut) -> i64 {
    let mode = MODE.with(Cell::get);
    if mode == DmaMode::Refuse || len == 0 {
        return -12;
    }
    let size = len as usize;
    // SAFETY: a non-zero size and a power-of-two alignment.
    let va = unsafe { alloc_zeroed(Layout::from_size_align(size, 4096).unwrap()) } as usize;
    let base = if mode == DmaMode::High { 0x1_0000_0000 } else { 0 };
    let dev = NEXT.with(|n| {
        let d = n.get();
        n.set(d + len.next_multiple_of(4096) + 4096);
        d + base
    });
    REGIONS.with(|r| r.borrow_mut().push((dev, va, size)));
    *out = DmaMapOut { user_va: va as u64, device_addr: dev, length: len, grant_id: dev };
    0
}

/// The host address of device bytes `dev..dev + len`, if one grant holds
/// them all; `None` is a DMA to memory the driver never mapped.
pub fn dma_at(dev: u32, len: usize) -> Option<*mut u8> {
    REGIONS.with(|r| {
        r.borrow().iter().find_map(|&(base, va, size)| {
            let off = u64::from(dev).checked_sub(base)? as usize;
            (off.checked_add(len)? <= size).then_some((va + off) as *mut u8)
        })
    })
}

/// Whether any grant still holds `needle`: a key left behind in DMA memory.
pub fn dma_holds(needle: &[u8]) -> bool {
    REGIONS.with(|r| {
        r.borrow().iter().any(|&(_, va, size)| {
            // SAFETY: a live grant of `size` bytes at `va`.
            let s = unsafe { std::slice::from_raw_parts(va as *const u8, size) };
            s.windows(needle.len()).any(|w| w == needle)
        })
    })
}
