// NONOS Operating System (AGPL-3.0-or-later)

//! DMA memory: whole pages below 4 GiB or nothing (the WFDMA's addresses
//! are 32 bits), every access inside the grant or not made, and a slice
//! of a grant the same memory under the same device addresses.

use nonos_libc::{set_dma_mode, DmaMode};

use crate::dma::Grant;

#[test]
fn a_grant_is_whole_pages_and_below_four_gibibytes() {
    let g = Grant::map(1, 1, 10).unwrap();
    assert_eq!(g.size(), 4096);
    assert!(u64::from(g.dev(0)) + 4096 <= 1 << 32);
    set_dma_mode(DmaMode::High);
    let high = Grant::map(1, 1, 4096);
    set_dma_mode(DmaMode::Refuse);
    let refused = Grant::map(1, 1, 4096);
    set_dma_mode(DmaMode::Grant);
    assert!(high.is_none(), "a grant past 4 GiB is refused, not truncated");
    assert!(refused.is_none());
}

#[test]
fn nothing_outside_a_grant_is_read_or_written() {
    let g = Grant::map(1, 1, 4096).unwrap();
    g.write(4090, &[7u8; 6]);
    g.write(4091, &[9u8; 6]);
    g.wr32(4096, 5);
    g.wr32(2, 5);
    let mut b = [1u8; 6];
    g.read(4090, &mut b);
    assert_eq!(b, [7u8; 6], "the write past the end changed nothing");
    g.read(4091, &mut b);
    assert_eq!(b, [0u8; 6], "a read past the end reads zeros");
    assert_eq!((g.rd32(4096), g.rd32(2), g.rd32(0)), (0, 0, 0), "out of range, unaligned");
}

#[test]
fn a_slice_is_the_same_memory() {
    let g = Grant::map(1, 1, 8192).unwrap();
    let s = g.sub(4096, 64).unwrap();
    assert_eq!(s.dev(0), g.dev(4096));
    s.wr32(4, 0xa5a5_5a5a);
    assert_eq!(g.rd32(4100), 0xa5a5_5a5a);
    assert!(g.sub(8190, 4).is_none() && g.sub(usize::MAX, 2).is_none());
    s.fill(60, 8, 0xff);
    assert_eq!(g.rd32(4096 + 60), 0, "a fill past the slice's end writes nothing");
}
