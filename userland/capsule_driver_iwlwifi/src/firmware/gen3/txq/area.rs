// NONOS Operating System (AGPL-3.0-or-later)

//! A queue's area in the family's layout. A V2 (BZ) byte count table holds
//! `TFD_QUEUE_BC_SIZE_GEN3_BZ` entries, so the first-buffer slots and frame
//! buffers after it start one page later; a V1 area is as before.

use super::super::csr::CsrLayout;
use super::layout::{BC_TABLE, BC_TABLE_BZ, BUFS, FIRST, TXQ_STRIDE, TX_REGION};
use super::queue::TxQueue;

const fn pages(n: usize) -> usize {
    (n + 4095) & !4095
}

/// The byte count table's bytes beyond V1's, page rounded.
pub const fn bc_extra(csr: CsrLayout) -> usize {
    match csr {
        CsrLayout::V1 => 0,
        CsrLayout::V2 => pages(BC_TABLE_BZ) - pages(BC_TABLE),
    }
}

/// One queue's area in a layout.
pub const fn txq_stride(csr: CsrLayout) -> usize {
    TXQ_STRIDE + bc_extra(csr)
}

/// The transmit region: the management queue's area, then the data queue's.
pub const fn tx_region(csr: CsrLayout) -> usize {
    TX_REGION + 2 * bc_extra(csr)
}

impl TxQueue {
    /// Queue `n` (0 or 1) of a transmit region in the family's layout.
    pub const fn nth(n: usize, csr: CsrLayout) -> Self {
        Self { extra: bc_extra(csr), ..Self::new(n * txq_stride(csr)) }
    }

    // The first-buffer slots and the frame buffers, after the table.
    pub(super) const fn first(&self) -> usize {
        self.base + FIRST + self.extra
    }

    pub(super) const fn bufs(&self) -> usize {
        self.base + BUFS + self.extra
    }
}
