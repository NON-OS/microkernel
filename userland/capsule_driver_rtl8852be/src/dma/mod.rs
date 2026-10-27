// NONOS Operating System (AGPL-3.0-or-later)

//! DMA memory: the seam the rings are written against, and the broker grant
//! that backs it on the card.

mod grant;
mod mem;

pub use grant::{map_dma32, Grant};
pub use mem::DmaMem;
