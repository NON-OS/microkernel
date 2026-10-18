// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the aarch64 boot path's DMA pool placement.

// Test builds only: the kernel keeps `place` private to its boot path, so a
// library build would have nothing that calls it.
#[cfg(test)]
#[path = "../../../src/arch/aarch64/boot/memory/dma_place.rs"]
mod dma_place;

#[cfg(test)]
mod brute_tests;
#[cfg(test)]
mod layout_tests;
