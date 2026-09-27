// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/memory/dma/types/direction.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/memory/dma/types/direction.rs"]
pub mod direction;

pub fn dmadirection_writes_to_device(this: direction::DmaDirection) -> bool {
    this.writes_to_device()
}

pub fn dmadirection_reads_from_device(this: direction::DmaDirection) -> bool {
    this.reads_from_device()
}

