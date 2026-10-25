// NONOS Operating System (AGPL-3.0-or-later)
//! The register window and a DMA grant keep every access inside what was
//! mapped: a register past the window, or not aligned to its size, reads all
//! ones and takes no write; a copy that would run past a grant writes nothing.

use crate::fw::dma::{DmaMem, Grant};
use crate::regs::{Mmio, Regs};

#[test]
fn the_register_window_reads_all_ones_past_its_end_and_takes_no_write_there() {
    let mut words = [0u32; 5];
    let base = words.as_mut_ptr() as u64;
    // SAFETY: 16 bytes of `words` are live for the whole test; the fifth word
    // stands past the window to show a write there does not land.
    let r = unsafe { Regs::new(base, 16) };
    r.write32(12, 0x1122_3344);
    assert_eq!(r.read32(12), 0x1122_3344);
    assert_eq!((r.read8(12), r.read16(14)), (0x44, 0x1122), "inside, aligned");
    r.write32(16, 0xDEAD_BEEF);
    r.write16(16, 0xBEEF);
    r.write8(16, 0xEF);
    assert_eq!(words[4], 0, "a write past the window goes nowhere");
    let r = unsafe { Regs::new(base, 16) };
    assert_eq!((r.read32(16), r.read16(16), r.read8(16)), (u32::MAX, u16::MAX, u8::MAX));
    assert_eq!(r.read32(14), u32::MAX, "a word crossing the end");
    assert_eq!((r.read32(2), r.read16(13)), (u32::MAX, u16::MAX), "not aligned");
    assert_eq!(r.read32(usize::MAX - 1), u32::MAX, "an offset that overflows");
    r.write32(2, 1);
    assert_eq!(r.read32(0), 0, "an unaligned write goes nowhere");
}

#[test]
fn a_grant_writes_nothing_that_would_run_past_it() {
    let mut mem = vec![0u8; 33];
    // SAFETY: 32 bytes of `mem` are live for the whole test; the last byte
    // stands past the grant.
    let g = unsafe { Grant::new(mem.as_mut_ptr() as u64, 0x8000, 32) };
    g.write_bytes(29, &[1, 2, 3]);
    g.write_bytes(30, &[4, 5, 6]);
    g.write_bytes(usize::MAX, &[7]);
    assert_eq!((g.capacity(), g.as_slice().len()), (32, 32));
    assert_eq!(&mem[28..], &[0, 1, 2, 3, 0], "the fitting copy only, nothing past");
}
