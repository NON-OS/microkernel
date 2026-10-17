// NONOS Operating System (AGPL-3.0-or-later)
//! Reserved DMA ranges from a damaged tree: a short entry, a phandle naming
//! no node, and more entries than kept each refuse the read whole.

use crate::arch::fdt::find::reserved_dma::Refusal;
use crate::arch::fdt::FdtError;
use crate::reserved_dma_tree::{read, tree, PLATFORM};

#[test]
fn damage_and_overflow_refuse_the_read() {
    let short = [(Some((0x9000_0000, 0x1000)), vec![PLATFORM, 0, 0x9000_0000, 0])];
    assert_eq!(read(&tree(&short, None)), Err(Refusal::Tree(FdtError::OutOfBounds)));
    let nobody = [(Some((0x9000_0000, 0x1000)), vec![77, 0, 0x9000_0000, 0, 0x1000])];
    assert_eq!(read(&tree(&nobody, None)), Err(Refusal::Tree(FdtError::NoSuchNode)));
    let one = [PLATFORM, 0, 0x9000_0000, 0, 0x1000];
    let many = [(Some((0x9000_0000, 0x1000)), one.repeat(17))];
    assert_eq!(read(&tree(&many, None)), Err(Refusal::TooMany));
    assert_eq!(
        read(&tree(&[(Some((0x9000_0000, 0x1000)), one.repeat(16))], None)).unwrap().0.len(),
        16
    );
}
